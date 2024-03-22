use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use multithread::ThreadPool;

fn main() {
    // Listens to the incoming requests from a webbrowser from the specified website(here 127.0.0.1)
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4); 

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
            });
        
    }

}
// reads the request from site and write a response with the HTTP-Version Status-code Reason-Phrase CRLF
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    //Validating the request
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) =
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK","index.html")
        }
        else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_millis(5000));
            ("HTTP/1.1 200 OK","index.html")
        }
        else {
            ("HTTP/1.1 404 NOT FOUND","404.html")
        };

        let contents = fs::read_to_string(filename).unwrap();

        let response = format!(
            "{}\r\nContent-Lenght: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    
}