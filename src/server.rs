use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

const SERVER_ADDR: &str = "0.0.0.0:5000";
const LOGGER_ADDR: &str = "logger:6000";

type Clients = Arc<Mutex<Vec<TcpStream>>>;

fn broadcast(msg: &str, clients: &Clients) {
    let mut clients = clients.lock().unwrap();

    clients.retain_mut(|client| client.write_all(msg.as_bytes()).is_ok());
}

fn log_message(msg: &str, logger: &Arc<Mutex<Option<TcpStream>>>) {
    if let Some(stream) = logger.lock().unwrap().as_mut() {
        let _ = stream.write_all(msg.as_bytes());
    }
}

fn handle_client(stream: TcpStream, clients: Clients, logger: Arc<Mutex<Option<TcpStream>>>) {
    let reader_stream = stream.try_clone().unwrap();

    {
        let mut locked = clients.lock().unwrap();
        locked.push(stream.try_clone().unwrap());
    }

    let reader = BufReader::new(reader_stream);

    for line in reader.lines() {
        match line {
            Ok(msg) => {
                let full = format!("{msg}\n");

                println!("msg: {}", msg);

                broadcast(&full, &clients);
                log_message(&full, &logger);
            }

            Err(_) => break,
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
