use std::io::Result;

mod cli;
mod common;
mod proto;
mod utils;

// Example usage
pub fn main() -> Result<()> {
    // Create a UDP socket for IPv4
    let src_addr = utils::macros::ipv4!("127.0.0.1:8080")?;
    let src_socket = proto::socket::UDPSocket::new(src_addr)?;
    println!("Created source UDP socket for IPv4. Bound to {}", src_addr);

    let dst_addr = utils::macros::ipv4!("127.0.0.1:9090")?;
    let dst_socket = proto::socket::UDPSocket::new(dst_addr)?;
    println!(
        "Created destination UDP socket for IPv4. Bound to {}",
        dst_addr
    );

    // Create a UDP packet
    let payload = b"Hello, UDP!".to_vec();
    src_socket.send(payload, &dst_addr)?;

    println!("Packet sent! Waiting for response...");

    // Receive a response
    let (response, from_addr) = dst_socket.receive_packet(1024)?;
    println!(
        "Received response from {:?}: {:?}",
        from_addr,
        response.to_string()
    );

    Ok(())
}

// pub fn main() -> Result<()> {
//     // using rust's std::net::UdpSocket
//     let src_addr = utils::macros::ipv4!("127.0.0.1:8080")?;
//     let src_socket = std::net::UdpSocket::bind(src_addr)?;
//     println!("Created source UDP socket for IPv4. Bound to {}", src_addr);

//     let dst_addr = utils::macros::ipv4!("127.0.0.1:9090")?;
//     let dst_socket = std::net::UdpSocket::bind(dst_addr)?;
//     println!(
//         "Created destination UDP socket for IPv4. Bound to {}",
//         dst_addr
//     );

//     // Create a UDP packet
//     let payload = b"Hello, UDP!".to_vec();
//     src_socket.send_to(&payload, &dst_addr)?;

//     println!("Packet sent! Waiting for response...");

//     // Receive a response
//     let mut buf = [0; 1024];
//     let (n, from_addr) = dst_socket.recv_from(&mut buf)?;
//     println!(
//         "Received response from {:?}: {:?}",
//         from_addr,
//         String::from_utf8_lossy(&buf[..n])
//     );

//     Ok(())
// }
