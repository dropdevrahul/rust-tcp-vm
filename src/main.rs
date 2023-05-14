use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str;

mod stack;
pub use stack::stack::Stack;

struct HttpServer {
    host: String,
    port: i16,
    stack: Stack,
}

impl HttpServer {
    fn serve(&mut self) -> std::io::Result<()> {
        println!("Serving traffic at {}:{}", self.host, self.port);
        let addr = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(addr)?;

        for stream in listener.incoming() {
            self.handle(stream?).unwrap();
        }

        Ok(())
    }

    fn handle(&mut self, mut stream: TcpStream) -> Result<(), String> {
        let mut buff = vec![0; 128];
        let _n = match stream.read(&mut buff) {
           Ok(n) => n,
           Err(error) => panic!("error while reading from connection {}", error),
        };

        match self.stack.push(buff) {
            Ok(_) => (),
            Err(error) => return Err(error.to_string()),
        };


        let peek_value = match self.stack.peek() {
            Ok(n) => n,
            Err(error) => return Err(error),
        };

        let s = match str::from_utf8(&peek_value) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        println!("stack {}", s);

        stream.write(s.as_bytes()).unwrap();

        Ok(())
    }
}


fn main() {
    let mut server = HttpServer{
        host: String::from("127.0.0.1"),
        port: 8080,
        stack: Stack{
            top: -1,
            store: Vec::with_capacity(1000000),
        },
    };

    match server.serve() {
        Ok(v) => v,
        Err(e) => panic!("Error while handling request {}", e)
    };
}
