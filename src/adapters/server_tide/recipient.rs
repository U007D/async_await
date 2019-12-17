#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Recipient {
    Server,
    Client(u8),
}
