macro_rules! ipv4 {
    ($ip:expr, $port:expr) => {{
        use std::str::FromStr;
        Ok::<_, std::io::Error>(std::net::SocketAddrV4::new(
            std::net::Ipv4Addr::from_str($ip)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?,
            $port,
        ))
    }};
    ($addr:expr) => {{
        use std::str::FromStr;
        Ok::<_, std::io::Error>(
            std::net::SocketAddrV4::from_str($addr)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?,
        )
    }};
}

pub(crate) use ipv4;
