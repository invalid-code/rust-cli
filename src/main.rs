use std::env;
use std::io::{stdin, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
// use std::str::from_utf8;
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn server(port: &str) {
    let listening_addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(listening_addr).unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port {}", port);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}

fn client(addr: &str) {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 2000");

            let mut cmd = String::new();

            stdin().read_line(&mut cmd).expect("failed to read stdin");
            // let msg = b"Hello!";

            // stream.write(msg).unwrap();
            // println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    // if &data == msg {
                    //     println!("Reply is ok!");
                    // } else {
                    //     let text = from_utf8(&data).unwrap();
                    //     println!("Unexpected reply: {}", text);
                    // }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("help message"),
        2 => client(&args[1]),
        3 => server(&args[2]),
        _ => println!("..."),
    }
}
