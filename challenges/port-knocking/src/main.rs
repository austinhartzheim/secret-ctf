use std::net::{SocketAddr, Ipv4Addr, IpAddr};

extern crate mio;
use mio::*;
use mio::net::UdpSocket;

const BASE_PORT: u16 = 4000;

fn main() {
    let bind_addr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let poll = Poll::new().unwrap();

    let mut sockets: Vec<UdpSocket> = vec![];
    for i in 0..1000u16 {
        let addr = SocketAddr::new(bind_addr, BASE_PORT + i);
        let socket = UdpSocket::bind(&addr).unwrap();
        sockets.push(socket);
        poll.register(
            &sockets[i as usize],
            Token(i as usize),
            Ready::readable(),
            PollOpt::edge(),
        ).unwrap();
    }

    // Create storage for events
    let mut events = Events::with_capacity(1024);

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                Token(i) => {
                    // Accept and drop the socket immediately, this will close
                    // the socket and notify the client of the EOF.
                    let mut buffer: [u8; 512] = [0; 512];
                    let (_, addr) = sockets[i].recv_from(&mut buffer).unwrap();
                    println!(
                        "Got data from: {} on port {}.",
                        addr,
                        BASE_PORT + (i as u16)
                    );
                }
                _ => unreachable!(),
            }
        }
    }
}
