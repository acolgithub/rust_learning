// for listener and reader
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};

fn main() {

    // create listener which monitors local address 127.0.0.1:7878
    // for incoming TCP streams
    // bind is used to bind to port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();  // 7878 is a port that usually doesn't accept HTTP but is rust typed on a telephone

    // when listener gets an incoming stream print connection established
    for stream in listener.incoming() {
        let stream = stream.unwrap();  // unwrap is used to stop program if error occurs

        // separate function which handles processing connections
        handle_connection(stream);
    }

    
}


// function to read data being sent from browser
fn handle_connection(mut stream: TcpStream) {

    // add buffering by managing calls to std::io::Read trait method
    let buf_reader = BufReader::new(&mut stream);

    // collect the lines of request the browser sends to our server
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())        // get each string
        .take_while(|line| !line.is_empty())  // detect end of stream by second new line
        .collect();

    // create response to request when successful
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    // convert response to bytes and write to stream
    stream.write_all(response.as_bytes()).unwrap();
}
