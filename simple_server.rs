use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
use std::thread::Thread;

fn handle_request( mut stream: TcpStream ) {
    stream.write( b"HTTP/1.1 200 OK\n" );
    stream.write( b"Date: Wed, 30 Jul 2014 23:03:54 GMT\n" );
    stream.write( b"Content-Type: text/plain; charset=utf-8\n" );
    stream.write( b"Content-Length: 23\n\n");
    stream.write( b"Rust server is working!\n\n" );
}

fn main() {
    let listener = TcpListener::bind( "127.0.0.1:8080" );
    let mut acceptor = listener.listen();

    for stream in acceptor.incoming() {
        match stream {
            Err(e) => { println!( "Error: {}", e )  }
            Ok(stream) => {
                Thread::spawn(move || {
                    handle_request( stream )
                });
            }
        }
    }

    drop( acceptor );
}
