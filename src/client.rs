use std::io::{stdin, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

const SERVER_ADDR: &str = "server:5000";

fn main() {
    let mut stream = TcpStream::connect(SERVER_ADDR).unwrap();

    println!("connected to server");

    let read_stream = stream.try_clone().unwrap();

    thread::spawn(move || {
        let reader = BufReader::new(read_stream);

        for line in reader.lines() {
            match line {
                Ok(msg) => {
                    println!("{}", msg);
                }

                Err(_) => {
                    println!("disconnected");
                    break;
                }
            }
        }
    });

    loop {
        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        if stream.write_all(input.as_bytes()).is_err() {
            println!("send failed");
            break;
        }
    }
}
