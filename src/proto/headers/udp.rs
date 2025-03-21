use std::io::{Error, ErrorKind, Result};

use crate::common::traits::{FromBytes, ToBytes};

/// UDP Header (https://datatracker.ietf.org/doc/html/rfc768)
#[derive(Debug, Clone)]
pub struct UDPHeader {
    source_port: u16,       // 16 bits
    destination_port: u16,  // 16 bits
    pub(crate) length: u16, // Length of UDP header and data in bytes
    checksum: u16,          // internet checksum of pseudo header
}

impl UDPHeader {
    pub fn new(source_port: u16, destination_port: u16, payload_length: usize) -> Result<Self> {
        // total length is header size (8 bytes) + payload length
        let total_length = payload_length + 8;

        if total_length > u16::MAX as usize {
            return Err(Error::new(ErrorKind::InvalidData, "UDP payload too large"));
        }

        Ok(UDPHeader {
            source_port,
            destination_port,
            length: total_length as u16,
            checksum: 0, // uninitialized checksum
        })
    }

    pub fn set_checksum(&mut self, checksum: u16) {
        self.checksum = checksum;
    }
}

impl FromBytes for UDPHeader {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Not enough bytes for UDP header",
            ));
        }

        let source_port = u16::from_be_bytes([bytes[0], bytes[1]]);
        let destination_port = u16::from_be_bytes([bytes[2], bytes[3]]);
        let length = u16::from_be_bytes([bytes[4], bytes[5]]);
        let checksum = u16::from_be_bytes([bytes[6], bytes[7]]);

        Ok(UDPHeader {
            source_port,
            destination_port,
            length,
            checksum,
        })
    }
}

impl ToBytes for UDPHeader {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let source_port = self.source_port.to_be_bytes();
        let destination_port = self.destination_port.to_be_bytes();
        let length = self.length.to_be_bytes();
        let checksum = self.checksum.to_be_bytes();

        let mut header = Vec::with_capacity(8);

        // 16 bits (8 * 2)
        header.extend_from_slice(&source_port);
        // 16 bits (8 * 2)
        header.extend_from_slice(&destination_port);
        // 16 bits (8 * 2)
        header.extend_from_slice(&length);
        // 16 bits (8 * 2)
        header.extend_from_slice(&checksum);

        Ok(header)
    }
}
