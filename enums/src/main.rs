fn main() {
    // Custom enum for storing IP addresses
    enum CustomIpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let _home = CustomIpAddr::V4(127, 0, 0, 1);
    let _loopback = CustomIpAddr::V6(String::from("::1"));

    // Enum using types from the stdlib
    // Also available at std::net::IpAddr
    use std::net::Ipv4Addr;
    use std::net::Ipv6Addr;
    enum _IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
}
