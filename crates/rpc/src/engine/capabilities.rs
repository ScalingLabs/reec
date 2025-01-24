use serde_json::{Value, json};

use crate::RpcErr;

pub fn exchange_capabilities() -> Result<Value, RpcErr> {
    Ok(json!([]))
}