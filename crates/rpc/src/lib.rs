use axum::{routing::post, Json, Router};
use serde_json::Value;
use engine::capabilities::exchange_capabilities;
use eth::{
    block::get_block_by_number, 
    client::{chain_id, syncing}
};

mod engine;
mod utils;
