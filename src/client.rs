use std::io::{stdin, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

const SERVER_ADDR: &str = "87.242.87.212:5000";

fn main() {
    let mut stream = TcpStream::connect(SERVER_ADDR).unwrap();

    println!("connected to server");

    let read_stream = stream.try_clone().unwrap();

    thread::spawn(move || {
        let reader = BufReader::new(read_stream);

        for line in reader.lines() {
            match line {
                Ok(msg) => {
                    println!("getting msg: {}", msg);
                }

                Err(_) => {
                    println!("disconnected");
                    break;
                }
            }
        }
    });

    loop {
        println!("Write your message:");
        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        match stream.write_all(input.as_bytes()) {
            Ok(_) => {
                println!("sent");
            }

            Err(e) => {
                println!("send error: {}", e);
                break;
            }
        }

        stream.flush().unwrap();
    }
}
