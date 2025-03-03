use std::collections::HashMap;

use bytes::Bytes;
use ethereum_types::{H256, U256};
use crate::rlp::encode::RLPEncode;
use super::GenesisAccount;


#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct Account {
    pub info: AccountInfo,
    pub code: Bytes,
    pub storage: HashMap<H256, H256>,
    pub balance: U256,
}

#[derive(Debug, PartialEq)]
pub struct AccountInfo {
    pub code_hash: H256,
    pub balance: U256,
    pub nonce: u64,
}

impl From<GenesisAccount> for Account {
    fn from(genesis: GenesisAccount) -> Self {
        Self {
            info: AccountInfo {
                code_hash: code_hash(&genesis.code),
                balance: genesis.balance,
                nonce: genesis.nonce,
            },
            code: genesis.code,
            storage: genesis.storage,
        }
    }
}

fn code_hash(code: &Bytes) -> H256 {
    keccak_hash::keccak(code.as_ref())
}

impl RLPEncode for AccountInfo {
    fn encode(&self, buf: &mut dyn bytes::BufMut) {
        self.code_hash.encode(buf);
        self.balance.encode(buf);
        self.nonce.encode(buf);
    }
}