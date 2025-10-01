use std::{
    fs::File,
    io::{BufReader, Error, Read, StdoutLock, Write, stdout},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    process::exit,
    result::{
        Result,
        Result::{Err, Ok},
    },
    string::String,
};

use crate::hypertext_transfer::{
    http_headers::{
        HTTP_ACCEPT, HTTP_CONNECTION, HTTP_CONTENT_LENGTH, HTTP_CONTENT_TYPE, HTTP_HOST,
        HTTP_SERVER,
    },
    http_mime_types::{HTTP_HTML_MIME_TYPE, HTTP_JSON_MIME_TYPE, HTTP_PLAIN_MIME_TYPE},
    http_status_codes::{HTTP_OK, HTTP_TWO_HUNDRED},
    http_versions::HTTP_VERSION_ONE,
};

// Web Application Home Page Route
pub fn home_route(transmission_listener: Result<TcpListener, Error>) -> () {
    match transmission_listener {
        Ok(listener) => {
            for transmission_stream in listener.incoming() {
                let mut stream: TcpStream = transmission_stream.unwrap();
                let source_path: PathBuf = PathBuf::from("./web/src/main.html");
                let source_file: File = File::open(source_path).unwrap();
                let mut source_buffer: String = String::new();
                let mut source_reader: BufReader<&File> = BufReader::new(&source_file);

                source_reader.read_to_string(&mut source_buffer).unwrap();
                writeln!(
                    stream,
                    "{} {} {}",
                    HTTP_VERSION_ONE, HTTP_TWO_HUNDRED, HTTP_OK
                )
                .unwrap();
                writeln!(stream, "{}: localhost:7878", HTTP_HOST).unwrap();
                writeln!(stream, "{}: keep-alive", HTTP_CONNECTION).unwrap();
                writeln!(
                    stream,
                    "{}: {},{}",
                    HTTP_ACCEPT, HTTP_JSON_MIME_TYPE, HTTP_PLAIN_MIME_TYPE
                )
                .unwrap();
                writeln!(stream, "{}: {}", HTTP_CONTENT_TYPE, HTTP_HTML_MIME_TYPE).unwrap();
                writeln!(stream, "{}: {}", HTTP_CONTENT_LENGTH, source_buffer.len()).unwrap();
                writeln!(stream, "{}: htnet/0.2.0", HTTP_SERVER).unwrap();
                writeln!(stream, "").unwrap();
                writeln!(stream, "{}", source_buffer).unwrap();
            }
        }
        Err(error) => {
            eprintln!("Error Initializing Transmission Listener: {}", error);
            exit(1);
        }
    };

    return ();
}

// Hypertext Transfer Protocol Connection Management
pub fn manage_connection(transmission_listener: Result<TcpListener, Error>) -> () {
    match transmission_listener {
        Ok(listener) => {
            for transmission_stream in listener.incoming() {
                let stream: TcpStream = transmission_stream.unwrap();
                let mut standard_output: StdoutLock = stdout().lock();
                let mut buffered_reader: BufReader<&TcpStream> = BufReader::new(&stream);
                let mut stream_buffer: String = String::new();

                buffered_reader.read_to_string(&mut stream_buffer).unwrap();
                writeln!(standard_output, "Hypertext Tranfer Protocol Request: ").unwrap();
                writeln!(standard_output, "").unwrap();
                writeln!(standard_output, "{}", stream_buffer).unwrap();
            }
        }
        Err(error) => {
            eprintln!("Error Initializing Transmission Listener: {}", error);
            exit(1);
        }
    };

    return ();
}
