fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // Attach data to each variant of enum directly
    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String)
    }

    let home2 = IpAddr2::V4(127,0,0,1);
    let loopback2 = IpAddr::V6(String::from("::1"));

    // More complex data attachment?
    struct Ipv4Addr {
        // Put everything in here
    }

    struct Ipv6Addr {
        // Put everything in here
    }

    enum IpAddr3 {
        V4(Ipv4Addr),
        V6(Ipv6Addr)
    }

    // Also can implement methods like struct
    impl IpAddr2 {
        fn print(&self){
            // do something
        }
    }

    let m = IpAddr2::V4(127,0,0,1)
    m.print();
}
