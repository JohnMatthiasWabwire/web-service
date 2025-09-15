#![allow(dead_code)]

use std::net::{IpAddr, SocketAddr, TcpListener, UdpSocket};

// Hypertext Transfer Protocol Connection Definition
pub struct HttpConnection {
    pub datagram_socket: UdpSocket,
    pub ip_address: IpAddr,
    pub socket_address: SocketAddr,
    pub transmission_listener: TcpListener,
}

// Hypertext Transfor Protocol Connection
pub fn protocol_connection(connection: &HttpConnection) -> &HttpConnection {
    return connection;
}
