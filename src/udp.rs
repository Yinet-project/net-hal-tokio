use addr_hal::SocketAddr;   
use async_trait::async_trait;
use net_hal::udp::{UdpServer, UdpSocket};

//use tokio::net::UdpSocket;

use crate::addr;

#[derive(Debug)]
pub struct TokioUdpSocket {
    inner: tokio::net::UdpSocket,
}

#[async_trait]
impl UdpSocket for TokioUdpSocket {
    type SA4 = addr::SocketV4Inner;
    type SA6 = addr::SocketV6Inner;
    type Error = tokio::io::Error;
    async fn connect(&self, addr: SocketAddr<Self::SA4, Self::SA6>) -> Result<(), Self::Error> {
        //    let mut a = match addr.to_socket_addrs(){
        //        Ok(s) => s,
        //        Err(error) => panic!("to socket addrs return addr error:{:?}",error),
        //    };

        match addr {
            SocketAddr::V4(v) => self.inner.connect(v.inner.inner).await,
            SocketAddr::V6(v) => self.inner.connect(v.inner.inner).await,
        }
    }

    async fn send(&mut self, buffer: &[u8]) -> Result<usize, Self::Error> {
        self.inner.send(buffer).await
    }

    async fn recv(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        self.inner.recv(buffer).await
    }
}

#[derive(Debug)]
pub struct TokioUdpServer {}

#[async_trait]
impl UdpServer for TokioUdpServer {
    type SA4 = addr::SocketV4Inner;
    type SA6 = addr::SocketV6Inner;
    type Error = tokio::io::Error;
    type BindResult = TokioUdpSocket;

    async fn bind(addr: SocketAddr<Self::SA4, Self::SA6>) -> Result<Self::BindResult, Self::Error> {
        let r = match addr {
            SocketAddr::V4(v) => tokio::net::UdpSocket::bind(v.inner.inner).await,
            SocketAddr::V6(v) => tokio::net::UdpSocket::bind(v.inner.inner).await,
        };
        let s = TokioUdpSocket { inner: r.unwrap() };
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server() {
        let mut server = match TokioUdpServer::bind(SocketAddr::from(([127, 0, 0, 1], 3401))).await {
            Ok(s) => s,
            Err(error) => panic!("couldn't bind to address{:?}", error),
        };

        let mut buf = [0;10];
        let _buf_size = match server.recv(&mut buf).await {
            Ok(received) => println!("received {} bytes {:?}", received, &buf[..received]),
            Err(e) => panic!("recv function failed: {:?}", e),
        };
    }

    #[tokio::test]
    async fn test_sock() {
        let mut sock = match TokioUdpServer::bind(SocketAddr::from(([127, 0, 0, 1], 3400))).await {
            Ok(s) => s,
            Err(error) => panic!("couldn't bind to address{:?}", error),
        };

        match sock.connect(SocketAddr::from(([127, 0, 0, 1], 3401))).await {
            Ok(s) => s,
            Err(error) => panic!("couldn't connect to address{:?}", error),
        };

        let _buf_size = match sock.send(&[0, 1, 2]).await {
            Ok(s) => println!("send buffer size = {}",s),
            Err(error) => panic!("couldn't send to address{:?}", error),
        };
        //assert_eq!(buf_size,3);
    }
}
