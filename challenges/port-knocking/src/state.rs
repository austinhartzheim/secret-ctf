use std::collections::HashMap;
use std::net::IpAddr;

const SEQUENCE_LENGTH: usize = 3;
const SEQUENCE: [u16; SEQUENCE_LENGTH] = [4002, 4841, 4219];

#[derive(Debug, PartialEq)]
pub enum KnockResult {
    Success, // The knock sequence was successful
    Unknown, // The ip address is not known
    Fail, // The current sequence is incorrect
}

pub struct PortKnockingState {
    knocks: HashMap<IpAddr, Vec<u16>>,
}

impl PortKnockingState {
    pub fn new() -> Self {
        PortKnockingState { knocks: HashMap::new() }
    }

    pub fn knock(self: &mut Self, srcaddr: IpAddr, port: u16) {
        let port_list = self.knocks.entry(srcaddr).or_insert(vec![]);
        port_list.push(port);
        if port_list.len() > SEQUENCE_LENGTH {
            port_list.remove(0);
        }
    }

    pub fn check(self: &Self, addr: IpAddr) -> KnockResult {
        match self.knocks.get(&addr) {
            Some(port_list) => {
                for (actual, expected) in port_list.iter().zip(SEQUENCE.iter()) {
                    if actual != expected {
                        return KnockResult::Fail;
                    }
                }
                KnockResult::Success
            }
            None => KnockResult::Unknown,
        }

    }
}

#[cfg(test)]
mod tests {
    use state::{KnockResult, PortKnockingState};
    use state::SEQUENCE;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_check_no_knocks_returns_unknown() {
        let state = PortKnockingState::new();
        assert_eq!(state.check(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
                   KnockResult::Unknown)
    }

    #[test]
    fn test_check_one_knock_returns_fail() {
        let mut state = PortKnockingState::new();
        let addr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        state.knock(addr, 1234);
        assert_eq!(state.check(addr), KnockResult::Fail);
    }

    #[test]
    fn test_check_correct_sequence_returns_success() {
        let mut state = PortKnockingState::new();
        let addr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        for port in SEQUENCE.iter() {
            state.knock(addr, *port);
        }
        assert_eq!(state.check(addr), KnockResult::Success);
    }
}
