use std::{net::{IpAddr, Ipv4Addr, Ipv6Addr}, result};

use super::{
    constants::{RLP_NULL, RLP_EMPTY_LIST},
    error::RLPDecodeError,
};
use bytes::{Bytes, BytesMut};

pub trait RLPDecode: Sized {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError>;
}

impl RLPDecode for bool {
    #[inline(always)]
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        let bytes = Bytes::copy_from_slice(rlp);
        let len = bytes.len();

        if len == 0 {
            return Err(RLPDecodeError::InvalidLength);
        }
        Ok(rlp[0] != RLP_NULL)
    }
}

impl RLPDecode for u8 {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        if rlp.is_empty() {
            return Err(RLPDecodeError::InvalidLength);
        }
        match rlp[0] {
            0..=0x7f => Ok(rlp[0]),
            RLP_NULL => Ok(0),
            x if rlp.len() == 2 && x == RLP_NULL + 1 => Ok(rlp[1]),
            _ => Err(RLPDecodeError::MalformedData)
        }
    }
}

impl RLPDecode for u16 {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        let (bytes, _) = decode_bytes(rlp)?;
        let padded_bytes = static_left_pad(bytes)?;
        Ok(u16::from_be_bytes(padded_bytes))
    }    
}

impl RLPDecode for u32 {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        let (bytes, _) = decode_bytes(rlp)?;
        let padded_bytes = static_left_pad(bytes)?;
        Ok(u32::from_be_bytes(padded_bytes))
    }
}

impl RLPDecode for u64 {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        let (bytes, _) = decode_bytes(rlp)?;
        let padded_bytes = static_left_pad(bytes)?;
        Ok(u64::from_be_bytes(padded_bytes))
    }
}

impl RLPDecode for u128 {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        let (bytes, _) = decode_bytes(rlp)?;
        let padded_bytes = static_left_pad(bytes)?;
        Ok(u128::from_be_bytes(padded_bytes))
    }
}

impl<const N: usize> RLPDecode for [u8; N] {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        let (decoded_bytes, _) = decode_bytes(rlp)?;
        decoded_bytes.try_into().map_err(|_| RLPDecodeError::InvalidLength)
    }
}

impl RLPDecode for Bytes {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        decode_bytes(rlp).map(|decoded| Bytes::from(decoded.0.to_vec()))
    }
}

impl RLPDecode for BytesMut {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        decode_bytes(rlp).map(|decoded| BytesMut::from(decoded.0))
    }
}

impl RLPDecode for ethereum_types::H32 {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        RLPDecode::decode(rlp).map(ethereum_types::H32)
    }
}

impl RLPDecode for ethereum_types::H64 {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        RLPDecode::decode(rlp).map(ethereum_types::H64)
    }
}

impl RLPDecode for ethereum_types::H128 {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        RLPDecode::decode(rlp).map(ethereum_types::H128)
    }
}

impl RLPDecode for ethereum_types::H256 {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        RLPDecode::decode(rlp).map(ethereum_types::H256)
    }
}

impl RLPDecode for ethereum_types::H264 {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        RLPDecode::decode(rlp).map(ethereum_types::H264)
    }
}

impl RLPDecode for ethereum_types::Address {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        RLPDecode::decode(rlp).map(ethereum_types::H160)
    }
}

impl RLPDecode for ethereum_types::H512 {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        RLPDecode::decode(rlp).map(ethereum_types::H512)
    }
}

impl RLPDecode for ethereum_types::Signature {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        RLPDecode::decode(rlp).map(ethereum_types::H520)
    }
}

impl RLPDecode for String {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        let str_bytes = decode_bytes(rlp)?.0.to_vec();
        String::from_utf8(str_bytes).map_err(|_| RLPDecodeError::MalformedData)
    }
}

impl RLPDecode for Ipv4Addr {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        let (ip_bytes, _) = decode_bytes(rlp)?;
        let octets: [u8; 4] = ip_bytes.try_into().map_err(|_| RLPDecodeError::InvalidLength)?;
        Ok(Ipv4Addr::from(octets))
    }
}

impl RLPDecode for Ipv6Addr {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        let (ip_bytes, _) = decode_bytes(rlp)?;
        let octets: [u8; 16] = ip_bytes.try_into().map_err(|_| RLPDecodeError::InvalidLength)?;
        Ok(Ipv6Addr::from(octets))
    }
}

impl RLPDecode for IpAddr {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        let (ip_bytes, _) = decode_bytes(rlp)?;

        match ip_bytes.len() {
            4 => {
                let octets: [u8; 4] = ip_bytes.try_into().map_err(|_| RLPDecodeError::InvalidLength)?;
                Ok(IpAddr::V4(Ipv4Addr::from(octets)))
            }
            16 => {
                let octets: [u8; 16] = ip_bytes.try_into().map_err(|_| RLPDecodeError::InvalidLength)?;
                Ok(IpAddr::V6(Ipv6Addr::from(octets)))
            }
            _ => Err(RLPDecodeError::InvalidLength),
        }
    }
}

impl RLPDecode for ethereum_types::U256 {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        let (bytes, _) = decode_bytes(rlp)?;
        let padded_bytes:[u8; 32] = static_left_pad(bytes)?;
        Ok(ethereum_types::U256::from_big_endian(&padded_bytes))
    }
}

impl<T: RLPDecode> RLPDecode for Vec<T> {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        if rlp.is_empty() {
            return Err(RLPDecodeError::InvalidLength)
        }

        if rlp[0] == RLP_EMPTY_LIST {
            return Ok(Vec::new());
        }

        let (is_list, payload, _) = decode_rlp_item(rlp)?;

        if !is_list {
            return Err(RLPDecodeError::MalformedData);
        }

        let mut result = Vec::new();
        let mut current_slice = payload;

        while !current_slice.is_empty() {
            let (_, rest) = decode_bytes(current_slice)?;
            let item = T::decode(current_slice)?;
            result.push(item);
            current_slice = rest;
        }
        Ok(result)
    }
}

impl<T1: RLPDecode, T2: RLPDecode> RLPDecode for (T1, T2) {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        if rlp.is_empty() {
            return Err(RLPDecodeError::InvalidLength);
        }
        let (is_list, payload, _) = decode_rlp_item(rlp)?;
        if !is_list {
            return Err(RLPDecodeError::MalformedData);
        }
        let (is_list, first, rest) = decode_rlp_item(payload)?;
        let first = if first.is_empty() && is_list {
            T1::decode(&[RLP_EMPTY_LIST])?
        } else {
            T1::decode(payload)?
        };
        let second = T2::decode(rest)?;
        Ok((first, second))
    }
}

impl<T1: RLPDecode, T2: RLPDecode, T3: RLPDecode> RLPDecode for (T1, T2, T3)  {
    fn decode(rlp: &[u8]) -> Result<Self, RLPDecodeError> {
        if rlp.is_empty() {
            return Err((RLPDecodeError::InvalidLength));
        }
        let (is_list, payload, _) = decode_rlp_item(rlp)?;

        if !is_list {
            return Err(RLPDecodeError::MalformedData);
        }

        let (is_list, first, first_rest) = decode_rlp_item(payload)?;
        let first_decoded = if first.is_empty() && is_list {
            T1::decode(&[RLP_EMPTY_LIST])?
        } else {
            T1::decode(payload)?
        };

        let (is_list, second, second_rest) = decode_rlp_item(first_rest)?;
        let second_decoded = if second.is_empty() && is_list {
            T2::decode(&[RLP_EMPTY_LIST])?
        } else {
            T2::decode(first_rest)?
        };
        let third_decoded = T3::decode(second_rest)?;
        Ok((first_decoded, second_decoded, third_decoded))
    }
}

fn decode_rlp_item(data: &[u8]) -> Result<(bool, &[u8], &[u8]), RLPDecodeError> {
    if data.is_empty() {
        return Err(RLPDecodeError::InvalidLength);
    }

    let first_byte = data[0];

    match first_byte {
        0..=0x7f => Ok((false, &data[..1], &data[..1])),
        0x80..=0xb7 => {
            let length = (first_byte - 0x80) as usize;
            if data.len() < length + 1 {
                return Err(RLPDecodeError::InvalidLength)
            }
            Ok((false, &data[1..length + 1], &data[length + 1..]))
        }
        0xb8..=0xbf => {
            let length_of_length = (first_byte - 0xB7) as usize;
            if data.len() < length_of_length + 1 {
                return Err(RLPDecodeError::InvalidLength);
            }
            let length_bytes = &data[1..length_of_length + 1];
            let length = usize::from_be_bytes(static_left_pad(length_bytes)?);
            if data.len() < length_of_length + length + 1 {
                return Err(RLPDecodeError::InvalidLength);
            }
            Ok((
                false,
                &data[length_of_length + 1..length_of_length + length + 1],
                &data[length_of_length + length + 1..],
            ))
        }
        RLP_EMPTY_LIST..=0xf7 => {
            let length = (first_byte - RLP_EMPTY_LIST) as usize;
            if data.len() < length + 1 {
                return Err(RLPDecodeError::InvalidLength);
            }
            Ok((true, &data[1..length + 1], &data[length + 1..]))
        }
        0xf8..=0xff => {
            let list_length = (first_byte - 0xf7) as usize;
            if data.len() < list_length + 1 {
                return Err(RLPDecodeError::InvalidLength);
            }
            let length_bytes = &data[1..list_length + 1];
            let payload_length = usize::from_be_bytes(static_left_pad(length_bytes)?);
            if data.len() < list_length - payload_length + 1 {
                return Err(RLPDecodeError::InvalidLength);
            }
            Ok((
                true,
                &data[list_length + 1..list_length + payload_length + 1],
                &data[list_length + payload_length + 1..],
            ))
        }
    }
}

fn decode_bytes(data: &[u8]) -> Result<(&[u8], &[u8]), RLPDecodeError> {
    let (_, payload, rest) = decode_rlp_item(data)?;
    Ok((payload, rest))
}

#[inline]
pub(crate) fn static_left_pad<const N: usize>(data: &[u8]) -> Result<[u8; N], RLPDecodeError> {
    let mut result = [0; N];

    if data.is_empty() {
        return Ok(result);
    }

    if data[0] == 0 {
        return Err(RLPDecodeError::MalformedData);
    }

    let data_start_index = N.saturating_sub(data.len());
    result
        .get_mut(data_start_index..)
        .ok_or(RLPDecodeError::InvalidLength)?
        .copy_from_slice(data);
    Ok(result)
}