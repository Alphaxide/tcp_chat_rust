use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let mut buffer = [0; 512];
    loop { 
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                let message = String::from_utf8_lossy(&buffer);
                println!("Received : {}", message);


                let mut clients = clients.lock().unwrap();
                for client in clients.iter_mut() {
                    if client.peer_addr().unwrap() !=  stream.peer_addr().unwrap() {
                        client.write_all(message.as_bytes()).unwrap();
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from client : {}", e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080");

    let clients = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) =>{
                print!("New client: {}", stream.peer_addr().unwrap());

                let clients_clone = Arc::clone(&clients);
                clients.lock().unwrap().push(stream.try_clone().unwrap());

                thread::spawn(move || {
                    handle_client(stream, clients_clone);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connections: {}", e);
            }
        }
    }
}