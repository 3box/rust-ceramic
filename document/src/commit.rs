use crate::{CommitArgs, Signed};
use anyhow::Result;
use ceramic_core::DagCborEncoded;
use cid::{multibase, Cid};
use multihash::{Code, MultihashDigest};
use rand::Fill;
use serde::Serialize;

pub struct Commit {
    pub cid: Cid,
    pub signed: Signed,
}

impl Commit {
    pub async fn new<'a, T: Serialize>(args: &'a CommitArgs<'a>, data: &T) -> Result<Self> {
        let model = args.model.cid.to_bytes();
        let mut rng = rand::thread_rng();
        let mut header = [0u32; 12];
        header.try_fill(&mut rng)?;
        let header = Some(header.to_vec());
        let genesis_commit = crate::GenesisCommit {
            data: data,
            header: crate::GenesisHeader {
                controllers: &args.controllers,
                model: &model,
                sep: crate::SEP,
                header: &header,
            },
        };
        // encode our commit with dag cbor, hashing that to create cid
        let linked_block = DagCborEncoded::new(&genesis_commit)?;
        let cid = cid::Cid::new_v1(0x12, Code::Sha3_256.digest(linked_block.as_ref()));
        let encoded_cid = multibase::encode(multibase::Base::Base64Url, cid.to_bytes());
        let encoded_data = multibase::encode(multibase::Base::Base64, linked_block);
        // create jws with encoded_cid and { linkedBlock: encoded_data }
        // did_createjws with { did: id, linkedBlock: encoded_data, payload: encoded_cid }
        let claims = Claims {
            did: &args.signer,
            //protected: None,
            linked_block: encoded_data,
            payload: encoded_cid,
        };
        let signed = Signed::new(&claims, args.signer).await?;
        Ok(Self {
            cid: cid,
            signed: signed,
        })
    }
}

#[derive(Serialize)]
struct Claims<'a> {
    did: &'a crate::DidDocument,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //protected: Option<&'a Caco>,
    linked_block: String,
    payload: String,
}
