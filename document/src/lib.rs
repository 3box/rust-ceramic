extern crate core;

mod commit;
mod convert;
mod deterministic_commit;
mod signed;

pub use ssi;
pub use ssi::did::Document as DidDocument;

pub use ceramic_core::*;
pub use ceramic_stream_id::{Cid, StreamId};
pub use cid;
pub use signed::Signed;

use anyhow::Result;
use serde::Serialize;

const SEP: &'static str = "model";

pub struct CommitArgs<'a> {
    pub signer: &'a DidDocument,
    pub controllers: Vec<&'a DidDocument>,
    pub model: &'a StreamId,
}

impl<'a> CommitArgs<'a> {
    pub async fn deterministic_commit<T: Serialize>(
        &self,
        data: &T,
    ) -> Result<deterministic_commit::DeterministicCommit> {
        let cmt = deterministic_commit::DeterministicCommit::new(self, data)?;
        Ok(cmt)
    }

    pub async fn commit<T: Serialize>(&self, data: &T) -> Result<commit::Commit> {
        let cmt = commit::Commit::new(self, data).await?;
        Ok(cmt)
    }
}

#[derive(Serialize)]
struct GenesisHeader<'a> {
    controllers: &'a Vec<&'a crate::DidDocument>,
    model: &'a Vec<u8>,
    sep: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: &'a Option<Vec<u32>>,
}

#[derive(Serialize)]
struct GenesisCommit<'a, T: Serialize> {
    data: &'a T,
    header: GenesisHeader<'a>,
}
