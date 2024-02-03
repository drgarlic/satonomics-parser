use bincode::{Decode, Encode};

// https://unchained.com/blog/bitcoin-address-types-compared/
#[derive(Debug, Encode, Decode, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum RawAddressType {
    Empty,
    #[default]
    Unknown,
    MultiSig,
    P2PK,
    P2PKH,
    P2SH,
    P2WPKH,
    P2WSH,
    P2TR,
}
