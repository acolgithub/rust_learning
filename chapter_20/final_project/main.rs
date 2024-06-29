// for listener and reader
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration
};

fn main() {

    // create listener which monitors local address 127.0.0.1:7878
    // for incoming TCP streams
    // bind is used to bind to port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();  // 7878 is a port that usually doesn't accept HTTP but is rust typed on a telephone

    // create thread pool
    let pool = ThreadPool::new(5);

    // when listener gets an incoming stream print connection established
    for stream in listener.incoming() {
        let stream = stream.unwrap();  // unwrap is used to stop program if error occurs

        //spawn new thread from pool to handle request
        pool.execute(|| {
            // separate function which handles processing connections
            handle_connection(stream);
        });
    }

    
}


// function to read data being sent from browser
fn handle_connection(mut stream: TcpStream) {

    // add buffering by managing calls to std::io::Read trait method
    let buf_reader = BufReader::new(&mut stream);

    // read only first line of HTTP request by calling next
    // use first unwrap to handle option in case iterator has no items
    // use second unwrap to handle the result
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // get status and appropriate file
    let (status_line, filename) = match &request_line[..] {
        
        // match if request for /
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),

        // match if request for /sleep
        "GET /sleep HTTP/1.1" => {
            // sleep for 5 seconds before rendering
            thread::sleep(Duration::from_secs(5));

            // return 200 message
            ("HTTP/1.1 200 OK", "hello.html")
        },
        
        // all other requests get 404
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };


    if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // get contents from html file
    let contents = fs::read_to_string(filename).unwrap();

    // get length of contents
    let length = contents.len();

    // create response to request when successful
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    // convert response to bytes and write to stream
        stream.write_all(response.as_bytes()).unwrap();
}
