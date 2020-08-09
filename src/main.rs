// use rust libs
use std::thread;
use std::net::{
    TcpListener,
    TcpStream,
    Shutdown
};
use std::io::{ Read, Write };

fn handle_client(mut stream: TcpStream) {
    // define buffer variable
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    // read up to 50 bytes and the while loop will continue if read result is OK
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything back
            stream.write(&data[0..size]).unwrap();
            // return true
            true
        },
        Err(err) => {
            println!("An error occurred, terminating connection with {} with error {}", stream.peer_addr().unwrap(), err);
            // shutdown stream connection
            stream.shutdown(Shutdown::Both).unwrap();
            // return false
            false
        }
    } {
        // no while body
    }
}

fn main() {
    // Creates a TCP listener bound to `0.0.0.0:3333`
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each request
    println!("Server listening on port 3333");
    // iterate on Tcp streams
    for stream in listener.incoming() {
        // match result
        match stream {
            Ok(stream) => {
                // Spin up another thread, so one thread per stream
                thread::spawn(move|| {
                    // thread code to handle client data when connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                // print error cause
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}