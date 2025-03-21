use std::{
    io::{Error, Result},
    mem,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

use libc::{AF_INET, IPPROTO_UDP, SOCK_RAW, bind, c_int, recvfrom, sendto, socket};

use crate::common::traits::{FromBytes, ToBytes};

use super::packet::UDPPacket;

/// UDP socket wrapper using socket2
pub struct UDPSocket {
    socket: i32,
    addr: SocketAddrV4,
}

impl UDPSocket {
    pub fn new(addr: SocketAddrV4) -> Result<Self> {
        let sock_addr = libc::sockaddr_in {
            #[cfg(target_os = "linux")]
            sin_family: libc::AF_INET as u16,
            #[cfg(target_os = "macos")]
            sin_family: libc::AF_INET as u8,
            sin_port: addr.port().to_be(),
            sin_addr: libc::in_addr {
                s_addr: u32::from_be_bytes(addr.ip().octets()).to_be(),
            },
            #[cfg(target_os = "macos")]
            sin_len: mem::size_of::<libc::sockaddr_in>() as u8,
            sin_zero: [0; 8],
        };

        let sock: c_int;
        unsafe {
            sock = socket(AF_INET, SOCK_RAW, IPPROTO_UDP);
            if sock < 0 {
                return Err(Error::last_os_error());
            }

            let res = bind(
                sock,
                &sock_addr as *const libc::sockaddr_in as *const libc::sockaddr,
                mem::size_of::<libc::sockaddr_in>() as u32,
            );
            if res < 0 {
                return Err(Error::last_os_error());
            }
        }

        Ok(UDPSocket { socket: sock, addr })
    }

    pub fn send_packet(&self, packet: &impl ToBytes, addr: &SocketAddrV4) -> Result<usize> {
        let sock_addr = libc::sockaddr_in {
            #[cfg(target_os = "linux")]
            sin_family: libc::AF_INET as u16,
            #[cfg(target_os = "macos")]
            sin_family: libc::AF_INET as u8,
            sin_port: addr.port().to_be(),
            sin_addr: libc::in_addr {
                s_addr: u32::from_be_bytes(addr.ip().octets()).to_be(),
            },
            sin_zero: [0; 8],
            #[cfg(target_os = "macos")]
            sin_len: mem::size_of::<libc::sockaddr_in>() as u8,
        };

        let packet_data = packet.to_bytes()?;
        let sent_bytes = unsafe {
            sendto(
                self.socket,
                packet_data.as_ptr() as *const libc::c_void,
                packet_data.len(),
                0,
                &sock_addr as *const libc::sockaddr_in as *const libc::sockaddr,
                mem::size_of::<libc::sockaddr_in>() as u32,
            )
        };

        if sent_bytes < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(sent_bytes as usize)
        }
    }

    pub fn send(&self, payload: Vec<u8>, addr: &SocketAddrV4) -> Result<usize> {
        let packet = UDPPacket::new(&self.addr, addr, payload)?;

        log::trace!("Sending packet\n\n{:?}\n", packet);

        self.send_packet(&packet, addr)
    }

    pub fn receive_packet(&self, buffer_size: usize) -> Result<(UDPPacket, SocketAddr)> {
        let mut buffer = vec![0; buffer_size];

        let mut sock_addr: libc::sockaddr_in = unsafe { mem::zeroed() };
        let mut addr_len = mem::size_of::<libc::sockaddr_in>() as u32;

        let bytes_received = unsafe {
            recvfrom(
                self.socket,
                buffer.as_mut_ptr() as *mut libc::c_void,
                buffer.len(),
                0,
                &mut sock_addr as *mut libc::sockaddr_in as *mut libc::sockaddr,
                &mut addr_len as *mut u32,
            )
        };

        if bytes_received < 0 {
            return Err(Error::last_os_error());
        }

        let ip = Ipv4Addr::from(u32::from_be(sock_addr.sin_addr.s_addr));
        let port = u16::from_be(sock_addr.sin_port);
        let addr = SocketAddrV4::new(ip, port);

        // skip forward 20 bytes to skip the IP header
        log::trace!(
            "Received packet\n\n{:?}\n",
            &buffer[20..bytes_received as usize]
        );
        let packet = UDPPacket::from_bytes(&buffer[20..bytes_received as usize])?;

        Ok((packet, std::net::SocketAddr::V4(addr)))
    }
}
