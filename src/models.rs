use anyhow::Result;
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateLinkModel {
    pub url: String,
}

impl CreateLinkModel {
    pub(crate) fn from_bytes(b: &Bytes) -> Result<Self> {
        let r: CreateLinkModel = serde_json::from_slice(b)?;
        Ok(r)
    }
}
#[derive(Debug, Serialize)]
pub struct LinkCreatedModel {
    pub short: String,
    pub url: String,
}
