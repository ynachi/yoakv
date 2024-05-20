use std::io;

use mio::{Events, Interest, Poll, Token};
use mio::net::TcpListener;

// tokens are used to identify event, socket tuple
const SERVER: Token = Token(0);
const MAX_CONCURRENT_CLIENTS: usize = 128;

const DATA: &[u8] = b"Hello World!\n";

pub fn server() -> io::Result<()> {
    tracing_subscriber::fmt::try_init().expect("unable to initialize the logger");
    // create a pool instance
    let mut poll = Poll::new()?;
    // create storage for events
    let mut events = Events::with_capacity(MAX_CONCURRENT_CLIENTS);

    // set up TCP socket
    let addr = "127.0.0.1:8081".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;

    // Register the server. It listens to read events only because all it wants it to be aware
    // of somebody trying to connect.
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    Ok(())
}
