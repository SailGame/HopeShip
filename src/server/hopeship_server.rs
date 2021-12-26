use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::collections::HashMap;

pub struct HopeShipServer {
    listen_addr : String,
    client_token: usize,
    connections: HashMap<usize, TcpStream>
}

const SERVER: Token = Token(0);

impl HopeShipServer {
    pub fn new(listen_addr: String, handler: Handler) -> HopeShipServer {
        return HopeShipServer {
            listen_addr: listen_addr,
            client_token: 1,
            connections: HashMap::new()
        };
    }

    pub fn start(&mut self) {
        let mut poll = Poll::new().expect("failed to create poll");
        let mut events = Events::with_capacity(128);

        let addr = self.listen_addr.parse().expect("failed to parse address");
        let mut server = TcpListener::bind(addr).expect("failed to bind tcp address");
        poll.registry()
                .register(&mut server, SERVER, Interest::READABLE).expect("failed to register server to poll");

        loop {
            poll.poll(&mut events, None).expect("failed to poll");

            for event in events.iter() {
                match event.token() {
                    SERVER => {
                        let mut connection = server.accept().unwrap().0;
                        poll.registry()
                            .register(&mut connection, Token(self.client_token), Interest::READABLE | Interest::WRITABLE).expect("failed to register conn to poll");
                        self.connections.insert(self.client_token, connection);
                        self.client_token = self.client_token + 1;
                    }
                    token => {
                        if event.is_writable() {
                            // We can (likely) write to the socket without blocking.
                        }
                        else if event.is_readable() {
                            // We can (likely) read from the socket without blocking.
                        }
                        else
                        {
                            self.connections.remove(&token.0);
                        }
                    }
                }
            }
        }
    }
}