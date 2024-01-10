use bincode::{Decode, Encode};
use bitcoin::{address::Payload, TxOut};
use bitcoin_hashes::{hash160, Hash};
use itertools::Itertools;

use crate::{
    bitcoin::multisig_addresses,
    structs::{U8x19, U8x31, SANAKIRJA_MAX_KEY_SIZE},
};

use super::Counters;

#[derive(Debug, Encode, Decode, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RawAddressType {
    Empty,
    Unknown,
    MultiSig,
    P2PK,
    P2PKH,
    P2SH,
    P2WPKH,
    P2WSH,
    P2TR,
}

#[derive(Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum RawAddress {
    // https://mempool.space/tx/7bd54def72825008b4ca0f4aeff13e6be2c5fe0f23430629a9d484a1ac2a29b8
    Empty(u32),
    Unknown(u32),
    // https://mempool.space/tx/274f8be3b7b9b1a220285f5f71f61e2691dd04df9d69bb02a8b3b85f91fb1857
    MultiSig(Box<[u8]>),
    P2PK((u8, U8x19)),
    P2PKH((u8, U8x19)),
    P2SH((u8, U8x19)),
    P2WPKH((u8, U8x19)),
    P2WSH((u8, U8x31)),
    P2TR((u8, U8x31)),
}

impl RawAddress {
    pub fn to_type(&self) -> RawAddressType {
        match self {
            Self::Empty(_) => RawAddressType::Empty,
            Self::Unknown(_) => RawAddressType::Unknown,
            Self::MultiSig(_) => RawAddressType::MultiSig,
            Self::P2PK(_) => RawAddressType::P2PK,
            Self::P2PKH(_) => RawAddressType::P2PKH,
            Self::P2SH(_) => RawAddressType::P2SH,
            Self::P2WPKH(_) => RawAddressType::P2WPKH,
            Self::P2WSH(_) => RawAddressType::P2WSH,
            Self::P2TR(_) => RawAddressType::P2TR,
        }
    }

    pub fn from(txout: &TxOut, counters: &mut Counters) -> Self {
        let script = &txout.script_pubkey;

        match Payload::from_script(script) {
            Ok(payload) => {
                let slice = payload_to_slice(&payload);
                let prefix = slice[0];
                let rest = &slice[1..];

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
                    unreachable!()
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

                    let slice = &hash[..];
                    let prefix = slice[0];
                    let rest = &slice[1..];

                    Self::P2PK((prefix, rest.into()))
                } else if script.is_empty() {
                    let empty_addresses_counter = &mut counters.empty_addresses;
                    let index = empty_addresses_counter.inner();
                    empty_addresses_counter.increment();
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

                    // TODO: Terrible! Store everything instead of only the 510 first bytes
                    if vec.len() > SANAKIRJA_MAX_KEY_SIZE {
                        vec = vec.drain(..SANAKIRJA_MAX_KEY_SIZE).collect_vec();
                    }

                    Self::MultiSig(vec.into())
                } else {
                    let unknown_addresses_counter = &mut counters.unknown_addresses;
                    let index = unknown_addresses_counter.inner();
                    unknown_addresses_counter.increment();
                    Self::Unknown(index)
                }
            }
        }
    }
}

fn payload_to_slice(payload: &Payload) -> &[u8] {
    match payload {
        Payload::PubkeyHash(hash) => &hash[..],
        Payload::ScriptHash(hash) => &hash[..],
        Payload::WitnessProgram(witness_program) => witness_program.program().as_bytes(),
        _ => unreachable!(),
    }
}
