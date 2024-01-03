use std::thread;

use bitcoin::{address::Payload, Address, WitnessVersion};
use bitcoin_hashes::Hash;
use itertools::Itertools;
use sanakirja::Error;

use crate::{
    structs::{Database, SizedDatabase, UnsizedDatabase, U8_20, U8_32},
    traits::Databases,
};

use super::AddressKind;

type Value = u32;
type DbU820 = SizedDatabase<U8_20, Value>;
type DbU832 = SizedDatabase<U8_32, Value>;
type DbUnsized = UnsizedDatabase<Box<[u8]>, [u8], Value>;

type DbP2PKH = DbU820;
type DbP2SH = DbU820;
type DbP2WPKH = DbU820;
type DbP2WSH = DbU832;
type DbP2TR = DbU832;
type DbUnknown = DbUnsized;
type DbMultisig = DbUnsized;

// https://unchained.com/blog/bitcoin-address-types-compared/
#[derive(Default)]
pub struct AddressToAddressIndex {
    p2pkh: Option<DbP2PKH>,
    p2sh: Option<DbP2SH>,
    p2wpkh: Option<DbP2WPKH>,
    p2wsh: Option<DbP2WSH>,
    p2tr: Option<DbP2TR>,
    unknown: Option<DbUnknown>,
    multisig: Option<DbMultisig>,
}

impl AddressToAddressIndex {
    pub fn get(&mut self, addresses: &[Address]) -> Option<Value> {
        if addresses.is_empty() {
            panic!("Shouldn't be empty");
        } else if addresses.len() == 1 {
            let address = addresses.first().unwrap();

            // Clone (very cheap) to satisfy the borrow checker, will be fixed soon
            // https://github.com/rust-lang/rust/issues/54663
            self.get_single(address).cloned().or(self
                .open_unknown()
                .get(&Self::address_to_payload_vec(address).into_boxed_slice())
                .cloned())
        } else {
            self.open_multisig()
                .get(&Self::concat_addresses(addresses))
                .cloned()
        }
    }

    fn get_single(&mut self, address: &Address) -> Option<&Value> {
        match address.payload() {
            Payload::PubkeyHash(hash) => {
                if let Some(value) = self.open_p2pkh().get(&U8_20::from(&hash[..])) {
                    return Some(value);
                }
            }
            Payload::ScriptHash(hash) => {
                if let Some(value) = self.open_p2sh().get(&U8_20::from(&hash[..])) {
                    return Some(value);
                }
            }
            Payload::WitnessProgram(witness_program) => {
                let program = witness_program.program();
                let version = witness_program.version();

                let slice = program.as_bytes();

                match version {
                    WitnessVersion::V0 if program.len() == 20 => {
                        if let Some(value) = self.open_p2wpkh().get(&U8_20::from(slice)) {
                            return Some(value);
                        }
                    }
                    WitnessVersion::V0 if program.len() == 32 => {
                        if let Some(value) = self.open_p2wsh().get(&U8_32::from(slice)) {
                            return Some(value);
                        }
                    }
                    WitnessVersion::V1 if program.len() == 32 => {
                        if let Some(value) = self.open_p2tr().get(&U8_32::from(slice)) {
                            return Some(value);
                        }
                    }
                    _ => {}
                }
            }
            &_ => unreachable!(),
        }

        None
    }

    pub fn insert(&mut self, addresses: &[Address], value: Value) -> (AddressKind, Option<Value>) {
        if addresses.is_empty() {
            panic!("Shouldn't be empty");
        } else if addresses.len() == 1 {
            let address = addresses.first().unwrap();

            match address.payload() {
                Payload::PubkeyHash(hash) => {
                    return (
                        AddressKind::P2PKH,
                        self.open_p2pkh().insert(U8_20::from(&hash[..]), value),
                    );
                }
                Payload::ScriptHash(hash) => {
                    return (
                        AddressKind::P2SH,
                        self.open_p2sh().insert(U8_20::from(&hash[..]), value),
                    );
                }
                Payload::WitnessProgram(witness_program) => {
                    let program = witness_program.program();
                    let version = witness_program.version();

                    let slice = program.as_bytes();

                    match version {
                        WitnessVersion::V0 if program.len() == 20 => {
                            return (
                                AddressKind::P2WPKH,
                                self.open_p2wpkh().insert(U8_20::from(slice), value),
                            );
                        }
                        WitnessVersion::V0 if program.len() == 32 => {
                            return (
                                AddressKind::P2WSH,
                                self.open_p2wsh().insert(U8_32::from(slice), value),
                            );
                        }
                        WitnessVersion::V1 if program.len() == 32 => {
                            return (
                                AddressKind::P2TR,
                                self.open_p2tr().insert(U8_32::from(slice), value),
                            );
                        }
                        _ => {}
                    }
                }
                &_ => unreachable!(),
            }

            (
                AddressKind::Unknown,
                self.open_unknown().insert(
                    Self::address_to_payload_vec(address).into_boxed_slice(),
                    value,
                ),
            )
        } else {
            (
                AddressKind::MultiSig,
                self.open_multisig()
                    .insert(Self::concat_addresses(addresses), value),
            )
        }
    }

    fn address_to_payload_vec(address: &Address) -> Vec<u8> {
        match address.payload() {
            Payload::PubkeyHash(hash) => hash.as_byte_array().to_vec(),
            Payload::ScriptHash(hash) => hash.as_byte_array().to_vec(),
            Payload::WitnessProgram(witness_program) => {
                witness_program.program().as_bytes().to_vec()
            }
            _ => unreachable!(),
        }
    }

    fn concat_addresses(addresses: &[Address]) -> Box<[u8]> {
        addresses
            .iter()
            .map(Self::address_to_payload_vec)
            .sorted_unstable()
            .concat()
            .into_boxed_slice()
    }

    fn open_p2pkh(&mut self) -> &mut DbP2PKH {
        self.p2pkh
            .get_or_insert_with(|| Database::open(Self::folder(), "p2pkh", |key| key).unwrap())
    }

    fn open_p2sh(&mut self) -> &mut DbP2SH {
        self.p2sh
            .get_or_insert_with(|| Database::open(Self::folder(), "p2sh", |key| key).unwrap())
    }

    fn open_p2wpkh(&mut self) -> &mut DbP2WPKH {
        self.p2wpkh
            .get_or_insert_with(|| Database::open(Self::folder(), "p2wpkh", |key| key).unwrap())
    }

    fn open_p2wsh(&mut self) -> &mut DbP2WSH {
        self.p2wsh
            .get_or_insert_with(|| Database::open(Self::folder(), "p2wsh", |key| key).unwrap())
    }

    fn open_p2tr(&mut self) -> &mut DbP2TR {
        self.p2tr
            .get_or_insert_with(|| Database::open(Self::folder(), "p2tr", |key| key).unwrap())
    }

    fn open_unknown(&mut self) -> &mut DbUnknown {
        self.unknown.get_or_insert_with(|| {
            Database::open(Self::folder(), "unknown", |key| key as &[u8]).unwrap()
        })
    }

    fn open_multisig(&mut self) -> &mut DbMultisig {
        self.multisig.get_or_insert_with(|| {
            Database::open(Self::folder(), "multisig", |key| key as &[u8]).unwrap()
        })
    }
}

impl Databases for AddressToAddressIndex {
    fn open(height: usize) -> color_eyre::Result<Self> {
        if height == 0 {
            let _ = Self::clear();
        }

        Ok(Self::default())
    }

    fn export(self) -> color_eyre::Result<(), Error> {
        thread::scope(|s| {
            s.spawn(|| self.p2pkh.map(|db| db.export()));
            s.spawn(|| self.p2sh.map(|db| db.export()));
            s.spawn(|| self.p2wpkh.map(|db| db.export()));
            s.spawn(|| self.p2wsh.map(|db| db.export()));
            s.spawn(|| self.p2tr.map(|db| db.export()));
            s.spawn(|| self.unknown.map(|db| db.export()));
            s.spawn(|| self.multisig.map(|db| db.export()));
        });

        Ok(())
    }

    fn folder<'a>() -> &'a str {
        "address_to_address_index"
    }
}
