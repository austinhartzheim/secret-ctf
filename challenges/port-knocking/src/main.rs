use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::collections::HashMap;

extern crate mio;
use mio::*;
use mio::net::{UdpSocket, TcpListener, TcpStream};

mod state;
use state::{KnockResult, PortKnockingState};

const BASE_PORT: u16 = 4000;
const NUM_PORTS: u16 = 100;
const TELNET_TOKEN_NUM: usize = (NUM_PORTS + 1) as usize;

const TELNET_PORT: u16 = 2323;

fn main() {
    let bind_addr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(1024);
    let mut state = PortKnockingState::new();

    // Create UDP listener sockets
    let mut sockets: Vec<UdpSocket> = vec![];
    for i in 0..NUM_PORTS {
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

    // Create telnet listener socket
    let telnet_listener = TcpListener::bind(&SocketAddr::new(bind_addr, TELNET_PORT)).unwrap();
    poll.register(
        &telnet_listener,
        Token(TELNET_TOKEN_NUM),
        Ready::readable(),
        PollOpt::edge(),
    );

    // Set up token tracking
    let mut next_token = TELNET_TOKEN_NUM + 1;

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                Token(TELNET_TOKEN_NUM) => {
                    // TODO: how can we reject bad connections without accepting them first?
                    match telnet_listener.accept() {
                        Ok((socket, addr)) => {
                            println!("Got a telnet connection from: {}", addr);
                            // TODO: we may want to place a limit on the max number of open sockets
                            match state.check(addr.ip()) {
                                KnockResult::Success => {
                                    println!("Got a successful connection from {}", addr);
                                    /*let token = Token(next_token);
                                    next_token += 1;
                                    poll.register(
                                        &socket,
                                        token,
                                        Ready::writable(),
                                        PollOpt::edge(),
                                    ).unwrap();*/
                                    continue; // Not registering the socket will shut it down
                                }
                                _ => {
                                    // TODO: attempt to close socket
                                    // Not registering the socket will shut it down
                                }

                            }
                        }
                        Err(_) => {
                            // Accepting the socket failed. Oh well.
                            println!("Accepting socket failed.")
                        }
                    }
                }
                Token(i) => {
                    // Accept and drop the socket immediately, this will close
                    // the socket and notify the client of the EOF.
                    let mut buffer: [u8; 512] = [0; 512];
                    let (_, addr) = sockets[i].recv_from(&mut buffer).unwrap();
                    state.knock(addr.ip(), i as u16);
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
