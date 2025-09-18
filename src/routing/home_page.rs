use std::{
    fs::File,
    io::{BufReader, Error, Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    primitive::char,
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
    http_status_codes::{HTTP_OK, HTTP_TWO_HUNDRED},
    http_versions::HTTP_VERSION_ONE,
};

// Web Application Home Page Route
pub fn home_route(transmission_listener: Result<TcpListener, Error>) -> () {
    match transmission_listener {
        Ok(listener) => {
            for transmission_stream in listener.incoming() {
                let mut stream: TcpStream = transmission_stream.unwrap();
                let source_path: PathBuf = PathBuf::from("./web/build/main.js");
                let source_file: Result<File, Error> = File::open(source_path);
                let mut file_buffer: String = String::new();
                let file_characters: Vec<char> = file_buffer.chars().collect();

                match source_file {
                    Ok(file) => {
                        let mut buffered_reader: BufReader<&File> = BufReader::new(&file);

                        buffered_reader.read_to_string(&mut file_buffer).unwrap();
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
                            "{}: application/json;charset=UTF-8,text/plain;charset=UTF-8",
                            HTTP_ACCEPT
                        )
                        .unwrap();
                        writeln!(
                            stream,
                            "{}: text/javascript;charset=UTF-8",
                            HTTP_CONTENT_TYPE
                        )
                        .unwrap();
                        writeln!(stream, "{}: {}", HTTP_CONTENT_LENGTH, file_characters.len())
                            .unwrap();
                        writeln!(stream, "{}: htnet/0.2.0", HTTP_SERVER).unwrap();
                        writeln!(stream, "").unwrap();
                        writeln!(stream, "{}", file_buffer).unwrap();
                    }
                    Err(error) => {
                        eprintln!("Error Opening File: {}", error);
                        exit(1);
                    }
                };
            }
        }
        Err(error) => {
            eprintln!("Error Initializing Transmission Listener: {}", error);
            exit(1);
        }
    };

    return ();
}
