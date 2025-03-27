mod execution_result;
use crate::types::TxKind;

use super::{
    types::{Account, BlockHeader, Transaction},
    Address,
};
use revm::{
    inspector_handle_register,
    inspectors::TracerEip3155,
    primitives::{BlockEnv, Bytecode, TxEnv, U256},
    CacheState, Evm,
};
use std::collections::HashMap;

// Rename imported types for clarity
use revm::primitives::AccountInfo as RevmAccountInfo;
use revm::primitives::Address as RevmAddress;
use revm::primitives::TxKind as RevmTxKind;

// Export needed types
pub use execution_result::*;
pub use revm::primitives::SpecId;

pub fn execute_tx(
    tx: &Transaction,
    header: &BlockHeader,
    // TODO: Modify this type when we have a defined State structure
    pre: &HashMap<Address, Account>,
    spec_id: SpecId,
) -> ExecutionResult {
    let block_env = block_env(header);
}

fn block_env(header: &BlockHeader) -> BlockEnv {
    BlockEnv { 
        number: U256::from(header.number), 
        coinbase: RevmAddress(header.coinbase.0.into()), 
        timestamp: U256::from(header.timestamp), 
        gas_limit: U256::from(header.gas_limit), 
        basefee: U256::from(header.base_fee_per_gas), 
        difficulty: U256::from_limbs(header.difficulty.0), 
        prevrandao: Some(header.prev_randao.as_fixed_bytes().into()), 
        ..Default::default() 
    }
}

fn tx_env(tx: &Transaction) -> TxEnv {
    TxEnv { 
        caller: RevmAddress(tx.sender().0.into()), 
        gas_limit: tx.gas_limit(), 
        gas_price: U256::from(tx.gas_price()), 
        transact_to: tx.to().into(), 
        value: U256::from_limbs(tx.value().0), 
        data: tx.data().clone().into(), 
        nonce: Some(tx.nonce()), 
        chain_id: tx.chain_id(), 
        access_list: tx
            .access_list()
            .into_iter()
            .map(|(addr, list)| {
                (
                    RevmAddress(addr.0.into()),
                    list.into_iter().map(|a| U256::from_be_bytes(a.0)).collect(),
                )
            })
            .collect(), 
        gas_priority_fee: tx.max_priority_fee().map(U256::from),
        ..Default::default() 
    }
}

impl From<TxKind> for RevmTxKind {
    fn from(val: TxKind) -> Self {
        match val {
            TxKind::Call(address) => RevmTxKind::Call(address.0.into()),
            TxKind::Create => RevmTxKind::Create,
        }
    }
}