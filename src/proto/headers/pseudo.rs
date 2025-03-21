use std::{io::Result, net::Ipv4Addr};

use crate::common::traits::ToBytes;

// Pseudoheader used for checksum calculation
pub struct UDPPseudoHeader {
    src_addr: [u8; 4], // 32 bits (8 * 4)
    dst_addr: [u8; 4], // 32 bits (8 * 4)
    zeros: u8,         // 8 bits
    protocol: u8,      // 8 bits (always set to 17 for UDP)
    udp_length: u16,   // 16 bits (Length of UDP header + data)
}

/// https://en.wikipedia.org/wiki/User_Datagram_Protocol#IPv4_pseudo_header
impl UDPPseudoHeader {
    pub fn new(src_ip: &Ipv4Addr, dst_ip: &Ipv4Addr, udp_length: u16) -> Self {
        UDPPseudoHeader {
            src_addr: src_ip.octets(),
            dst_addr: dst_ip.octets(),
            zeros: 0,
            protocol: 17, // UDP protocol number
            udp_length,
        }
    }
}

impl ToBytes for UDPPseudoHeader {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let length = self.udp_length.to_be_bytes();

        let mut header = Vec::with_capacity(12);

        // 32 bits (8 * 4)
        header.extend_from_slice(&self.src_addr);
        // 32 bits (8 * 4)
        header.extend_from_slice(&self.dst_addr);
        // 8 bits (8 * 1)
        header.push(self.zeros);
        // 8 bits (8 * 1)
        header.push(self.protocol);
        // 16 bits (8 * 2)
        header.extend_from_slice(&length);

        Ok(header)
    }
}
