extern crate mio;
use mio::*;
use std::net::SocketAddr;
use mio::tcp::*;

struct WebSocketServer {
    socket: TcpListener,
    clients: HashMap<Token, TcpStream>,
    token_counter: usize
}

impl Handler for WebSocketServer {
    // Traits can have useful default implementations, so in fact the handler
    // interface requires us to provide only two things: concrete types for
    // timeouts and messages.
    // We're not ready to cover these fancy details, and we wouldn't get to them
    // anytime soon, so let's get along with the defaults from the mio examples:
    type Timeout = usize;
    type Message = ();

}

fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    let mut handler = WebSocketServer;
    event_loop.run(&mut handler).unwrap();

    let address = "0.0.0.0:10000".parse::<SocketAddr>().unwrap();
    let server_socket = TcpListener::bind(&address).unwrap();
    event_loop.register(&server_socket,
                    Token(0),
                    EventSet::readable(),
                    PollOpt::edge()).unwrap();

    println!("Hello, world!");
}
