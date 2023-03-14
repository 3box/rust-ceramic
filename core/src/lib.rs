use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct Base64String(String);

impl Base64String {
    pub fn try_from_cid(cid: &cid::Cid) -> anyhow::Result<Self> {
        let s = cid.to_string_of_base(multibase::Base::Base64)?;
        Ok(Self(s))
    }
}

impl AsRef<str> for Base64String {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<&Vec<u8>> for Base64String {
    fn from(value: &Vec<u8>) -> Self {
        Self(multibase::encode(multibase::Base::Base64, value))
    }
}

#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct Base64UrlString(String);

impl Base64UrlString {
    pub fn try_from_cid(cid: &cid::Cid) -> anyhow::Result<Self> {
        let s = cid.to_string_of_base(multibase::Base::Base64Url)?;
        Ok(Self(s))
    }
}

impl AsRef<str> for Base64UrlString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<&Vec<u8>> for Base64UrlString {
    fn from(value: &Vec<u8>) -> Self {
        Self(multibase::encode(multibase::Base::Base64Url, value))
    }
}

#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct DagCborEncoded(Vec<u8>);

impl AsRef<[u8]> for DagCborEncoded {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl DagCborEncoded {
    pub fn new<T: Serialize>(value: &T) -> anyhow::Result<Self> {
        let res = serde_ipld_dagcbor::to_vec(value)?;
        Ok(Self(res))
    }
}
