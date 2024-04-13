use savefile_derive::Savefile;

// https://unchained.com/blog/bitcoin-address-types-compared/
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Savefile)]
pub enum AddressType {
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
