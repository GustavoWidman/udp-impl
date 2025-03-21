use std::io::Result;

/// Trait for deserialization from bytes
pub trait FromBytes {
    fn from_bytes(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized;
}

/// Trait for serialization to bytes
pub trait ToBytes {
    fn to_bytes(&self) -> Result<Vec<u8>>;
}

// /// Trait for computing checksums according to RFC 1071
// pub trait Checksumable {
//     fn compute_checksum(&self, src_ip: Ipv4Addr, dst_ip: Ipv4Addr) -> u16;
// }
