use std::{future::IntoFuture, net::SocketAddr};
use axum::{http::request, routing::post, Json, Router};
use serde_json::Value;
use tracing::info;
use tokio::net::TcpListener;
use axum::extract::State;

use reec_storage::Store;
use engine::{ExchangeCapabilitiesRequest, NewPayloadV3Request};
use eth::{
    block::{self, GetBlockByHashRequest, GetBlockByNumberRequest, GetBlockTransactionCountByNumberRequest}, 
    client
};
use utils::{RpcErr, RpcErrorMetadata, RpcErrorResponse, RpcRequest, RpcSuccessResponse};

mod engine;
mod utils;
mod eth;
mod admin;


pub async fn start_api(http_addr: SocketAddr, authrpc_addr: SocketAddr, storage: Store) {
    let http_router = Router::new().route("/", post(handle_http_request)).with_state(storage.clone());
    let http_listener = TcpListener::bind(http_addr).await.unwrap();
    let http_server = axum::serve(http_listener, http_router).with_graceful_shutdown(shutdown_signal()).into_future();
    info!("HTTP Server listening on {}", http_addr);

    let authrpc_router = Router::new().route("/", post(handle_authrpc_request)).with_state(storage);
    let authrpc_listener = TcpListener::bind(authrpc_addr).await.unwrap();
    let authrpc_server = axum::serve(authrpc_listener, authrpc_router).with_graceful_shutdown(shutdown_signal()).into_future();
    info!("AUTH-RPC Server listening on {}", authrpc_addr);

    let _ = tokio::try_join!(http_server, authrpc_server).inspect_err(|e| info!("Error shutting down servers: {:?}", e));
}

async fn shutdown_signal(){
    tokio::signal::ctrl_c().await.expect("Failed to listen to the shutdown signal");
}

pub async fn handle_authrpc_request(State(storage): State<Store>, body: String) -> Json<Value>{
    let req: RpcRequest = serde_json::from_str(&body).unwrap();
    let res = match map_requests(&req, storage.clone()) {
        res @ Ok(_) => res,
        _ => map_internal_requests(&req, storage),
    };
    rpc_response(req.id, res)
}

pub async fn handle_http_request(State(storage): State<Store>, body: String) -> Json<Value> {
    let req: RpcRequest = serde_json::from_str(&body).unwrap();

    let res = map_requests(&req, storage);
    rpc_response(req.id, res)
}

/// Handle requests that can come from either clients or other users
pub fn map_requests(req: &RpcRequest, storage: Store) -> Result<Value, RpcErr> {
    match req.method.as_str() {
        "engine_exchangeCapabilities" => {
            let capabilities: ExchangeCapabilitiesRequest = req
                .params.as_ref()
                .ok_or(RpcErr::BadParams)?
                .first()
                .ok_or(RpcErr::BadParams).
                and_then(|v| serde_json::from_value(v.clone()).map_err(|_| RpcErr::BadParams))?;
            engine::exchange_capabilities(&capabilities)
        }
        "eth_chainId" => client::chain_id(),
        "eth_syncing" => client::syncing(),
        "eth_getBlockByNumber" => {
            let request = GetBlockByNumberRequest::parse(&req.params).ok_or(RpcErr::BadParams)?;
            block::get_block_by_number(&request, storage)
        }
        "eth_getBlockByHash" => {
            let request = GetBlockByHashRequest::parse(&req.params).ok_or(RpcErr::BadParams)?;
            block::get_block_by_hash(&request, storage)
        }
        "eth_getBlockTransactionCountByNumber" => {
            let request = GetBlockTransactionCountByNumberRequest::parse(&req.params).ok_or(RpcErr::BadParams)?;
        }
        "engine_forkchoiceUpdatedV3" => engine::forkchoice_Updated_V3(),
        "engine_newPayloadV3" => {
            let request = parse_new_payload_v3_request(req.params.as_ref().ok_or(RpcErr::BadParams)?)?;
            Ok(serde_json::to_value(engine::new_payload_v3(request)?).unwrap())
        }
        "admin_nodeInfo" => admin::node_info(),
        _ => Err(RpcErr::MethodNotFound)
    }
}

/// Handle requests from other clients
pub fn map_internal_requests(_req: &RpcRequest, _storage: Store) -> Result<Value, RpcErr> {
    Err(RpcErr::MethodNotFound)
}

fn rpc_response<E>(id: i32, res: Result<Value, E>) -> Json<Value>
where E: Into<RpcErrorMetadata>
{
    match res {
        Ok(result) => Json(
            serde_json::to_value(RpcSuccessResponse {
                id,
                jsonrpc: "2.0".to_string(),
                result,
            }).unwrap(),
        ),
        Err(error) => Json(
            serde_json::to_value(RpcErrorResponse {
                id,
                jsonrpc: "2.0".to_string(),
                error: error.into(),
            }).unwrap(),
        )
    }
}

fn parse_new_payload_v3_request(params: &[Value]) -> Result<NewPayloadV3Request, RpcErr> {
    if params.len() != 3 {
        return Err(RpcErr::BadParams);
    }
    let payload = serde_json::from_value(params[0].clone()).map_err(|_| RpcErr::BadParams)?;
    let expected_blob_versioned_hashes = serde_json::from_value(params[1].clone()).map_err(|_| RpcErr::BadParams)?;
    let parent_beacon_block_root = serde_json::from_value(params[2].clone()).map_err(|_| RpcErr::BadParams)?;
    Ok(NewPayloadV3Request { 
        payload, 
        expected_blob_versioned_hashes, 
        parent_beacon_block_root, 
    })
}


