use bincode::{Decode, Encode};
use bitcoin::{address::Payload, ScriptBuf};
use bitcoin_hashes::{hash160, Hash};
use itertools::Itertools;

use crate::{
    bitcoin::multisig_addresses,
    computers::UnknownAddressCounter,
    structs::{U8_20, U8_32, U8_4},
};

#[derive(Debug, Encode, Decode, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RawAddressType {
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
    Unknown(U8_4),
    MultiSig(Box<[u8]>),
    P2PK(U8_20),
    P2PKH(U8_20),
    P2SH(U8_20),
    P2WPKH(U8_20),
    P2WSH(U8_32),
    P2TR(U8_32),
}

impl RawAddress {
    pub fn to_type(&self) -> RawAddressType {
        match self {
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

    pub fn from(script: &ScriptBuf, unknown_counter: &mut UnknownAddressCounter) -> Self {
        match Payload::from_script(script) {
            Ok(payload) => {
                let slice = payload_to_raw_address_slice(&payload);

                if script.is_p2pkh() {
                    Self::P2PKH(slice.into())
                } else if script.is_p2sh() {
                    Self::P2SH(slice.into())
                } else if script.is_p2wpkh() {
                    Self::P2WPKH(slice.into())
                } else if script.is_p2wsh() {
                    Self::P2WSH(slice.into())
                } else if script.is_p2tr() {
                    Self::P2TR(slice.into())
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

                    let slice = hash.as_byte_array().as_slice();

                    Self::P2PK(slice.into())
                } else if script.is_op_return()
                    || script.is_provably_unspendable()
                    || script.is_empty()
                {
                    unreachable!()
                } else if script.is_multisig() {
                    let addresses = multisig_addresses(script);

                    dbg!(&addresses);

                    Self::MultiSig(
                        addresses
                            .iter()
                            .map(|address| script_to_raw_address_slice(&address.script_pubkey()))
                            .sorted_unstable()
                            .concat()
                            .into(),
                    )
                } else {
                    let bytes = unknown_counter.to_le_bytes();

                    unknown_counter.increment();

                    Self::Unknown(bytes.as_slice().into())
                }
            }
        }
    }
}

fn script_to_raw_address_slice(script: &ScriptBuf) -> Vec<u8> {
    match Payload::from_script(script) {
        Ok(payload) => payload_to_raw_address_slice(&payload).to_vec(),
        Err(_) => unreachable!(),
    }
}

fn payload_to_raw_address_slice(payload: &Payload) -> &[u8] {
    match payload {
        Payload::PubkeyHash(hash) => &hash[..],
        Payload::ScriptHash(hash) => &hash[..],
        Payload::WitnessProgram(witness_program) => witness_program.program().as_bytes(),
        _ => unreachable!(),
    }
}
