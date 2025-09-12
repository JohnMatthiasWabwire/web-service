use std::{
    io::{BufReader, Error, Read, StdoutLock, Write, stdout},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    process::exit,
    result::{
        Result,
        Result::{Err, Ok},
    },
    thread,
};

use crate::routing::home_page::home_route;

// Hypertext Transfer Protocol Connection Management
pub fn manage_connection(transmission_listener: Result<TcpListener, Error>) -> () {
    match transmission_listener {
        Ok(listener) => {
            for transmission_stream in listener.incoming() {
                thread::spawn(|| {
                    let stream: TcpStream = transmission_stream.unwrap();
                    let mut standard_output: StdoutLock = stdout().lock();
                    let mut buffered_reader: BufReader<&TcpStream> = BufReader::new(&stream);
                    let mut stream_buffer: String = String::new();

                    buffered_reader.read_to_string(&mut stream_buffer).unwrap();
                    writeln!(standard_output, "Hypertext Tranfer Protocol Request: ").unwrap();
                    writeln!(standard_output, "").unwrap();
                    writeln!(standard_output, "{}", stream_buffer).unwrap();
                });
            }
        }
        Err(error) => {
            eprintln!("Error Initializing Transmission Listener: {}", error);
            exit(1);
        }
    };

    return ();
}

// Application Service
pub fn web_service() -> () {
    let ip_address: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let socket_address: SocketAddr = SocketAddr::new(ip_address, 7878);
    let listener: Result<TcpListener, Error> = TcpListener::bind(socket_address);
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "Service Listening on Port: 7878").unwrap();
    manage_connection(listener);

    return ();
}
