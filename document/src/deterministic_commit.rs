use crate::CommitArgs;
use anyhow::Result;
use ceramic_core::DagCborEncoded;
use serde::Serialize;

pub struct DeterministicCommit {
    pub encoded: DagCborEncoded,
}

impl DeterministicCommit {
    pub fn new<T: Serialize>(args: &CommitArgs, data: &T) -> Result<Self> {
        let model = args.model.cid.to_bytes();
        let header = None;
        let genesis_commit = crate::GenesisCommit {
            data,
            header: crate::GenesisHeader {
                controllers: &args.controllers,
                model: &model,
                sep: crate::SEP,
                header: &header,
            },
        };

        let data = DagCborEncoded::new(&genesis_commit)?;
        Ok(Self { encoded: data })
    }
}
