use std::{collections::BTreeMap, mem, thread};

use rayon::prelude::*;

use crate::structs::{
    Database, RawAddress, SizedDatabase, U8x19, U8x31, UnsizedDatabase as _UnsizedDatabase,
};

use super::{AnyDatabaseGroup, Metadata};

type Value = u32;
type U8x19Database = SizedDatabase<U8x19, Value>;
type U8x31Database = SizedDatabase<U8x31, Value>;
type U32Database = SizedDatabase<u32, Value>;
type UnsizedDatabase = _UnsizedDatabase<Box<[u8]>, [u8], Value>;

type P2PKDatabase = U8x19Database;
type P2PKHDatabase = U8x19Database;
type P2SHDatabase = U8x19Database;
type P2WPKHDatabase = U8x19Database;
type P2WSHDatabase = U8x31Database;
type P2TRDatabase = U8x31Database;
type UnknownDatabase = U32Database;
type EmptyDatabase = U32Database;
type MultisigDatabase = UnsizedDatabase;

pub struct RawAddressToAddressIndex {
    p2pk: BTreeMap<u16, P2PKDatabase>,
    p2pkh: BTreeMap<u16, P2PKHDatabase>,
    p2sh: BTreeMap<u16, P2SHDatabase>,
    p2wpkh: BTreeMap<u16, P2WPKHDatabase>,
    p2wsh: BTreeMap<u16, P2WSHDatabase>,
    p2tr: BTreeMap<u16, P2TRDatabase>,
    unknown: Option<UnknownDatabase>,
    empty: Option<EmptyDatabase>,
    multisig: Option<MultisigDatabase>,
    pub metadata: Metadata,
}

impl RawAddressToAddressIndex {
    #[allow(unused)]
    pub fn safe_get(&mut self, raw_address: &RawAddress) -> Option<&Value> {
        match raw_address {
            RawAddress::Empty(key) => self.open_empty().get(key),
            RawAddress::Unknown(key) => self.open_unknown().get(key),
            RawAddress::MultiSig(key) => self.open_multisig().get(key),
            RawAddress::P2PK((prefix, rest)) => self.open_p2pk(*prefix).get(rest),
            RawAddress::P2PKH((prefix, rest)) => self.open_p2pkh(*prefix).get(rest),
            RawAddress::P2SH((prefix, rest)) => self.open_p2sh(*prefix).get(rest),
            RawAddress::P2WPKH((prefix, rest)) => self.open_p2wpkh(*prefix).get(rest),
            RawAddress::P2WSH((prefix, rest)) => self.open_p2wsh(*prefix).get(rest),
            RawAddress::P2TR((prefix, rest)) => self.open_p2tr(*prefix).get(rest),
        }
    }

    pub fn open_db(&mut self, raw_address: &RawAddress) {
        match raw_address {
            RawAddress::Empty(_) => {
                self.open_empty();
            }
            RawAddress::Unknown(_) => {
                self.open_unknown();
            }
            RawAddress::MultiSig(_) => {
                self.open_multisig();
            }
            RawAddress::P2PK((prefix, _)) => {
                self.open_p2pk(*prefix);
            }
            RawAddress::P2PKH((prefix, _)) => {
                self.open_p2pkh(*prefix);
            }
            RawAddress::P2SH((prefix, _)) => {
                self.open_p2sh(*prefix);
            }
            RawAddress::P2WPKH((prefix, _)) => {
                self.open_p2wpkh(*prefix);
            }
            RawAddress::P2WSH((prefix, _)) => {
                self.open_p2wsh(*prefix);
            }
            RawAddress::P2TR((prefix, _)) => {
                self.open_p2tr(*prefix);
            }
        }
    }

    /// Doesn't check if the database is open contrary to `safe_get` which does and opens if needed.
    /// Though it makes it easy to use with rayon
    pub fn unsafe_get(&self, raw_address: &RawAddress) -> Option<&Value> {
        match raw_address {
            RawAddress::Empty(key) => self.empty.as_ref().unwrap().get(key),
            RawAddress::Unknown(key) => self.unknown.as_ref().unwrap().get(key),
            RawAddress::MultiSig(key) => self.multisig.as_ref().unwrap().get(key),
            RawAddress::P2PK((prefix, key)) => self.p2pk.get(prefix).unwrap().get(key),
            RawAddress::P2PKH((prefix, key)) => self.p2pkh.get(prefix).unwrap().get(key),
            RawAddress::P2SH((prefix, key)) => self.p2sh.get(prefix).unwrap().get(key),
            RawAddress::P2WPKH((prefix, key)) => self.p2wpkh.get(prefix).unwrap().get(key),
            RawAddress::P2WSH((prefix, key)) => self.p2wsh.get(prefix).unwrap().get(key),
            RawAddress::P2TR((prefix, key)) => self.p2tr.get(prefix).unwrap().get(key),
        }
    }

    pub fn unsafe_get_from_puts(&self, raw_address: &RawAddress) -> Option<&Value> {
        match raw_address {
            RawAddress::Empty(key) => self.empty.as_ref().unwrap().get_from_puts(key),
            RawAddress::Unknown(key) => self.unknown.as_ref().unwrap().get_from_puts(key),
            RawAddress::MultiSig(key) => self.multisig.as_ref().unwrap().get_from_puts(key),
            RawAddress::P2PK((prefix, key)) => self.p2pk.get(prefix).unwrap().get_from_puts(key),
            RawAddress::P2PKH((prefix, key)) => self.p2pkh.get(prefix).unwrap().get_from_puts(key),
            RawAddress::P2SH((prefix, key)) => self.p2sh.get(prefix).unwrap().get_from_puts(key),
            RawAddress::P2WPKH((prefix, key)) => {
                self.p2wpkh.get(prefix).unwrap().get_from_puts(key)
            }
            RawAddress::P2WSH((prefix, key)) => self.p2wsh.get(prefix).unwrap().get_from_puts(key),
            RawAddress::P2TR((prefix, key)) => self.p2tr.get(prefix).unwrap().get_from_puts(key),
        }
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

    pub fn open_p2pk(&mut self, prefix: u16) -> &mut P2PKDatabase {
        self.p2pk.entry(prefix).or_insert_with(|| {
            Database::open(
                &format!("{}/{}", Self::folder(), "p2pk"),
                &prefix.to_string(),
                |key| key,
            )
            .unwrap()
        })
    }

    pub fn open_p2pkh(&mut self, prefix: u16) -> &mut P2PKHDatabase {
        self.p2pkh.entry(prefix).or_insert_with(|| {
            Database::open(
                &format!("{}/{}", Self::folder(), "p2pkh"),
                &prefix.to_string(),
                |key| key,
            )
            .unwrap()
        })
    }

    pub fn open_p2sh(&mut self, prefix: u16) -> &mut P2SHDatabase {
        self.p2sh.entry(prefix).or_insert_with(|| {
            Database::open(
                &format!("{}/{}", Self::folder(), "p2sh"),
                &prefix.to_string(),
                |key| key,
            )
            .unwrap()
        })
    }

    pub fn open_p2wpkh(&mut self, prefix: u16) -> &mut P2WPKHDatabase {
        self.p2wpkh.entry(prefix).or_insert_with(|| {
            Database::open(
                &format!("{}/{}", Self::folder(), "p2wpkh"),
                &prefix.to_string(),
                |key| key,
            )
            .unwrap()
        })
    }

    pub fn open_p2wsh(&mut self, prefix: u16) -> &mut P2WSHDatabase {
        self.p2wsh.entry(prefix).or_insert_with(|| {
            Database::open(
                &format!("{}/{}", Self::folder(), "p2wsh"),
                &prefix.to_string(),
                |key| key,
            )
            .unwrap()
        })
    }

    pub fn open_p2tr(&mut self, prefix: u16) -> &mut P2TRDatabase {
        self.p2tr.entry(prefix).or_insert_with(|| {
            Database::open(
                &format!("{}/{}", Self::folder(), "p2tr"),
                &prefix.to_string(),
                |key| key,
            )
            .unwrap()
        })
    }

    pub fn open_unknown(&mut self) -> &mut UnknownDatabase {
        self.unknown
            .get_or_insert_with(|| Database::open(Self::folder(), "unknown", |key| key).unwrap())
    }

    pub fn open_empty(&mut self) -> &mut UnknownDatabase {
        self.empty
            .get_or_insert_with(|| Database::open(Self::folder(), "empty", |key| key).unwrap())
    }

    pub fn open_multisig(&mut self) -> &mut MultisigDatabase {
        self.multisig.get_or_insert_with(|| {
            Database::open(Self::folder(), "multisig", |key| key as &[u8]).unwrap()
        })
    }
}

impl AnyDatabaseGroup for RawAddressToAddressIndex {
    fn import() -> Self {
        Self {
            p2pk: BTreeMap::default(),
            p2pkh: BTreeMap::default(),
            p2sh: BTreeMap::default(),
            p2wpkh: BTreeMap::default(),
            p2wsh: BTreeMap::default(),
            p2tr: BTreeMap::default(),
            unknown: None,
            empty: None,
            multisig: None,
            metadata: Metadata::import(&Self::full_path()),
        }
    }

    fn export(&mut self) -> color_eyre::Result<()> {
        thread::scope(|s| {
            s.spawn(|| {
                mem::take(&mut self.p2pk)
                    .into_par_iter()
                    .try_for_each(|(_, db)| db.export())
            });
            s.spawn(|| {
                mem::take(&mut self.p2pkh)
                    .into_par_iter()
                    .try_for_each(|(_, db)| db.export())
            });
            s.spawn(|| {
                mem::take(&mut self.p2sh)
                    .into_par_iter()
                    .try_for_each(|(_, db)| db.export())
            });
            s.spawn(|| {
                mem::take(&mut self.p2wpkh)
                    .into_par_iter()
                    .try_for_each(|(_, db)| db.export())
            });
            s.spawn(|| {
                mem::take(&mut self.p2wsh)
                    .into_par_iter()
                    .try_for_each(|(_, db)| db.export())
            });
            s.spawn(|| {
                mem::take(&mut self.p2tr)
                    .into_par_iter()
                    .try_for_each(|(_, db)| db.export())
            });

            s.spawn(|| self.unknown.take().map(|db| db.export()));
            s.spawn(|| self.empty.take().map(|db| db.export()));
            s.spawn(|| self.multisig.take().map(|db| db.export()));
        });

        self.metadata.export()?;

        Ok(())
    }

    fn sub_reset(&mut self) {
        self.metadata.reset()
    }

    fn folder<'a>() -> &'a str {
        "raw_address_to_address_index"
    }
}
