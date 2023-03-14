use crate::DidDocument;
use ssi::jwk::Algorithm;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Signed(String);

impl Signed {
    pub async fn new<T: Serialize>(claims: &T, signer: &DidDocument) -> anyhow::Result<Self> {
        let jwk = crate::convert::convert(signer).await?;
        let s = ssi::jwt::encode_sign(Algorithm::EdDSA, &claims, &jwk)?;
        return Ok(Self(s));
    }
}