use serde::{Serialize, Deserialize};
use serde_json::Value;

pub enum RpcErr {
    MethodNotFound,
    BadParams,
    UnsuportedFork,
    Internal,
}

impl From<RpcErr> for RpcErrorMetadata {
    fn from(value: RpcErr) -> Self {
        match value {
            RpcErr::MethodNotFound => RpcErrorMetadata {
                code: -32601,
                message: "Method Not Found".to_string(),
            },
            RpcErr::BadParams => RpcErrorMetadata {
                code: -32602,
                message: "Invalid params".to_string(),
            },
            RpcErr::UnsuportedFork => RpcErrorMetadata { 
                code: -38005, 
                message: "Unsupported fork".to_string(),
            },
            RpcErr::Internal => RpcErrorMetadata { 
                code: -32603, 
                message: "Internal Error".to_string(), 
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcErrorMetadata {
    code: i32,
    message: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcRequest {
    pub id: i32,
    pub method: String,
    pub jsonrpc: String,
    pub params: Option<Vec<Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcSuccessResponse {
    pub id: i32,
    pub jsonrpc: String,
    pub result: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcErrorResponse {
    pub id: i32,
    pub jsonrpc: String,
    pub error: RpcErrorMetadata,
}