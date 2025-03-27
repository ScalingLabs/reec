use bytes::Bytes;
use reec_core::types::{
    code_hash, Account as ReecAccount, AccountInfo,
    EIP1559Transaction, LegacyTransaction, Transaction as ReecTransaction,
};

use reec_core::{types::BlockHeader, Address, Bloom, H256, U256, U64};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TestUnit {
    #[serde(default, rename = "_info")]
    pub info: Option<serde_json::Value>,
    pub blocks: Vec<Block>,
    pub genesis_block_header: Header,
    #[serde(rename = "genesisRLP")]
    pub genesis_rlp: serde_json::Value,
    pub lastblockhash: serde_json::Value,
    pub network: serde_json::Value,
    pub post_state: serde_json::Value,
    pub pre: HashMap<Address, Account>,
    pub seal_engine: serde_json::Value,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Account {
    pub balance: U256,
    #[serde(deserialize_with = "deser_hex_str")]
    pub code: Bytes,
    pub nonce: U256,
    pub storage: HashMap<U256, U256>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Env {
    pub current_coinbase: Address,
    pub current_difficulty: U256,
    pub current_gas_limit: U256,
    pub current_number: U256,
    pub current_timestamp: U256,
    pub current_base_fee: Option<U256>,
    pub previous_hash: Option<H256>,
    pub current_random: Option<H256>,
    pub current_beacon_root: Option<H256>,
    pub current_withdrawals_root: Option<H256>,
    pub parent_blob_gas_used: Option<U256>,
    pub parent_excess_blob_gas: Option<U256>,
    pub current_excess_blob_gas: Option<U256>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AccessListItem {
    pub address: Address,
    pub storage_keys: Vec<H256>,
}

pub type AccessList = Vec<AccessListItem>;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub bloom: Bloom,
    pub coinbase: Address,
    pub difficulty: U256,
    pub extra_data: Bytes,
    pub gas_limit: U256,
    pub gas_used: U256,
    pub hash: H256,
    pub mix_hash: H256,
    pub nonce: U64,
    pub number: U256,
    pub parent_hash: H256,
    pub receipt_trie: H256,
    pub state_root: U256,
    pub timestamp: U256,
    pub transactions_trie: H256,
    pub uncle_hash: H256,
    pub base_fee_per_gas: Option<U256>,
    pub withdrawals_root: Option<H256>,
    pub blob_gas_used: Option<U256>,
    pub excess_blob_gas: Option<U256>,
    pub parent_beacon_block_root: Option<H256>,
    pub requests_root: Option<H256>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub block_header: Option<Header>,
    pub rlp: Bytes,
    pub transactions: Option<Vec<Transaction>>,
    pub uncle_headers: Option<Vec<Header>>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[serde(rename = "type")]
    pub transaction_type: Option<U256>,
    #[serde(deserialize_with = "deser_hex_str")]
    pub data: Bytes,
    pub gas_limit: U256,
    pub gas_price: Option<U256>,
    pub nonce: U256,
    pub r: U256,
    pub s: U256,
    pub v: U256,
    pub value: U256,
    pub chain_id: Option<U256>,
    pub access_list: Option<AccessList>,
    pub max_fee_per_gas: Option<U256>,
    pub max_priority_fee_per_gas: Option<U256>,
    pub hash: Option<H256>,
    pub sender: Address,
    pub to: Address,
}

// Conversions between EFtests & Reec types

impl From<Header> for BlockHeader {
    fn from(val: Header) -> Self {
        BlockHeader { 
            parent_hash: val.parent_hash, 
            ommers_hash: val.uncle_hash, 
            coinbase: val.coinbase, 
            state_root: val.state_root, 
            transactions_root: val.transactions_trie, 
            receip_root: val.receipt_trie, 
            logs_bloom: val.bloom.into(), 
            difficulty: val.difficulty, 
            number: val.number.as_u64(), 
            gas_limit: val.gas_limit.as_u64(), 
            gas_used: val.gas_used.as_u64(), 
            timestamp: val.timestamp.as_u64(), 
            extra_data: val.extra_data, 
            prev_randao: val.extra_data, 
            nonce: val.mix_hash, 
            base_fee_per_gas: val.nonce.as_u64(), 
            withdrawals_root: val.base_fee_per_gas.unwrap().as_u64(), 
            blob_gas_used: val.blob_gas_used.unwrap().as_u64(), 
            excess_blob_gas: val.excess_blob_gas.unwrap().as_u64(), 
            parent_beacon_block_root: val.parent_beacon_block_root.unwrap(),
        }
    }
}

impl From<Transaction> for ReecTransaction {
    fn from(val: Transaction) -> Self {
        match val.transaction_type {
            Some(tx_type) => match tx_type.as_u64() {
                2 => ReecTransaction::EIP1559Transaction(val.into()),
                _ => unimplemented!(),
            },
            None => ReecTransaction::LegacyTransaction(val.into())
        }
    }
}