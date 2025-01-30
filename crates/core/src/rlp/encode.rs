use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use bytes::BufMut;
use tinyvec::ArrayVec;

pub trait RLPEncode {
    fn encode(&self, buf: &mut dyn BufMut);
    
    fn length(&self) -> usize {
        let mut buf = Vec::new();
        self.encode(&mut buf);
        buf.len()
    }
}

impl RLPEncode for bool {
    #[inline(always)]
    fn encode(&self, buf: &mut dyn BufMut) {
        if *self {
            buf.put_u8(0x01);
        } else {
            buf.put_u8(0x80);
        }
    }

    #[inline(always)]
    fn length(&self) -> usize {
        1
    }
}

impl RLPEncode for u8 {
    fn encode(&self, buf: &mut dyn BufMut) {
        match *self {
            0 => buf.put_u8(0x80),
            n @ 1..=0x7f => buf.put_u8(n),
            n => {
                let mut bytes = ArrayVec::<[u8; 8]>::new();
                bytes.extend_from_slice(&n.to_be_bytes());
                let start = bytes.iter().position(|&x| x != 0).unwrap();
                let len = bytes.len() - start;
                buf.put_u8(0x80 + len as u8);
                buf.put_slice(&bytes[start..]);
            }
        }
    }
}

impl RLPEncode for u16 {
    fn encode(&self, buf: &mut dyn BufMut) {
        match *self {
            0 => buf.put_u8(0x80),
            n@ 1..=0x7f => buf.put_u8(n as u8),
            n => {
                let mut bytes = ArrayVec::<[u8; 8]>::new();
                bytes.extend_from_slice(&n.to_be_bytes());
                let start = bytes.iter().position(|&x| x != 0).unwrap();
                let len = bytes.len() - start;
                buf.put_u8(0x80 + len as u8);
                buf.put_slice(&bytes[start..]);
            }
        }
    }
}

impl RLPEncode for u32 {
    fn encode(&self, buf: &mut dyn BufMut) {
        match *self {
            0 => buf.put_u8(0x80),
            n @ 1..0x7f => buf.put_u8(n),
            n => {
                let mut bytes = ArrayVec::<[u8; 8]>::new();
                bytes.extend_from_slice(&n.to_be_bytes());
                let start = bytes.iter().position(|&x| x != 0).unwrap();
                let len = bytes.len() - start;
                buf.put_u8(0x80 + len as u8);
                buf.put_slice(&bytes[start..]);
            }
        }
    }
}

impl RLPEncode for u64 {
    fn encode(&self, buf: &mut dyn BufMut) {
        match *self {
            0 => buf.put_u8(0x80),
            n @ 1..0x7f => buf.put_u8(n as u8),
            n => {
                let mut bytes = ArrayVec::<[u8; 8]>::new();
                bytes.extend_from_slice(&n.to_be_bytes());
                let start = bytes.iter().position(|&x| x != 0).unwrap();
                let len = bytes.len() - start;
                buf.put_u8(0x80 + len as u8);
                buf.put_slice(&bytes[start..]);
            }
        }
    }
}

impl RLPEncode for usize {
    fn encode(&self, buf: &mut dyn BufMut) {
        match *self {
            0 => buf.put_u8(0x80),
            n @ 1..=0x7f => buf.put_u8(n as u8),
            n => {
                let mut bytes = ArrayVec::<[u8; 8]>::new();
                bytes.extend_from_slice(&n.to_be_bytes());
                let start = bytes.iter().position(|&x|x != 0).unwrap();
                let len = bytes.len() - start;
                buf.put_u8(0x80 + len as u8);
                buf.put_slice(&bytes[start..]);
            }
        }
    }
}

impl RLPEncode for () {
    fn encode(&self, buf: &mut dyn BufMut) {
        buf.put_u8(0x80);
    }
}