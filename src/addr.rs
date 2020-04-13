use addr_hal::Ipv4Address;
use addr_hal::Ipv6Address;
use addr_hal::SocketAddressV4;
use addr_hal::SocketAddressV6;

use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::net::SocketAddrV4;
use std::net::SocketAddrV6;

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Debug)]
pub struct IpAddressV4Inner {
    pub inner: Ipv4Addr,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SocketV4Inner {
    pub inner: SocketAddrV4,
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Debug)]
pub struct IpAddressV6Inner {
    pub inner: Ipv6Addr,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SocketV6Inner {
    pub inner: SocketAddrV6,
}

impl Ipv4Address for IpAddressV4Inner {
    const LOCALHOST: Self = IpAddressV4Inner {
        inner: Ipv4Addr::new(127, 0, 0, 1),
    };
    const UNSPECIFIED: Self = IpAddressV4Inner {
        inner: Ipv4Addr::new(0, 0, 0, 0),
    };
    const BROADCAST: Self = IpAddressV4Inner {
        inner: Ipv4Addr::new(255, 255, 255, 255),
    };

    fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self {
            inner: Ipv4Addr::new(a, b, c, d),
        }
    }

    fn octets(&self) -> [u8; 4] {
        self.inner.octets()
    }
}

impl SocketAddressV4 for SocketV4Inner {
    type IpAddress = IpAddressV4Inner;

    fn new(ip: addr_hal::Ipv4Addr<Self::IpAddress>, port: u16) -> Self {
        let oct = ip.octets();
        let std_ipv4 = Ipv4Addr::new(oct[0], oct[1], oct[2], oct[3]);
        Self {
            inner: SocketAddrV4::new(std_ipv4, port),
        }
    }

    fn ip(&self) -> &addr_hal::Ipv4Addr<Self::IpAddress> {
        let ip = self.inner.ip();
        unsafe { &*(ip as *const Ipv4Addr as *const addr_hal::Ipv4Addr<Self::IpAddress>) }
    }

    fn set_ip(&mut self, ip: addr_hal::Ipv4Addr<Self::IpAddress>) {
        let oct = ip.octets();
        self.inner
            .set_ip(Ipv4Addr::new(oct[0], oct[1], oct[2], oct[3]));
    }

    fn port(&self) -> u16 {
        self.inner.port()
    }

    fn set_port(&mut self, port: u16) {
        self.inner.set_port(port);
    }
}

impl Ipv6Address for IpAddressV6Inner {
    const LOCALHOST: Self = IpAddressV6Inner {
        inner: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
    };
    const UNSPECIFIED: Self = IpAddressV6Inner {
        inner: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
    };

    fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Self {
        Self {
            inner: Ipv6Addr::new(a, b, c, d, e, f, g, h),
        }
    }

    fn segments(&self) -> [u16; 8] {
        self.inner.segments()
    }
}

impl SocketAddressV6 for SocketV6Inner {
    type IpAddress = IpAddressV6Inner;

    fn new(
        ip: addr_hal::Ipv6Addr<Self::IpAddress>,
        port: u16,
        flowinfo: u32,
        scope_id: u32,
    ) -> Self {
        let seg = ip.segments();
        let std_ipv6 = Ipv6Addr::new(
            seg[0], seg[1], seg[2], seg[3], seg[4], seg[5], seg[6], seg[7],
        );
        Self {
            inner: SocketAddrV6::new(std_ipv6, port, flowinfo, scope_id),
        }
    }

    fn ip(&self) -> &addr_hal::Ipv6Addr<Self::IpAddress> {
        let ip = self.inner.ip();
        unsafe { &*(ip as *const Ipv6Addr as *const addr_hal::Ipv6Addr<Self::IpAddress>) }
    }

    fn set_ip(&mut self, new_ip: addr_hal::Ipv6Addr<Self::IpAddress>) {
        let seg = new_ip.segments();
        self.inner.set_ip(Ipv6Addr::new(
            seg[0], seg[1], seg[2], seg[3], seg[4], seg[5], seg[6], seg[7],
        ));
    }

    fn port(&self) -> u16 {
        self.inner.port()
    }

    fn set_port(&mut self, new_port: u16) {
        self.inner.set_port(new_port);
    }

    fn flowinfo(&self) -> u32 {
        self.inner.flowinfo()
    }

    fn set_flowinfo(&mut self, new_flowinfo: u32) {
        self.inner.set_flowinfo(new_flowinfo);
    }

    fn scope_id(&self) -> u32 {
        self.inner.scope_id()
    }

    fn set_scope_id(&mut self, new_scope_id: u32) {
        self.inner.set_scope_id(new_scope_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ip4_new() {
        let addr = addr_hal::Ipv4Addr::<IpAddressV4Inner>::new(127, 0, 0, 1);
        assert_eq!("127.0.0.1".parse(), Ok(addr));
        assert_eq!(addr.is_loopback(), true);

        let localhost = IpAddressV4Inner::LOCALHOST;
        assert_eq!(localhost, IpAddressV4Inner::new(127, 0, 0, 1));

        let unspecified = IpAddressV4Inner::UNSPECIFIED;
        assert_eq!(unspecified, IpAddressV4Inner::new(0, 0, 0, 0));

        let broadcast = IpAddressV4Inner::BROADCAST;
        assert_eq!(broadcast, IpAddressV4Inner::new(255, 255, 255, 255));

        assert_eq!(addr.octets(), [127, 0, 0, 1]);
    }

    #[test]
    fn sock4_new() {
        let mut socket = addr_hal::SocketAddrV4::<SocketV4Inner>::new(
            addr_hal::Ipv4Addr::<IpAddressV4Inner>::new(127, 0, 0, 1),
            8080,
        );
        assert_eq!("127.0.0.1:8080".parse(), Ok(socket));

        assert_eq!(
            socket.ip(),
            &addr_hal::Ipv4Addr::<IpAddressV4Inner>::new(127, 0, 0, 1)
        );

        socket.set_ip(addr_hal::Ipv4Addr::<IpAddressV4Inner>::new(192, 168, 75, 1));
        assert_eq!(
            socket.ip(),
            &addr_hal::Ipv4Addr::<IpAddressV4Inner>::new(192, 168, 75, 1)
        );

        assert_eq!(socket.port(), 8080);

        socket.set_port(4242);
        assert_eq!(socket.port(), 4242);
    }

    #[test]
    fn ip6_new() {
        let addr6 = addr_hal::Ipv6Addr::<IpAddressV6Inner>::new(0, 0, 0, 0, 0, 0, 0, 1);
        assert_eq!("::1".parse(), Ok(addr6));
        assert_eq!(addr6.is_loopback(), true);

        let localhost = IpAddressV6Inner::LOCALHOST;
        assert_eq!(localhost, IpAddressV6Inner::new(0, 0, 0, 0, 0, 0, 0, 1));

        let unspecified = IpAddressV6Inner::UNSPECIFIED;
        assert_eq!(unspecified, IpAddressV6Inner::new(0, 0, 0, 0, 0, 0, 0, 0));

        assert_eq!(addr6.segments(), [0, 0, 0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn sock6_new() {
        let mut socket6 = addr_hal::SocketAddrV6::<SocketV6Inner>::new(
            addr_hal::Ipv6Addr::<IpAddressV6Inner>::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1),
            8080,
            0,
            0,
        );
        assert_eq!("[2001:db8::1]:8080".parse(), Ok(socket6));

        assert_eq!(
            socket6.ip(),
            &addr_hal::Ipv6Addr::<IpAddressV6Inner>::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)
        );

        socket6.set_ip(addr_hal::Ipv6Addr::<IpAddressV6Inner>::new(
            0, 0, 0, 0, 0, 0, 0, 1,
        ));
        assert_eq!(
            socket6.ip(),
            &addr_hal::Ipv6Addr::<IpAddressV6Inner>::new(0, 0, 0, 0, 0, 0, 0, 1)
        );

        assert_eq!(socket6.port(), 8080);

        socket6.set_port(5656);
        assert_eq!(socket6.port(), 5656);

        assert_eq!(socket6.flowinfo(), 0);

        socket6.set_flowinfo(10);
        assert_eq!(socket6.flowinfo(), 10);

        assert_eq!(socket6.scope_id(), 0);

        socket6.set_scope_id(56);
        assert_eq!(socket6.scope_id(), 56);
    }
}
