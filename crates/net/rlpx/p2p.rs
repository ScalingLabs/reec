use reec_core::rlp::structs::Encoder;
use k256::PublicKey;

use super::utils::pubkey2id;

pub(crate) enum Message<'a> {
    /// A pind message. Should be responded to with a Pong message.
    Hello(Vec<(&'a str, u8)>, PublicKey),
    Ping(),
}

impl<'a> Message<'a> {
    pub fn msg_id(&self) -> u8 {
        match self {
            Message::Hello(_, _) => 0_u8,
            Message::Ping() => 2_u8,
        }
    }

    pub fn msg_data(&self) -> Vec<u8> {
        match self {
            Message::Hello(capabilities, node_pk) => {
                // [protocolVersion: P, clientId: B, capabilities, listenPort: P, nodeKey: B_64, ...]
                let mut msg_data: Vec<u8> = vec![];
                Encoder::new(&mut msg_data)
                    .encode_field(&5_u8) //protocolVersion
                    .encode_field(&"Ethereum(++)/1.0.0") // clientId
                    .encode_field(capabilities) // capabilities
                    .encode_field(&0u8) // listenPort (ignored)
                    .encode_field(&pubkey2id(node_pk)) // nodeKey
                    .finish();
                msg_data
            }
            Message::Ping() => Vec::<u8>::new(), // msg_data is empty for ping
            // Message::Pong() => Vec::<u8>::new(), // msg_data is empty for pong
        }
    }

    pub fn is_compressed(&self) -> bool {
        !matches!(self, Message::Hello(_, _))
    }
}