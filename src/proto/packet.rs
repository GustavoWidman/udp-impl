use std::{
    io::{Error, ErrorKind, Result},
    net::{Ipv4Addr, SocketAddrV4},
};

use crate::{
    common::traits::{FromBytes, ToBytes},
    proto::headers::UDPPseudoHeader,
};

use super::headers::UDPHeader;

/// the full UDP packet (header and data)
#[derive(Debug, Clone)]
pub struct UDPPacket {
    pub(crate) header: UDPHeader,
    pub payload: Vec<u8>,
}

impl UDPPacket {
    pub fn new(
        source: &SocketAddrV4,
        destination: &SocketAddrV4,
        payload: Vec<u8>,
    ) -> Result<Self> {
        let header = UDPHeader::new(source.port(), destination.port(), payload.len())?;
        let mut packet = UDPPacket { header, payload };

        let checksum = packet.checksum(source.ip(), destination.ip())?;

        packet.header.set_checksum(checksum);

        Ok(packet)
    }

    // "internet checksum" https://datatracker.ietf.org/doc/html/rfc1071
    fn checksum(&self, src_ip: &Ipv4Addr, dst_ip: &Ipv4Addr) -> Result<u16> {
        // "pseudo header" for checksum calculation
        // we're effectively imagining what the header of our own packet would look
        // like if it was sent over the network, which if you think about it is
        // pretty fucking dumb and stupid, since the checksum is most likely
        // already calculated by someone else the time we get here. checksumming
        // our own data would be smarter, and we're technically doing that here
        // but there's no reason to also include information we shouldn't have
        let pseudo_header = UDPPseudoHeader::new(src_ip, dst_ip, self.header.length);

        // combine pseudo header, UDP header, and payload
        let mut checksum_data = pseudo_header.to_bytes()?;
        log::debug!(
            "Pseudo header bytes (len {}):\n\n{:?}\n",
            checksum_data.len(),
            checksum_data
        );
        checksum_data.extend_from_slice(&self.to_bytes()?);
        checksum_data.extend_from_slice(&self.payload);

        // pad with a zero byte if total length is odd
        if checksum_data.len() % 2 != 0 {
            checksum_data.push(0);
        }

        let mut sum: u32 = 0;

        // pair adjacent octets to form 16-bit integers
        for i in (0..checksum_data.len()).step_by(2) {
            let word = ((checksum_data[i] as u32) << 8) + checksum_data[i + 1] as u32;
            sum += word;
        }

        // add any carry bits
        while (sum >> 16) > 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        // take 1's complement
        let checksum = !(sum as u16);

        // if checksum is 0, use all ones
        // (as per RFC 768: "If the computed checksum is zero, it is transmitted as all ones")
        // why? idk, probably something to do with the math of all this...
        Ok(if checksum == 0 { 0xFFFF } else { checksum })
    }
}

impl ToBytes for UDPPacket {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = self.header.to_bytes()?;

        log::debug!("Header bytes (len {}):\n\n{:?}\n", bytes.len(), bytes); // TODO: remove

        bytes.extend_from_slice(&self.payload);

        Ok(bytes)
    }
}

impl FromBytes for UDPPacket {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Not enough bytes for UDP packet",
            ));
        }

        let header = UDPHeader::from_bytes(&bytes[0..8])?;

        if header.length as usize != bytes.len() {
            log::warn!(
                "Got {} bytes but expected {} (as per header). Ignoring...",
                bytes.len(),
                header.length
            ); // TODO: remove

            return Err(Error::new(ErrorKind::InvalidData, "Packet length mismatch"));
        }

        let payload = bytes[8..].to_vec();

        Ok(UDPPacket { header, payload })
    }
}
impl ToString for UDPPacket {
    fn to_string(&self) -> String {
        return String::from_utf8_lossy(&self.payload).to_string();
    }
}
