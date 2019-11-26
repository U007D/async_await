use crate::server::Recipient;

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Msg {
    recipient: Recipient,
    data: u8,
}
