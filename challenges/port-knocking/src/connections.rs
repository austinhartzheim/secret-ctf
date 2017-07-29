use std::collections::HashMap;
use std::io;

use mio::net::{TcpListener, TcpStream, UdpSocket};
use mio::event::Evented;
use mio::{Poll, Token};

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
}

impl ConnectionManager {
    pub fn new() -> Self {
        ConnectionManager { connections: HashMap::new() }
    }

    pub fn add_connection(self: &mut Self, token: Token, connection: ConnectionType) {
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

    pub fn remove_connection(self: &mut Self, token: Token, poll: &Poll) {
        if let Some(connection) = self.connections.remove(&token) {
            match connection.connection {
                ConnectionType::TcpTelnetListener(socket) => {
                    socket.deregister(poll).unwrap();
                }
                ConnectionType::TcpTelnetSession(socket) => {
                    socket.deregister(poll).unwrap();
                }
                ConnectionType::UdpKnockListener(socket, _) => {
                    socket.deregister(poll).unwrap();
                }
            }
        }
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
}
