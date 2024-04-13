use bitcoin::{address::Payload, TxOut};
use bitcoin_hashes::{hash160, Hash};
use itertools::Itertools;

use crate::{
    bitcoin::multisig_addresses,
    parse::{U8x19, U8x31, SANAKIRJA_MAX_KEY_SIZE},
};

use super::{AddressType, Counter};

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Address {
    // https://mempool.space/tx/7bd54def72825008b4ca0f4aeff13e6be2c5fe0f23430629a9d484a1ac2a29b8
    Empty(u32),
    Unknown(u32),
    // https://mempool.space/tx/274f8be3b7b9b1a220285f5f71f61e2691dd04df9d69bb02a8b3b85f91fb1857
    MultiSig(Box<[u8]>),
    P2PK((u16, U8x19)),
    P2PKH((u16, U8x19)),
    P2SH((u16, U8x19)),
    P2WPKH((u16, U8x19)),
    P2WSH((u16, U8x31)),
    P2TR((u16, U8x31)),
}

impl Address {
    pub fn to_type(&self) -> AddressType {
        match self {
            Self::Empty(_) => AddressType::Empty,
            Self::Unknown(_) => AddressType::Unknown,
            Self::MultiSig(_) => AddressType::MultiSig,
            Self::P2PK(_) => AddressType::P2PK,
            Self::P2PKH(_) => AddressType::P2PKH,
            Self::P2SH(_) => AddressType::P2SH,
            Self::P2WPKH(_) => AddressType::P2WPKH,
            Self::P2WSH(_) => AddressType::P2WSH,
            Self::P2TR(_) => AddressType::P2TR,
        }
    }

    pub fn from(
        txout: &TxOut,
        unknown_addresses: &mut Counter,
        empty_addresses: &mut Counter,
    ) -> Self {
        let script = &txout.script_pubkey;

        match Payload::from_script(script) {
            Ok(payload) => {
                let (prefix, rest) = Self::split_slice(Self::payload_to_slice(&payload));

                if script.is_p2pkh() {
                    Self::P2PKH((prefix, rest.into()))
                } else if script.is_p2sh() {
                    Self::P2SH((prefix, rest.into()))
                } else if script.is_p2wpkh() {
                    Self::P2WPKH((prefix, rest.into()))
                } else if script.is_p2wsh() {
                    Self::P2WSH((prefix, rest.into()))
                } else if script.is_p2tr() {
                    Self::P2TR((prefix, rest.into()))
                } else {
                    // https://mempool.space/address/bc1zqyqs3juw9m
                    Self::new_unknown(unknown_addresses)
                }
            }
            Err(_) => {
                if script.is_p2pk() {
                    let pk = match script.as_bytes().len() {
                        67 => &script.as_bytes()[1..66],
                        35 => &script.as_bytes()[1..34],
                        _ => unreachable!(),
                    };

                    let hash = hash160::Hash::hash(pk);

                    let (prefix, rest) = Self::split_slice(&hash[..]);

                    Self::P2PK((prefix, rest.into()))
                } else if script.is_empty() {
                    let index = empty_addresses.inner();

                    empty_addresses.increment();

                    Self::Empty(index)
                } else if script.is_op_return() || script.is_provably_unspendable() {
                    unreachable!()
                } else if script.is_multisig() {
                    let vec = multisig_addresses(script);

                    if vec.is_empty() {
                        dbg!(txout);
                        panic!("Multisig addresses cannot be empty !");
                    }

                    let mut vec = vec.into_iter().sorted_unstable().concat();

                    // TODO: Terrible! Store everything instead of only the 510 first bytes but how
                    // Sanakirja key limit is [u8; 510] and some multisig transactions have 999 keys
                    if vec.len() > SANAKIRJA_MAX_KEY_SIZE {
                        vec = vec.drain(..SANAKIRJA_MAX_KEY_SIZE).collect_vec();
                    }

                    Self::MultiSig(vec.into())
                } else {
                    Self::new_unknown(unknown_addresses)
                }
            }
        }
    }

    fn new_unknown(unknown_addresses: &mut Counter) -> Address {
        let index = unknown_addresses.inner();
        unknown_addresses.increment();
        Self::Unknown(index)
    }

    fn split_slice(slice: &[u8]) -> (u16, &[u8]) {
        let prefix = ((slice[0] as u16) << 2) + ((slice[1] as u16) >> 6);
        let rest = &slice[1..];
        (prefix, rest)
    }

    fn payload_to_slice(payload: &Payload) -> &[u8] {
        match payload {
            Payload::PubkeyHash(hash) => &hash[..],
            Payload::ScriptHash(hash) => &hash[..],
            Payload::WitnessProgram(witness_program) => witness_program.program().as_bytes(),
            _ => unreachable!(),
        }
    }
}
