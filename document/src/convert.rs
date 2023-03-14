use crate::DidDocument;
use cid::multibase;
use once_cell::sync::Lazy;
use ssi::did::{Resource, VerificationMethod};
use ssi::did_resolve::{dereference, Content, DIDResolver, DereferencingInputMetadata};
use ssi::jwk::{Base64urlUInt, OctetParams, Params, JWK};

static DID_TYPE_REGEX: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r#"did:(?P<T>[^:]+):(?P<K>[A-Za-z0-9:]+)"#).unwrap());

async fn did_as_jwk(resolver: &dyn DIDResolver, id: &str) -> anyhow::Result<Option<JWK>> {
    let (res, object, _) = dereference(resolver, id, &DereferencingInputMetadata::default()).await;
    if res.error.is_none() {
        println!("{:?}", object);
        match object {
            Content::Object(Resource::VerificationMethod(vm)) => {
                let jwk = vm.get_jwk()?;
                return Ok(Some(jwk));
            }
            Content::DIDDocument(doc) => {
                if let Some(vms) = &doc.verification_method {
                    for vm in vms {
                        if let VerificationMethod::Map(vm) = vm {
                            if let Ok(jwk) = vm.get_jwk() {
                                return Ok(Some(jwk));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Ok(None)
}

fn pk_as_jwk(public_key: &str) -> anyhow::Result<JWK> {
    let (_, data) = multibase::decode(&public_key)?;
    let curve = match data[0..2] {
        [0xed, 0x01] => "Ed25519",
        [0xe7, 0x01] => "SECP256K1",
        _ => anyhow::bail!("Unknown encoding prefix"),
    };
    Ok(JWK::from(Params::OKP(OctetParams {
        curve: curve.to_string(),
        public_key: Base64urlUInt(data[2..].to_vec()),
        private_key: None,
    })))
}

pub async fn convert(did: &DidDocument) -> anyhow::Result<JWK> {
    let cap = DID_TYPE_REGEX.captures(&did.id);
    if let Some(cap) = cap {
        match &cap["T"] {
            "key" => {
                if let Some(jwk) = did_as_jwk(&did_method_key::DIDKey, &did.id).await? {
                    Ok(jwk)
                } else {
                    pk_as_jwk(&cap["K"])
                }
            }
            "pkh" => {
                if let Some(jwk) = did_as_jwk(&did_pkh::DIDPKH, &did.id).await? {
                    Ok(jwk)
                } else {
                    anyhow::bail!("Failed to get jwk for {}", did.id);
                }
            }
            _ => {
                unimplemented!()
            }
        }
    } else {
        anyhow::bail!("Invalid DID")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ssi::did::DIDMethod;
    use ssi::did::VerificationMethod;

    #[tokio::test]
    async fn should_convert_did_key_generated_without_vm() {
        let jwk = JWK::generate_ed25519().unwrap();
        let did = did_method_key::DIDKey
            .generate(&ssi::did::Source::Key(&jwk))
            .unwrap();
        let did = DidDocument::new(&did);
        let other_jwk = convert(&did).await.unwrap();
        if let (Params::OKP(did), Params::OKP(orig)) = (other_jwk.params, jwk.params) {
            assert_eq!(did.public_key, orig.public_key)
        }
    }

    #[tokio::test]
    async fn should_convert_did_key_generated_with_vm() {
        let jwk = ssi::jwk::JWK::generate_ed25519().unwrap();
        let did = did_method_key::DIDKey
            .generate(&ssi::did::Source::Key(&jwk))
            .unwrap();
        let pko = ssi::did::VerificationMethodMap {
            public_key_jwk: Some(jwk.clone()),
            ..Default::default()
        };
        let did = ssi::did::DocumentBuilder::default()
            .id(did)
            .verification_method(vec![VerificationMethod::Map(pko)])
            .build()
            .unwrap();
        let other_jwk = convert(&did).await.unwrap();
        if let (Params::OKP(did), Params::OKP(orig)) = (other_jwk.params, jwk.params) {
            assert_eq!(did.public_key, orig.public_key)
        }
    }

    #[tokio::test]
    async fn should_convert_did_key_with_vm() {
        let did = DidDocument::new("did:key:zQ3shokFTS3brHcDQrn82RUDfCZESWL1ZdCEJwekUDPQiYBme#zQ3shokFTS3brHcDQrn82RUDfCZESWL1ZdCEJwekUDPQiYBme");
        let jwk = convert(&did).await.unwrap();
        println!("JWK={:?}", jwk);
        if let Params::OKP(op) = jwk.params {
            println!("OP={}", String::from_utf8_lossy(op.public_key.0.as_ref()))
        }
    }

    #[tokio::test]
    async fn should_fail_to_convert_did_pkh_with_vm() {
        let did = DidDocument::new("did:pkh:eip155:1:0xb9c5714089478a327f09197987f16f9e5d936e8a");
        if let Ok(_) = convert(&did).await {
            panic!("Cannot get JWK from document");
        }
    }
}
