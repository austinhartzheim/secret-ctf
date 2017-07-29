use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::io::Write;

extern crate mio;
use mio::*;
use mio::net::{UdpSocket, TcpListener};

mod state;
use state::{KnockResult, PortKnockingState};

mod connections;
use connections::{ConnectionManager, ConnectionType};

const BASE_PORT: u16 = 4000;
const NUM_PORTS: u16 = 1000;

const FLAG: &str = "flag_professional_port_knocker\n";

const TELNET_PORT: u16 = 2323;

fn set_up_sockets(connection_manager: &mut ConnectionManager) {
    let bind_addr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

    // Create UDP knock-listener sockets
    for i in 0..NUM_PORTS {
        let addr = SocketAddr::new(bind_addr, BASE_PORT + i);
        let socket = UdpSocket::bind(&addr).unwrap();
        connection_manager.add_connection(ConnectionType::UdpKnockListener(socket, BASE_PORT + i),
                                          Ready::readable(),
                                          PollOpt::level());
    }

    // Create telnet listener socket
    let telnet_listener = TcpListener::bind(&SocketAddr::new(bind_addr, TELNET_PORT)).unwrap();
    connection_manager.add_connection(ConnectionType::TcpTelnetListener(telnet_listener),
                                      Ready::readable(),
                                      PollOpt::level());
}

fn main() {
    // Set up mio event tracking
    let mut events = Events::with_capacity(1024);

    // Set up internal state tracking
    let mut state = PortKnockingState::new();
    let mut connection_manager = ConnectionManager::new();

    // Create sockets
    set_up_sockets(&mut connection_manager);

    // Begin looping over mio events
    loop {
        connection_manager.poll.poll(&mut events, None).unwrap();
        for event in events.iter() {
            let cloned_connection: ConnectionType;

            // We need to clone the connection from the connection_manager to avoid
            // two mutable borrows of the connection_manager. So we create a new
            // scope to perform the clone.
            {
                let connection = connection_manager.get_connection(event.token());
                if connection.is_none() {
                    println!("Error: Encountered an unregistered connection.");
                    continue; // This connection wasn't registered. This shouldn't happen.
                }
                cloned_connection = connection.unwrap().try_clone().unwrap();
            }

            match cloned_connection {
                ConnectionType::UdpKnockListener(socket, port) => {
                    let mut buffer: [u8; 1024] = [0; 1024];
                    let (_, addr) = socket.recv_from(&mut buffer).unwrap();
                    println!("Got knock from {} on port {}", addr, port);
                    state.knock(addr.ip(), port);
                }
                ConnectionType::TcpTelnetListener(socket) => {
                    match socket.accept() {
                        Ok((telnet_socket, addr)) => {
                            println!("Got a telnet connection from {}", addr);
                            if let KnockResult::Success = state.check(addr.ip()) {
                                // Successful knock received from this IP. Accept their telnet
                                // connection and reset the state for their IP.
                                connection_manager.add_connection(ConnectionType::TcpTelnetSession(telnet_socket), Ready::writable(), PollOpt::oneshot());
                                state.reset(addr.ip());
                            }
                        }
                        Err(_) => {
                            // Accepting the socket failed. Oh well.
                            println!("Accepting socket failed.")
                        }
                    }
                }
                ConnectionType::TcpTelnetSession(mut socket) => {
                    socket.write_all(FLAG.as_bytes()).unwrap();
                    connection_manager.remove_connection(event.token());
                }
            }
        }
    }
}
