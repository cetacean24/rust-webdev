use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// for whatever reason, Rust doesn't throw a fit if you skip out on the "," of a {} block in a match statement
// if a block isn't used and is a single line, Rust loses its mind... one bad comma

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // buffer to hold incoming data
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // connection closed (client disconnected)
                println!("Connection closed by client");
                break;
            }
            Ok(n) => {
                // echo the received data back to the client
                println!("Message recieved from client: {} bytes", n);
                if let Err(e) = stream.write_all(&buffer[..n]) {
                    eprintln!("Failed to write to stream: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                break;
            }
        }
    }
}
// server reads data from the client into a 1024 byte buffer using stream.read(&mut buffer)
// this occurs in a loop, meaning the server is constantly trying to read data from the stream (client)
// it tries to read the data from the buffer and in order to handle the Result, a match statement is used
// if read returns Ok(0), the client has closed the connection and the loop is exited (break); means read() returned 0 bytes and client is done sending
// if data is received Ok(n), it is written (echoed) back to the client with stream.write_all(&buffer[..n]) (exactly what was sent)
// Ok(n) means n number of bytes were sucessfully read, so stream.write_all() the buffer slice (thats meaningful) from index 0 to (not including) n -> [..n]
// when slicing, the upper bound is exclusive (not included) -> buffer = [0, 1, 2, 3] -> slice = &buffer[..3] -> includes elements 0, 1, 2 
// n and e in Err() and Ok() are declared in the match pattern, and the actual values of Ok and Err are bound to them
// errors during reading/writing are pinted to stderr (eprintln!()); the loop will break to close the connection
// NOTE: if let Err(e) = stream.write_all(&buffer[..n]) is shorthand; if the result is Err, grab the Err and run this block; otherwise ignore it
// specifically, if stream.write_all() fails (Err), eprint "Failed to write to stream: {e}", otherwise, if it succeeds, do nothing
// if let is used if only one variant of Result (Ok or Err) is important/vital; let is destructuring Err(e) so that e is available inside the following block
// write_all() is called first; if Err, run code underneath, if Ok, do nothing and continue

/* IDENTICAL TO:
match stream.write_all(&buffer[..n]) {
    Ok(_) => {},
    Err(e) => {
        eprintln!("Failed to write to stream: {}", e);
        break;
    }
} */

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on 127.0.0.1:8080");
    // bind the server to localhost:8080; this creates a TCP socket that listens for incoming connections
    // the ? at the end of TcpListener declaration is error propagation operator; shorthand for match statement handling Result
    
    /* IDENTICAL TO:
    let listener = match TcpListener::bind("127.0.0.1:8080") {
    Ok(l) => l,
    Err(e) => return Err(e),
    }; */

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // spawn a new thread for each client
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
    // accepts incoming connections; listener.incoming() returns an iterator wrapped in Result because it could fail
    // basically, the raw connection is wrapped in a result, and the iterator goes through each of these wrapped connections, checking if one errors
    // for each stream that is Ok, a thread is started; a closure (|| {}) is passed to that thread (a chunk of code for the new thread to run)
    // the || {} syntax is a closure; it is how an inline function is defined; tells Rust to start a new thread and run the block of code handle_client(stream) inside of it
    // closures can capture variables; stream is captured and passed into the new thread
    // by default, Rust tries to move captured variables into the closure (in case the original variable moves out of scope)
    // this is why move is included in front of || (it still runs if omitted in this example); it's needed to move ownership of stream into the thread

}