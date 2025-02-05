use std::future::IntoFuture;
use axum::{routing::post, Json, Router};
use serde_json::Value;
use tracing::info;
use tokio::net::TcpListener;
use engine::capabilities::exchange_capabilities;
use eth::{
    block::get_block_by_number, 
    client::{chain_id, syncing}
};
use utils::{RpcErr, RpcErrorMetadata, RpcErrorResponse, RpcRequest, RpcSuccessResponse};

mod engine;
mod utils;
mod eth;

#[tokio::main]
pub async fn start_api(http_addr: &str, http_port: &str, authrpc_addr: &str, authrpc_port: &str) {
    let http_router = Router::new().route("/", post(handle_http_request));
    let http_url = create_url(http_addr, http_port);
    let http_listener = TcpListener::bind(&http_url).await.unwrap();
    let http_server = axum::serve(http_listener, http_router).with_graceful_shutdown(shutdown_signal()).into_future();
    info!("HTTP Server listening on {}", http_url);

    let authrpc_router = Router::new().route("/", post(handle_authrpc_request));
    let authrpc_url = create_url(authrpc_addr, authrpc_port);
    let authrpc_listener = TcpListener::bind(&authrpc_url).await.unwrap();
    let authrpc_server = axum::serve(authrpc_listener, authrpc_router).with_graceful_shutdown(shutdown_signal()).into_future();
    info!("AuthRPC Server listening on {}", authrpc_url);

    info!("Servers started successfully. Press Ctrl+C to stop.");

    let res = tokio::try_join!(http_server, authrpc_server);
    match res {
        Ok(_) => {},
        Err(e) => info!("Error, shutting down servers: {:?}", e),
    }
}

async fn shutdown_signal(){
    tokio::signal::ctrl_c().await.expect("Failed to listen to the shutdown signal");
}


fn create_url(addr: &str, port: &str) -> String {
    format!("{}:{}", addr, port)
}

pub async fn handle_http_request(body: String) -> Json<Value> {
    let req: RpcRequest = serde_json::from_str(&body).unwrap();

    let res: Result<Value, RpcErr> = match req.method.as_str() {
        "engine_exchangeCapabilities" => exchange_capabilities(),
        "eth_chainId" => chain_id(),
        "eth_syncing" => syncing(),
        "eth_getBlockByNumber" => get_block_by_number(),
        _ => Err(RpcErr::MethodNotFound),
    };

    rpc_response(req, res)
}

pub async fn handle_authrpc_request(body: String) -> Json<Value>{
    let req: RpcRequest = serde_json::from_str(&body).unwrap();

    let res: Result<Value, RpcErr> = match req.method.as_str() {
        "engine_exchangeCapabilities" => exchange_capabilities(),
        _ => Err(RpcErr::MethodNotFound)
    };

    rpc_response(req, res)
}

fn rpc_response<E>(req: RpcRequest, res: Result<Value, E>) -> Json<Value>
where E: Into<RpcErrorMetadata>
{
    match res {
        Ok(result) => Json(
            serde_json::to_value(&RpcSuccessResponse {
                id: req.id,
                jsonrpc: "2.0".to_string(),
                result: result,
            }).unwrap(),
        ),
        Err(error) => Json(
            serde_json::to_value(&RpcErrorResponse {
                id: req.id,
                jsonrpc: "2.0".to_string(),
                error: error.into(),
            }).unwrap(),
        )
    }
}


