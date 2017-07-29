use std::collections::HashMap;
use std::io;

use mio::net::{TcpListener, TcpStream, UdpSocket};
use mio::event::Evented;
use mio::*;

#[derive(Debug)]
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
}

pub struct ConnectionManager {
    connections: HashMap<Token, Connection>,
    next_token: usize,
    pub poll: Poll,
}

impl ConnectionManager {
    pub fn new() -> Self {
        ConnectionManager {
            connections: HashMap::new(),
            next_token: 0,
            poll: Poll::new().unwrap(),
        }
    }

    pub fn add_connection(self: &mut Self,
                          connection: ConnectionType,
                          ready: Ready,
                          pollopt: PollOpt) {
        let token = self.create_token();

        match connection {
            ConnectionType::TcpTelnetListener(ref socket) => {
                self.poll.register(socket, token, ready, pollopt).unwrap();
            }
            ConnectionType::TcpTelnetSession(ref socket) => {
                self.poll.register(socket, token, ready, pollopt).unwrap();
            }
            ConnectionType::UdpKnockListener(ref socket, _) => {
                self.poll.register(socket, token, ready, pollopt).unwrap();
            }
        }

        self.connections
            .insert(token, Connection { connection: connection });
    }

    /// Look up a ConnectionType object based on a mio Token.
    pub fn get_connection(self: &mut Self, token: Token) -> Option<&ConnectionType> {
        // TODO: We should be able to lift the mutable requirement if we add lifetimes.
        // This would allow us to take out the clone nastiness in main().
        match self.connections.get_mut(&token) {
            Some(connection) => Some(&connection.connection),
            None => None,
        }
    }

    pub fn remove_connection(self: &mut Self, token: Token) {
        if let Some(connection) = self.connections.remove(&token) {
            match connection.connection {
                ConnectionType::TcpTelnetListener(socket) => {
                    socket.deregister(&self.poll).unwrap();
                }
                ConnectionType::TcpTelnetSession(socket) => {
                    socket.deregister(&self.poll).unwrap();
                }
                ConnectionType::UdpKnockListener(socket, _) => {
                    socket.deregister(&self.poll).unwrap();
                }
            }
        }
    }

    pub fn create_token(self: &mut Self) -> Token {
        let token = Token(self.next_token);
        self.next_token += 1;
        token
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use mio::Token;
    use mio::net::UdpSocket;

    use ConnectionManager;
    use ConnectionType;

    #[test]
    fn test_get_connection_returns_none_when_no_connections_added() {
        let mut connection_manager = ConnectionManager::new();
        assert!(connection_manager.get_connection(Token(0)).is_none());
        assert!(connection_manager.get_connection(Token(1)).is_none());
    }

    #[test]
    fn test_retrevial_of_connection_by_token() {
        const TOKEN: Token = Token(12);
        const PORT: u16 = 60213;
        let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

        let mut connection_manager = ConnectionManager::new();
        let socket = UdpSocket::bind(&SocketAddr::new(addr, PORT)).unwrap();
        connection_manager.add_connection(TOKEN, ConnectionType::UdpKnockListener(socket, PORT));
        assert!(connection_manager.get_connection(TOKEN).is_some());
    }

    #[test]
    fn test_create_token_sequence() {
        let mut connection_manager = ConnectionManager::new();
        assert_eq!(connection_manager.create_token(), Token(0));
        assert_eq!(connection_manager.create_token(), Token(1));
        assert_eq!(connection_manager.create_token(), Token(2));
        assert_eq!(connection_manager.create_token(), Token(3));
    }
}
