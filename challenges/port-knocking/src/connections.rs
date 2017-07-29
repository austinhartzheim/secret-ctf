use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use std::io;

use mio::net::{TcpListener, TcpStream, UdpSocket};
use mio::event::Evented;
use mio::{Poll, Token};

pub enum ConnectionType {
    UdpKnockListener(UdpSocket, u16),
    TcpTelnetListener(TcpListener),
    TcpTelnetSession(TcpStream),
}

impl ConnectionType {
    pub fn try_clone(&self) -> io::Result<ConnectionType> {
        match *self {
            ConnectionType::UdpKnockListener(ref socket, port) => {
                let new_socket = socket.try_clone()?;
                Ok(ConnectionType::UdpKnockListener(new_socket, port))
            }
            ConnectionType::TcpTelnetListener(ref socket) => {
                let new_socket = socket.try_clone()?;
                Ok(ConnectionType::TcpTelnetListener(new_socket))
            }
            ConnectionType::TcpTelnetSession(ref socket) => {
                let new_socket = socket.try_clone()?;
                Ok(ConnectionType::TcpTelnetSession(new_socket))
            }
        }
    }
}

struct Connection {
    connection: ConnectionType,
    last_access: SystemTime,
}

pub struct ConnectionManager {
    connections: HashMap<Token, Connection>,
    timeout_duration: Duration,
    next_token: usize,
}

impl ConnectionManager {
    pub fn new() -> Self {
        ConnectionManager {
            connections: HashMap::new(),
            timeout_duration: Duration::from_secs(60),
            next_token: 0,
        }
    }

    pub fn add_connection(self: &mut Self, token: Token, connection: ConnectionType) {
        self.connections
            .insert(token,
                    Connection {
                        connection: connection,
                        last_access: SystemTime::now(),
                    });
    }

    /// Look up a ConnectionType object based on a mio Token.
    pub fn get_connection(self: &mut Self, token: Token) -> Option<&ConnectionType> {
        match self.connections.get_mut(&token) {
            Some(connection) => Some(&connection.connection),
            None => None,
        }
    }

    pub fn remove_connection(self: &mut Self, token: Token, poll: &Poll) {
        match self.connections.remove(&token) {
            Some(connection) => {
                match connection.connection {
                    ConnectionType::TcpTelnetListener(socket) => {
                        socket.deregister(&poll).unwrap();
                    }
                    ConnectionType::TcpTelnetSession(socket) => {
                        socket.deregister(&poll).unwrap();
                    }
                    ConnectionType::UdpKnockListener(socket, _) => {
                        socket.deregister(&poll).unwrap();
                    }
                }
            }
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {}
