// subscriber.rs
use std::io::{BufRead, BufReader}; // provides tools for reading data line by line efficiently
use std::net::TcpStream; // allows a connection to publisher

fn main() {
    // connect to publisher
    let stream = TcpStream::connect("127.0.0.1:5151").expect("Failed to connect to publisher");
    println!("Connected to publisher at tcp://127.0.0.1:5151");
    // connects to publisher at 127.0.0.1:5151; .expect() crashes with an error if connection fails (if publisher isn't running)
    // prints a message upon successful connection with publisher

    // wrap the stream in a reader for line-based reading
    let reader = BufReader::new(stream);
    // BufReader::new() wraps TcpStream in a BufReader; this buffers data (reads it in chunks) for efficiency, especially when reading lines

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
    // reader.lines() returns an iterator over lines from the stream; each line is a Result<String, Error>
    // for loop reads each line as it arrives; match statement handles the result
    // Ok(message) means a message was received -> prints it; Err(_) means no message (publisher shutdown) -> print notice then exit loop
}
