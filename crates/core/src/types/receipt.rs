use crate::rlp::{encode::RLPEncode, structs::Encoder};
use bytes::Bytes;
use ethereum_types::{Address, Bloom, H256};
use super::TxType;
pub type Index = u64;


/// Result of a transaction
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Receipt {
    tx_type: TxType,
    succeeded: bool,
    cummulative_gas_used: u64,
    bloom: Bloom,
    logs: Vec<Log>,
}

impl Receipt {
    pub fn new(
        tx_type: TxType,
        succeeded: bool,
        cummulative_gas_used: u64,
        bloom: Bloom,
        logs: Vec<Log>,
    ) -> Self {
        Self {
            tx_type,
            succeeded,
            cummulative_gas_used,
            bloom,
            logs,
        }
    }
}

impl RLPEncode for Receipt {
    fn encode(&self, buf: &mut dyn bytes::BufMut) {
        Encoder::new(buf)
            .encode_field(&self.succeeded)
            .encode_field(&self.cummulative_gas_used)
            .encode_field(&self.bloom)
            .encode_field(&self.logs)
            .finish();
    }
}

/// Data record produced during the execution of a transaction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Log {
    address: Address,
    topics: Vec<H256>,
    data: Bytes,
}

impl RLPEncode for Log {
    fn encode(&self, buf: &mut dyn bytes::BufMut) {
        Encoder::new(buf)
            .encode_field(&self.address)
            .encode_field(&self.topics)
            .encode_field(&self.data)
            .finish();
    }
}