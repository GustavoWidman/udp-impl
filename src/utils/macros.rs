#[allow(unused_macros)]
macro_rules! ipv4 {
    () => {{
        Ok::<_, std::io::Error>(std::net::SocketAddrV4::new(
            std::net::Ipv4Addr::UNSPECIFIED,
            0,
        ))
    }};
    ($ip:expr, random-port) => {{
        use std::str::FromStr;
        Ok::<_, std::io::Error>(std::net::SocketAddrV4::new(
            std::net::Ipv4Addr::from_str($ip)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?,
            rand::random(),
        ))
    }};
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

macro_rules! timeit {
    ($code: block) => {{
        let start = std::time::Instant::now();
        let result = $code;
        let elapsed = start.elapsed();
        (result, elapsed)
    }};
}

#[allow(unused_imports)]
pub(crate) use ipv4;
pub(crate) use timeit;
