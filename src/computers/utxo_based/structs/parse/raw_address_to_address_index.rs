use std::thread;

use sanakirja::Error;

use crate::{
    structs::{Database, SizedDatabase, UnsizedDatabase, U8_20, U8_32, U8_4},
    traits::Databases,
};

use super::RawAddress;

type Value = u32;
type DbU8_4 = SizedDatabase<U8_4, Value>;
type DbU8_20 = SizedDatabase<U8_20, Value>;
type DbU8_32 = SizedDatabase<U8_32, Value>;
type DbUnsized = UnsizedDatabase<Box<[u8]>, [u8], Value>;

type DbP2PK = DbU8_20;
type DbP2PKH = DbU8_20;
type DbP2SH = DbU8_20;
type DbP2WPKH = DbU8_20;
type DbP2WSH = DbU8_32;
type DbP2TR = DbU8_32;
type DbUnknown = DbU8_4;
type DbMultisig = DbUnsized;

// https://unchained.com/blog/bitcoin-address-types-compared/
#[derive(Default)]
pub struct RawAddressToAddressIndex {
    p2pk: Option<DbP2PK>,
    p2pkh: Option<DbP2PKH>,
    p2sh: Option<DbP2SH>,
    p2wpkh: Option<DbP2WPKH>,
    p2wsh: Option<DbP2WSH>,
    p2tr: Option<DbP2TR>,
    unknown: Option<DbUnknown>,
    multisig: Option<DbMultisig>,
}

impl RawAddressToAddressIndex {
    pub fn get(&mut self, raw_address: &RawAddress) -> Option<Value> {
        (match raw_address {
            RawAddress::Unknown(v) => self.open_unknown().get(v),
            RawAddress::MultiSig(v) => self.open_multisig().get(v),
            RawAddress::P2PK(v) => self.open_p2pk().get(v),
            RawAddress::P2PKH(v) => self.open_p2pkh().get(v),
            RawAddress::P2SH(v) => self.open_p2sh().get(v),
            RawAddress::P2WPKH(v) => self.open_p2wpkh().get(v),
            RawAddress::P2WSH(v) => self.open_p2wsh().get(v),
            RawAddress::P2TR(v) => self.open_p2tr().get(v),
        })
        .cloned()
    }

    pub fn insert(&mut self, raw_address: RawAddress, value: Value) -> Option<Value> {
        match raw_address {
            RawAddress::Unknown(key) => self.open_unknown().insert(key, value),
            RawAddress::MultiSig(key) => self.open_multisig().insert(key, value),
            RawAddress::P2PK(key) => self.open_p2pk().insert(key, value),
            RawAddress::P2PKH(key) => self.open_p2pkh().insert(key, value),
            RawAddress::P2SH(key) => self.open_p2sh().insert(key, value),
            RawAddress::P2WPKH(key) => self.open_p2wpkh().insert(key, value),
            RawAddress::P2WSH(key) => self.open_p2wsh().insert(key, value),
            RawAddress::P2TR(key) => self.open_p2tr().insert(key, value),
        }
    }

    fn open_p2pk(&mut self) -> &mut DbP2PKH {
        self.p2pk
            .get_or_insert_with(|| Database::open(Self::folder(), "p2pk", |key| key).unwrap())
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
        self.unknown
            .get_or_insert_with(|| Database::open(Self::folder(), "unknown", |key| key).unwrap())
    }

    fn open_multisig(&mut self) -> &mut DbMultisig {
        self.multisig.get_or_insert_with(|| {
            Database::open(Self::folder(), "multisig", |key| key as &[u8]).unwrap()
        })
    }
}

impl Databases for RawAddressToAddressIndex {
    fn open(height: usize) -> color_eyre::Result<Self> {
        if height == 0 {
            let _ = Self::clear();
        }

        Ok(Self::default())
    }

    fn export(self) -> color_eyre::Result<(), Error> {
        thread::scope(|s| {
            s.spawn(|| self.p2pk.map(|db| db.export()));
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
