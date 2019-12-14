use crate::a_a_server_tide_adapter::Recipient;

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Msg {
    recipient: Recipient,
    data: u8,
}
