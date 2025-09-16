use std::{
    fs::File,
    io::{BufReader, Error, Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    primitive::usize,
    process::exit,
    result::{
        Result,
        Result::{Err, Ok},
    },
    string::String,
    thread,
};

use crate::hypertext_transfer::{
    http_headers::{HTTP_CONTENT_LENGTH, HTTP_CONTENT_TYPE},
    http_status_codes::{HTTP_OK, HTTP_TWO_HUNDRED},
    http_versions::HTTP_VERSION_ONE,
};

// Web Main Entry Point
pub fn web_main(mut transmission_stream: TcpStream) -> () {
    let source_path: PathBuf = PathBuf::from("./web/build/main.js");
    let source_file: Result<File, Error> = File::open(source_path);
    let mut file_buffer: String = String::new();
    let content_length: usize = file_buffer.len();

    match source_file {
        Ok(file) => {
            let mut buffered_reader: BufReader<&File> = BufReader::new(&file);

            buffered_reader.read_to_string(&mut file_buffer).unwrap();
            writeln!(
                transmission_stream,
                "{} {} {}",
                HTTP_VERSION_ONE, HTTP_TWO_HUNDRED, HTTP_OK
            )
            .unwrap();
            writeln!(
                transmission_stream,
                "{}: text/javascript; charset=utf-8",
                HTTP_CONTENT_TYPE
            )
            .unwrap();
            writeln!(
                transmission_stream,
                "{}: {:#?}",
                HTTP_CONTENT_LENGTH, content_length
            )
            .unwrap();
            writeln!(transmission_stream, "{}", file_buffer).unwrap();
        }
        Err(error) => {
            eprintln!("Error Opening File: {}", error);
            exit(1);
        }
    };
}

// Web Application Home Page Route
pub fn home_route(transmission_listener: Result<TcpListener, Error>) -> () {
    match transmission_listener {
        Ok(listener) => {
            for transmission_stream in listener.incoming() {
                let stream: TcpStream = transmission_stream.unwrap();
                thread::spawn(|| {
                    web_main(stream);
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
