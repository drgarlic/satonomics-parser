use std::thread;

use nohash_hasher::IntMap;
use rayon::prelude::*;
use sanakirja::Error;

use crate::{
    structs::{Database, SizedDatabase, U8x19, U8x31, UnsizedDatabase},
    traits::Databases,
};

use super::RawAddress;

type Value = u32;
type DbU8x19 = SizedDatabase<U8x19, Value>;
type DbU8x31 = SizedDatabase<U8x31, Value>;
type DbU32 = SizedDatabase<u32, Value>;
type DbUnsized = UnsizedDatabase<Box<[u8]>, [u8], Value>;

type DbP2PK = DbU8x19;
type DbP2PKH = DbU8x19;
type DbP2SH = DbU8x19;
type DbP2WPKH = DbU8x19;
type DbP2WSH = DbU8x31;
type DbP2TR = DbU8x31;
type DbUnknown = DbU32;
type DbEmpty = DbU32;
type DbMultisig = DbUnsized;

// https://unchained.com/blog/bitcoin-address-types-compared/
#[derive(Default)]
pub struct RawAddressToAddressIndex {
    p2pk: IntMap<u8, DbP2PK>,
    p2pkh: IntMap<u8, DbP2PKH>,
    p2sh: IntMap<u8, DbP2SH>,
    p2wpkh: IntMap<u8, DbP2WPKH>,
    p2wsh: IntMap<u8, DbP2WSH>,
    p2tr: IntMap<u8, DbP2TR>,
    unknown: Option<DbUnknown>,
    empty: Option<DbEmpty>,
    multisig: Option<DbMultisig>,
}

impl RawAddressToAddressIndex {
    pub fn get(&mut self, raw_address: &RawAddress) -> Option<Value> {
        (match raw_address {
            RawAddress::Empty(key) => self.open_empty().get(key),
            RawAddress::Unknown(key) => self.open_unknown().get(key),
            RawAddress::MultiSig(key) => self.open_multisig().get(key),
            RawAddress::P2PK((prefix, rest)) => self.open_p2pk(*prefix).get(rest),
            RawAddress::P2PKH((prefix, rest)) => self.open_p2pkh(*prefix).get(rest),
            RawAddress::P2SH((prefix, rest)) => self.open_p2sh(*prefix).get(rest),
            RawAddress::P2WPKH((prefix, rest)) => self.open_p2wpkh(*prefix).get(rest),
            RawAddress::P2WSH((prefix, rest)) => self.open_p2wsh(*prefix).get(rest),
            RawAddress::P2TR((prefix, rest)) => self.open_p2tr(*prefix).get(rest),
        })
        .cloned()
    }

    pub fn insert(&mut self, raw_address: RawAddress, value: Value) -> Option<Value> {
        match raw_address {
            RawAddress::Empty(key) => self.open_empty().insert(key, value),
            RawAddress::Unknown(key) => self.open_unknown().insert(key, value),
            RawAddress::MultiSig(key) => self.open_multisig().insert(key, value),
            RawAddress::P2PK((prefix, rest)) => self.open_p2pk(prefix).insert(rest, value),
            RawAddress::P2PKH((prefix, rest)) => self.open_p2pkh(prefix).insert(rest, value),
            RawAddress::P2SH((prefix, rest)) => self.open_p2sh(prefix).insert(rest, value),
            RawAddress::P2WPKH((prefix, rest)) => self.open_p2wpkh(prefix).insert(rest, value),
            RawAddress::P2WSH((prefix, rest)) => self.open_p2wsh(prefix).insert(rest, value),
            RawAddress::P2TR((prefix, rest)) => self.open_p2tr(prefix).insert(rest, value),
        }
    }

    pub fn open_p2pk(&mut self, prefix: u8) -> &mut DbP2PK {
        self.p2pk.entry(prefix).or_insert_with(|| {
            Database::open(
                &format!("{}/{}", Self::folder(), "p2pk"),
                &prefix.to_string(),
                |key| key,
            )
            .unwrap()
        })
    }

    pub fn open_p2pkh(&mut self, prefix: u8) -> &mut DbP2PKH {
        self.p2pkh.entry(prefix).or_insert_with(|| {
            Database::open(
                &format!("{}/{}", Self::folder(), "p2pkh"),
                &prefix.to_string(),
                |key| key,
            )
            .unwrap()
        })
    }

    pub fn open_p2sh(&mut self, prefix: u8) -> &mut DbP2SH {
        self.p2sh.entry(prefix).or_insert_with(|| {
            Database::open(
                &format!("{}/{}", Self::folder(), "p2sh"),
                &prefix.to_string(),
                |key| key,
            )
            .unwrap()
        })
    }

    pub fn open_p2wpkh(&mut self, prefix: u8) -> &mut DbP2WPKH {
        self.p2wpkh.entry(prefix).or_insert_with(|| {
            Database::open(
                &format!("{}/{}", Self::folder(), "p2wpkh"),
                &prefix.to_string(),
                |key| key,
            )
            .unwrap()
        })
    }

    pub fn open_p2wsh(&mut self, prefix: u8) -> &mut DbP2WSH {
        self.p2wsh.entry(prefix).or_insert_with(|| {
            Database::open(
                &format!("{}/{}", Self::folder(), "p2wsh"),
                &prefix.to_string(),
                |key| key,
            )
            .unwrap()
        })
    }

    pub fn open_p2tr(&mut self, prefix: u8) -> &mut DbP2TR {
        self.p2tr.entry(prefix).or_insert_with(|| {
            Database::open(
                &format!("{}/{}", Self::folder(), "p2tr"),
                &prefix.to_string(),
                |key| key,
            )
            .unwrap()
        })
    }

    pub fn open_unknown(&mut self) -> &mut DbUnknown {
        self.unknown
            .get_or_insert_with(|| Database::open(Self::folder(), "unknown", |key| key).unwrap())
    }

    pub fn open_empty(&mut self) -> &mut DbUnknown {
        self.empty
            .get_or_insert_with(|| Database::open(Self::folder(), "empty", |key| key).unwrap())
    }

    pub fn open_multisig(&mut self) -> &mut DbMultisig {
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

    fn export(mut self) -> color_eyre::Result<(), Error> {
        thread::scope(|s| {
            s.spawn(|| self.p2pk.par_drain().try_for_each(|(_, db)| db.export()));
            s.spawn(|| self.p2pkh.par_drain().try_for_each(|(_, db)| db.export()));
            s.spawn(|| self.p2sh.par_drain().try_for_each(|(_, db)| db.export()));
            s.spawn(|| self.p2wpkh.par_drain().try_for_each(|(_, db)| db.export()));
            s.spawn(|| self.p2wsh.par_drain().try_for_each(|(_, db)| db.export()));
            s.spawn(|| self.p2tr.par_drain().try_for_each(|(_, db)| db.export()));
            s.spawn(|| self.unknown.map(|db| db.export()));
            s.spawn(|| self.empty.map(|db| db.export()));
            s.spawn(|| self.multisig.map(|db| db.export()));
        });

        Ok(())
    }

    fn folder<'a>() -> &'a str {
        "raw_address_to_address_index"
    }
}
