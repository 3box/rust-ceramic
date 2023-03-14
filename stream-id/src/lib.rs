pub use cid::Cid;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct StreamId {
    pub r#type: u32,
    pub cid: Cid,
}

impl StreamId {
    pub fn document(id: Cid) -> Self {
        Self { r#type: 3, cid: id }
    }
}
