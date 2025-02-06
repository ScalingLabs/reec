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

impl RLPEncode for [u8] {
    #[inline(always)]
    fn encode(&self, buf: &mut dyn BufMut) {
        if self.len() == 1 && self[0] < 0x80 {
            buf.put_u8(self[0]);
        } else {
            let len = self.len();
            if len < 56 {
                buf.put_u8(0x80 + len as u8);
            } else {
                let mut bytes = ArrayVec::<[u8; 8]>::new();
                bytes.extend_from_slice(&len.to_be_bytes());
                let start = bytes.iter().position(|&x|x != 0).unwrap();
                let len = bytes.len() - start;
                buf.put_u8(0x7f + len as u8);
                buf.put_slice(&bytes[start..]);
            }
            buf.put_slice(self);
        }
    }
}

impl <const N: usize> RLPEncode for [u8; N] {
    fn encode(&self, buf: &mut dyn BufMut) {
        self.as_ref().encode(buf);
    }
}

impl RLPEncode for str {
    fn encode(&self, buf: &mut dyn BufMut) {
        self.as_bytes().encode(buf);
    }
}

impl RLPEncode for &str {
    fn encode(&self, buf: &mut dyn BufMut) {
        self.as_bytes().encode(buf);
    }
}

impl RLPEncode for String {
    fn encode(&self, buf: &mut dyn BufMut) {
        self.as_bytes().encode(buf);
    }
}

impl <T: RLPEncode> RLPEncode for Vec<T> {
    fn encode(&self, buf: &mut dyn BufMut) {
        if self.is_empty() {
            buf.put_u8(0xc0);
        } else {
            let mut total_len = 0;
            for item in self {
                total_len += item.length();
            }
            if total_len < 56 {
                buf.put_u8(0xc0 + total_len as u8);
            } else {
                let mut bytes = ArrayVec::<[u8; 8]>::new();
                bytes.extend_from_slice(&total_len.to_be_bytes());
                let start = bytes.iter().position(|&x| x!= 0).unwrap();
                let len = bytes.len() - start;
                buf.put_u8(0x7f + len as u8);
                buf.put_slice(&bytes[start..]);
            }
            for item in self {
                item.encode(buf);
            }
        }
    }
}

impl RLPEncode for Ipv4Addr {
    fn encode(&self, buf: &mut dyn BufMut) {
        self.octets().encode(buf);
    }
}

impl RLPEncode for Ipv6Addr {
    fn encode(&self, buf: &mut dyn BufMut) {
        self.octets().encode(buf);
    }
}

impl RLPEncode for IpAddr {
    fn encode(&self, buf: &mut dyn BufMut) {
        match self {
            IpAddr::V4(ip) => ip.encode(buf),
            IpAddr::V6(ip) => ip.encode(buf),
        }
    }
}

impl RLPEncode for ethereum_types::H32 {
    fn encode(&self, buf: &mut dyn BufMut) {
        self.as_bytes().encode(buf)
    }
}

impl RLPEncode for ethereum_types::H64 {
    fn encode(&self, buf: &mut dyn BufMut) {
        self.as_bytes().encode(buf);
    }
}

impl RLPEncode for ethereum_types::H128 {
    fn encode(&self, buf: &mut dyn BufMut) {
        self.as_bytes().encode(buf);
    }
}

impl RLPEncode for ethereum_types::H256 {
    fn encode(&self, buf: &mut dyn BufMut) {
        self.as_bytes().encode(buf);
    }
}

impl RLPEncode for ethereum_types::H512 {
    fn encode(&self, buf: &mut dyn BufMut) {
        self.as_bytes().encode(buf);
    }
}

impl RLPEncode for ethereum_types::Address {
    fn encode(&self, buf: &mut dyn BufMut) {
        self.as_bytes().encode(buf);
    }
}

impl RLPEncode for ethereum_types::Signature {
    fn encode(&self, buf: &mut dyn BufMut) {
        self.as_bytes().encode(buf);
    }
}


#[cfg(test)]
mod test {
    use std::net::IpAddr;
    use ethereum_types::Address;
    use hex_literal::hex;
    use super::RLPEncode;

    #[test]
    fn can_encode_booleans() {
        let mut encoded = Vec::new();
        true.encode(&mut encoded);
        assert_eq!(encoded, vec![0x01]);

        let mut encoded = Vec::new();
        false.encode(&mut encoded);
        assert_eq!(encoded, vec![0x80]);
    }

    #[test]
    fn can_encode_u8() {
        let mut encoded = Vec::new();
        0u8.encode(&mut encoded);
        assert_eq!(encoded, vec![0x80]);

        let mut encoded = Vec::new();
        1u8.encode(&mut encoded);
        assert_eq!(encoded, vec![0x01]);

        let mut encoded = Vec::new();
        0x7fu8.encode(&mut encoded);
        assert_eq!(encoded, vec![0x7f]);

        let mut encoded = Vec::new();
        0x80u8.encode(&mut encoded);
        assert_eq!(encoded, vec![0x80 + 1, 0x80]);

        let mut encoded = Vec::new();
        0x90u8.encode(&mut encoded);
        assert_eq!(encoded, vec![0x80 + 1, 0x90]);
    }

    #[test]
    fn can_encode_u16() {
        let mut encoded = Vec::new();
        0u16.encode(&mut encoded);
        assert_eq!(encoded, vec![0x80]);

        let mut encoded = Vec::new();
        1u16.encode(&mut encoded);
        assert_eq!(encoded, vec![0x01]);

        let mut encoded = Vec::new();
        0x7fu16.encode(&mut encoded);
        assert_eq!(encoded, vec![0x7f]);

        let mut encoded = Vec::new();
        0x80u16.encode(&mut encoded);
        assert_eq!(encoded, vec![0x80 + 1, 0x80]);

        let mut encoded = Vec::new();
        0x90u16.encode(&mut encoded);
        assert_eq!(encoded, vec![0x80 + 1, 0x90]);
    }

    #[test]
    fn can_encode_u32() {
        let mut encoded = Vec::new();
        0u16.encode(&mut encoded);
        assert_eq!(encoded, vec![0x80]);

        let mut encoded = Vec::new();
        1u16.encode(&mut encoded);
        assert_eq!(encoded, vec![0x01]);

        let mut encoded = Vec::new();
        0x7fu16.encode(&mut encoded);
        assert_eq!(encoded, vec![0x7f]);

        let mut encoded = Vec::new();
        0x80u16.encode(&mut encoded);
        assert_eq!(encoded, vec![0x80 + 1, 0x80]);

        let mut encoded = Vec::new();
        0x90u16.encode(&mut encoded);
        assert_eq!(encoded, vec![0x80 + 1, 0x90]);
    }

    #[test]
    fn can_encode_u64() {
        let mut encoded = Vec::new();
        0u8.encode(&mut encoded);
        assert_eq!(encoded, vec![0x80]);

        let mut encoded = Vec::new();
        1u8.encode(&mut encoded);
        assert_eq!(encoded, vec![0x01]);

        let mut encoded = Vec::new();
        0x7fu8.encode(&mut encoded);
        assert_eq!(encoded, vec![0x7f]);

        let mut encoded = Vec::new();
        0x80u8.encode(&mut encoded);
        assert_eq!(encoded, vec![0x80 + 1, 0x80]);

        let mut encoded = Vec::new();
        0x90u8.encode(&mut encoded);
        assert_eq!(encoded, vec![0x80 + 1, 0x90]);
    }

    #[test]
    fn can_encode_usize() {
        let mut encoded = Vec::new();
        0u8.encode(&mut encoded);
        assert_eq!(encoded, vec![0x80]);

        let mut encoded = Vec::new();
        1u8.encode(&mut encoded);
        assert_eq!(encoded, vec![0x01]);

        let mut encoded = Vec::new();
        0x7fu8.encode(&mut encoded);
        assert_eq!(encoded, vec![0x7f]);

        let mut encoded = Vec::new();
        0x80u8.encode(&mut encoded);
        assert_eq!(encoded, vec![0x80 + 1, 0x80]);

        let mut encoded = Vec::new();
        0x90u8.encode(&mut encoded);
        assert_eq!(encoded, vec![0x80 + 1, 0x90]);
    }

    #[test]
    fn can_encode_bytes() {
        let message: [u8; 1] = [0x00];
        let encoded = {
            let mut buf = vec![];
            message.encode(&mut buf);
            buf
        };
        assert_eq!(encoded, [0x00]);

        let message: [u8; 1] = [0x0f];
        let encoded = {
            let mut buf = vec![];
            message.encode(&mut buf);
            buf
        };
        assert_eq!(encoded, [0x0f]);

        let message: [u8; 2] = [0x04, 0x00];
        let encoded = {
            let mut buf = vec![];
            message.encode(&mut buf);
            buf
        };
        assert_eq!(encoded, [0x82, 0x04, 0x00])
    }

    #[test]
    fn can_encode_strings() {
        let message = "cat";
        let encoded = {
            let mut buf = vec![];
            message.encode(&mut buf);
            buf
        };
        let expected:[u8; 4] = [0x83, b'c', b'a', b't']; 
        assert_eq!(encoded, expected);
    }

    #[test]
    fn can_encode_list_of_strings() {
        let message = vec!["dog", "cat"];
        let encoded = {
            let mut buf = vec![];
            message.encode(&mut buf);
            buf
        };

        let expected:[u8; 9] = [0xc8, 0x83, b'd', b'o', b'g', 0x83, b'c', b'a', b't'];
        assert_eq!(encoded, expected);

        let message: Vec<&str> = vec![];
        let encoded = {
            let mut buf = vec![];
            message.encode(&mut buf);
            buf
        };
        let expected: [u8; 1] = [0xc0]; 
        assert_eq!(encoded, expected);
    }

    #[test]
    fn can_encode_ip() {
        let message = "192.168.0.1";
        let ip: IpAddr = message.parse().unwrap();
        let encoded = {
            let mut buf = vec![];
            ip.encode(&mut buf);
            buf
        };

        let expected:[u8; 5] = [0x84, 192, 168, 0, 1];
        assert_eq!(encoded, expected);

        let message = "2001:0000:130F:0000:0000:09C0:876A:130B";
        let ip: IpAddr = message.parse().unwrap();
        let encoded = {
            let mut buf = vec![];
            ip.encode(&mut buf);
            buf
        };
        let expected: [u8; 17] = [0x90, 0x20, 0x01, 0x00, 0x00, 0x13, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x09, 0xc0, 0x87,
        0x6a, 0x13, 0x0b];
        assert_eq!(encoded, expected)
    }

    #[test]
    fn can_encode_addresses() {
        let address = Address::from(hex!("ef2d6d194084c2de36e0dabfce45d046b37d1106"));
        let encoded = {
            let mut buf = vec![];
            address.encode(&mut buf);
            buf
        };

        let expected = hex!("94ef2d6d194084c2de36e0dabfce45d046b37d1106");
        assert_eq!(encoded, expected);
    }
}
