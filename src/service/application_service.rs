use std::{
    io::{BufReader, Error, Read, StdoutLock, Write, stdout},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    process::exit,
    result::{
        Result,
        Result::{Err, Ok},
    },
};

use hyper::{body, server::conn::http1, service::service_fn};

use super::home_page::home_route;

// Application Service
pub fn web_service() -> () {
    let ip_address: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let socket_address: SocketAddr = SocketAddr::new(ip_address, 7878);
    let listener: Result<TcpListener, Error> = TcpListener::bind(socket_address);
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "Service Listening on Port: 7878").unwrap();
    home_route(listener);

    return ();
}
