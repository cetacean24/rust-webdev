// publisher.rs
use std::io::Write; // provides write_all and flush methods to send data over network
use std::net::TcpListener; // allows creation of server that listens for incoming connections
use std::sync::{Arc, Mutex}; // provides tools for sharing data safely between threads
use std::thread; // allows code to be ran on separate threads
use std::time::Duration; // allows creation of timer (adds delay)

fn main() {
    // bind the publisher to TCP socket (port 5151)
    let listener = TcpListener::bind("127.0.0.1:5151").expect("TCPListener failed to bind to port 5151");
    // TcpListener::bind("127.0.0.1:5151") creates server that listens on 127.0.0.1:5151 (localhost on port 5151)
    // .expect() fires if binding fails; the program will crash with error message in (); used for handling errors when confident the code should work

    listener.set_nonblocking(true).expect("Cannot set non-blocking");
    // makes listener nonblocking; if nonblocking == false, program pauses until a connection is established
    // in this case, the publisher checks if there is a connection to listener, if not, it moves on without pausing the code (no delays)
    // .except() works the exact same way as the previous example

    println!("Publisher is running on tcp://127.0.0.1:5151"); // prints msg to confirm server is running

    // thread-safe (shared) list of connected subscribers
    let subscribers = Arc::new(Mutex::new(Vec::new()));
    // Vec::new() creates an empty vector (growable list) to store subscribers
    // each subscriber will be a TcpStream (a connection to a client)
    // Mutex::new() wraps vector in a mutex (mutual exclusion); ensures only one thread can access the vector at a time; prevents a race condition
    // Arc::new() wraps the mutex in an Arc (Atomic Reference Counting); allows sharing of subscriber list across threads (safely)
    // think of Arc as a way to give multiple threads a "shared pointer" to the same data

    // thread to accept new subscriber connections
    let subscribers_clone = Arc::clone(&subscribers);
    // creates a new reference to subscribers list; this reference will be given to the new thread
    // rust ownership rules require each thread to have its own reference to shared data

    // thread::spawn() starts a new thread; move || is a closure (mini function) that moves subscribers_clone into the new thread
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
    // listener.incoming() returns an iterator over incoming connections; each stream is a Result<TcpStream, Error>
    // match is rust's version of if-else for handling Result
    // Ok(stream) = subscriber is connected; a TcpStream is found (connection to that subscriber from publisher)
    // subscribers_clone.lock() locks the Mutex so vector can be accessed safely
    // .unwrap() gets the locked vector; if locking fails, .unwrap() will crash program
    // .push(stream) adds the new subscriber to the list; appends stream to back of subscribers_clone
    // Err(_) occurs if no connection/stream yet; waits 100ms with thread::sleep before checking again
    // this loop runs forever in its own thread; it is always ready to accept new subscribers

    // start publishing messages
    let mut counter = 0; // mutable variable starting at 0 to number messages

    // an infinite loop; the publisher will send messages forever
    loop {
        let message = format!("Update {}: Hello subscribers!\n", counter); // NOTE: the \n at the end ensures the message is properly read by subscriber
        println!("Publishing: {}", message.trim_end());
        // format! creates a string that places the varaible "counter" into the {}; \n is used for a newline (see NOTE)
        // message.trim_end() removes trailing newline in publisher console output

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
        // subscribers.lock().unwrap() locks subscriber list so it can be safely modified
        // .retain_mut() is special method that loops over vector, runs a closer (|subscriber|) for each item, and keeps only items that return true (deletes other items)
        // mut also means that each subscriber can be modified (written to)
        // write_all sends the message (as bytes) to subscribers; message.as_bytes converts the string to bytes; NOTE: networks deal in bytes, not strings
        // match statement handles the results; Ok(_) means writing succeeded; Err(e) means writing failed (the subscriber disconnected) -> prints error, returns false -> removing subscriber from list
        // subscriber.flush() forces message to send immediately (without this, it might buffer and delay)
        // if flush fails, print error and return false -> removing subscriber from list
        // if flush succeeds, return true -> keeps subscriber
        // if false is returned for any reason, the subscriber is dropped from the list -> this cleans up disconnected subscribers automatically

        counter += 1;
        thread::sleep(Duration::from_secs(1));
        // increments the counter by 1 and waits 1 second before sending the next message
    }
}

