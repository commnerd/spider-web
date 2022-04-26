use crate::connection::Connection;
use crate::thread::ThreadPool;
use config::Config;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process::Command;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct Server {
    config: Config,
    connections: Vec<Connection>,
}

impl Server {
    pub fn new(conf: Config) -> Self {
        Server{
            config: conf,
            connections: vec![],
        }
    }

    pub fn run(&self) {
        let config = &self.config;

        thread::spawn(|| runContainerd());
        runWebServer(config);
        
        println!("Shutting down.");
    }
}

fn runContainerd() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("dockerd")
                .output()
                .expect("failed to execute process")
    };
}

fn runWebServer(config: &Config) {
    let host = format!("127.0.0.1:{}", config.get_int("port").unwrap());
    let thread_count = config.get_int("threads").unwrap();
    let listener = TcpListener::bind(host).unwrap();
    let pool = ThreadPool::new(thread_count as usize);

    for stream in listener.incoming() {
        let src_dir = config.get_string("src_dir").unwrap();
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream, src_dir);
        });
    }
}

fn handle_connection<'reply>(mut stream: TcpStream, src_dir: String) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", format!("{}/{}", src_dir, "index.html"))
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", format!("{}/{}", src_dir, "index.html"))
    } else {
        ("HTTP/1.1 404 NOT FOUND", String::from("404.html"))
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}