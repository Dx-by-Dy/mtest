use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

const SERVER_ADDR: &str = "0.0.0.0:5000";
const LOGGER_ADDR: &str = "logger:6000";

type Clients = Arc<Mutex<Vec<TcpStream>>>;

fn broadcast(msg: &str, clients: &Clients) {
    println!("broadcasting");
    let mut clients = clients.lock().unwrap();

    clients.retain_mut(|client| match client.write_all(msg.as_bytes()) {
        Ok(_) => true,

        Err(e) => {
            println!("broadcast error: {}", e);
            false
        }
    });
}

fn log_message(msg: &str, logger: &Arc<Mutex<Option<TcpStream>>>) {
    if let Some(stream) = logger.lock().unwrap().as_mut() {
        let _ = stream.write_all(msg.as_bytes());
    }
}

fn handle_client(mut stream: TcpStream, clients: Clients, logger: Arc<Mutex<Option<TcpStream>>>) {
    {
        let mut locked = clients.lock().unwrap();
        locked.push(stream.try_clone().unwrap());
    }

    let mut buffer = [0u8; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("client disconnected");
                break;
            }

            Ok(n) => {
                let msg = String::from_utf8_lossy(&buffer[..n]);

                println!("msg: {}", msg);

                broadcast(&msg, &clients);
                log_message(&msg, &logger);
            }

            Err(e) => {
                println!("read error: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind(SERVER_ADDR).unwrap();

    println!("server started on {}", SERVER_ADDR);

    let logger = TcpStream::connect(LOGGER_ADDR).ok();

    if logger.is_some() {
        println!("logger connected");
    } else {
        println!("logger not connected");
    }

    let logger = Arc::new(Mutex::new(logger));

    let clients: Clients = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new client: {:?}", stream.peer_addr());

                let clients = clients.clone();
                let logger = logger.clone();

                thread::spawn(move || {
                    handle_client(stream, clients, logger);
                });
            }

            Err(e) => {
                println!("accept error: {}", e);
            }
        }
    }
}
