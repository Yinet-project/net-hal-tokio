use addr_hal::SocketAddr;
use net_hal::udp::{UdpServer, UdpSocket};

async fn echo_request<S: UdpSocket>(socket: &mut S) {
    match socket
        .connect(SocketAddr::from(([127, 0, 0, 1], 3400)))
        .await
    {
        Ok(s) => s,
        Err(_error) => panic!("couldn't connect to address"),
    };

    match socket.send(&[0, 1, 2]).await {
        Ok(s) => println!("send buffer size = {}", s),
        Err(_error) => panic!("couldn't send to address"),
    };
}

#[tokio::main]
async fn main() {
    use net_hal_tokio::udp::TokioUdpSocket;

    let mut handle = match TokioUdpSocket::bind(SocketAddr::from(([127, 0, 0, 1], 3401))).await {
        Ok(s) => s,
        Err(_error) => panic!("couldn't bind to address"),
    };
    echo_request(&mut handle).await;
}
