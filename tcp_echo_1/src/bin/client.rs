use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // connect to the server at localhost:8080 via TCP
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    // if server is not running, this will fail with connection refused error

    // define the message to be echoed/sent
    let message = "I connected to the server!";

    // send the message to the server
    stream.write_all(message.as_bytes())?;
    // once again, handles errors with error propagation operator (?)

    // buffer to hold the server's response
    let mut buffer = [0; 1024];

    // client reads server's repsonse (echo of the original message)
    let n = stream.read(&mut buffer)?;

    // print the echoed response with slicing and String::from_utf8_lossy()
    // that function converts bytes into string, handling invalid utf8 input gracefully
    // honestly, I don't know how to input invalid utf8 but the functions cool at least
    println!("Message received: {}", String::from_utf8_lossy(&buffer[..n]));
    Ok(())
}