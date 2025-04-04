// subscriber.rs
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

fn main() {
    // connect to publisher
    let stream = TcpStream::connect("127.0.0.1:5151").expect("Failed to connect to publisher");
    println!("Connected to publisher at tcp://127.0.0.1:5151");

    // wrap the stream in a reader for line-based reading
    let reader = BufReader::new(stream);

    // read messages continuously
    for line in reader.lines() {
        match line {
            Ok(message) => println!("Received: {}", message),
            Err(_) => {
                println!("Publisher closed connection.");
                break;
            }
        }
    }
}
