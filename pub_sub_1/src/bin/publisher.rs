// publisher.rs
use std::io::Write;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // bind the publisher to TCP socket (port 5151)
    let listener = TcpListener::bind("127.0.0.1:5151").expect("TCPListener failed to bind to port 5151");
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    println!("Publisher is running on tcp://127.0.0.1:5151");

    // shared list of connected subscribers
    let subscribers = Arc::new(Mutex::new(Vec::new()));

    // thread to accept new subscriber connections
    let subscribers_clone = Arc::clone(&subscribers);
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New subscriber connected!");
                    subscribers_clone.lock().unwrap().push(stream);
                }
                Err(_) => {
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
    });

    // start publishing messages
    let mut counter = 0;
    loop {
        let message = format!("Update {}: Hello subscribers!\n", counter); // NOTE: the \n at the end ensures the message is properly read by sub
        println!("Publishing: {}", message.trim_end());

        // send the message to all subscribers
        let mut subscribers_guard = subscribers.lock().unwrap();
        subscribers_guard.retain_mut(|subscriber| {
            match subscriber.write_all(message.as_bytes()) {
                Ok(_) => {
                    // flush to force send
                    if let Err(e) = subscriber.flush() {
                        eprintln!("Flush failed: {}", e);
                        return false;
                    }
                    true
                }
                Err(e) => {
                    println!("Subscriber disconnected. Reason: {}", e);
                    false
                }
            }
        });

        counter += 1;
        thread::sleep(Duration::from_secs(1)); // wait for 1 second before sending the next update
    }
}

