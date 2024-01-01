use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum AddressKind {
    Unknown,
    MultiSig,
    P2PKH,
    P2SH,
    P2WPKH,
    P2WSH,
    P2TR,
}
