use std::fs::{create_dir_all, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

const LOGGER_ADDR: &str = "logger:6000";

fn main() {
    let listener = TcpListener::bind(LOGGER_ADDR).unwrap();

    println!("logger started on {}", LOGGER_ADDR);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("server connected");

                create_dir_all("/app/logs").unwrap();

                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("/app/logs/log.txt")
                    .unwrap();

                let reader = BufReader::new(stream);

                for line in reader.lines() {
                    match line {
                        Ok(msg) => {
                            println!("log: {}", msg);

                            writeln!(file, "{}", msg).unwrap();
                        }

                        Err(_) => {
                            println!("server disconnected");
                            break;
                        }
                    }
                }
            }

            Err(e) => {
                println!("logger error: {}", e);
            }
        }
    }
}
