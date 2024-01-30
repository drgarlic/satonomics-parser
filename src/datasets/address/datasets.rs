use std::{fs, thread};

use crate::{
    datasets::{AnyDataset, AnyDatasets},
    structs::RawAddressType,
};

use super::{AddressDataset, AddressFilter};

pub struct AddressDatasets {
    plankton: AddressDataset,
    shrimp: AddressDataset,
    crab: AddressDataset,
    fish: AddressDataset,
    shark: AddressDataset,
    whale: AddressDataset,
    humpback: AddressDataset,
    megalodon: AddressDataset,

    p2pk: AddressDataset,
    p2pkh: AddressDataset,
    p2sh: AddressDataset,
    p2wpkh: AddressDataset,
    p2wsh: AddressDataset,
    p2tr: AddressDataset,
}

impl AddressDatasets {
    pub fn import(path: &str) -> color_eyre::Result<Self> {
        let path = format!("{path}/address");
        let f = |s: &str| format!("{path}/{s}");

        thread::scope(|scope| {
            let plankton_handle = scope.spawn(|| {
                AddressDataset::import(&f("plankton"), AddressFilter::new_from_to(0, 10_000_000))
            });
            let shrimp_handle = scope.spawn(|| {
                AddressDataset::import(
                    &f("shrimp"),
                    AddressFilter::new_from_to(10_000_000, 100_000_000),
                )
            });
            let crab_handle = scope.spawn(|| {
                AddressDataset::import(
                    &f("crab"),
                    AddressFilter::new_from_to(100_000_000, 1_000_000_000),
                )
            });
            let fish_handle = scope.spawn(|| {
                AddressDataset::import(
                    &f("fish"),
                    AddressFilter::new_from_to(1_000_000_000, 10_000_000_000),
                )
            });
            let shark_handle = scope.spawn(|| {
                AddressDataset::import(
                    &f("shark"),
                    AddressFilter::new_from_to(10_000_000_000, 100_000_000_000),
                )
            });
            let whale_handle = scope.spawn(|| {
                AddressDataset::import(
                    &f("whale"),
                    AddressFilter::new_from_to(100_000_000_000, 1_000_000_000_000),
                )
            });
            let humpback_handle = scope.spawn(|| {
                AddressDataset::import(
                    &f("humpback"),
                    AddressFilter::new_from_to(1_000_000_000_000, 10_000_000_000_000),
                )
            });
            let megalodon_handle = scope.spawn(|| {
                AddressDataset::import(
                    &f("megalodon"),
                    AddressFilter::new_from_to(10_000_000_000_000, u64::MAX),
                )
            });

            let p2pk_handle = scope.spawn(|| {
                AddressDataset::import(&f("p2pk"), AddressFilter::AddressType(RawAddressType::P2PK))
            });
            let p2pkh_handle = scope.spawn(|| {
                AddressDataset::import(
                    &f("p2pkh"),
                    AddressFilter::AddressType(RawAddressType::P2PKH),
                )
            });
            let p2sh_handle = scope.spawn(|| {
                AddressDataset::import(&f("p2sh"), AddressFilter::AddressType(RawAddressType::P2SH))
            });
            let p2wpkh_handle = scope.spawn(|| {
                AddressDataset::import(
                    &f("p2wpkh"),
                    AddressFilter::AddressType(RawAddressType::P2WPKH),
                )
            });
            let p2wsh_handle = scope.spawn(|| {
                AddressDataset::import(
                    &f("p2wsh"),
                    AddressFilter::AddressType(RawAddressType::P2WSH),
                )
            });
            let p2tr_handle = scope.spawn(|| {
                AddressDataset::import(&f("p2tr"), AddressFilter::AddressType(RawAddressType::P2TR))
            });

            Ok(Self {
                plankton: plankton_handle.join().unwrap()?,
                shrimp: shrimp_handle.join().unwrap()?,
                crab: crab_handle.join().unwrap()?,
                fish: fish_handle.join().unwrap()?,
                shark: shark_handle.join().unwrap()?,
                whale: whale_handle.join().unwrap()?,
                humpback: humpback_handle.join().unwrap()?,
                megalodon: megalodon_handle.join().unwrap()?,

                p2pk: p2pk_handle.join().unwrap()?,
                p2pkh: p2pkh_handle.join().unwrap()?,
                p2sh: p2sh_handle.join().unwrap()?,
                p2wpkh: p2wpkh_handle.join().unwrap()?,
                p2wsh: p2wsh_handle.join().unwrap()?,
                p2tr: p2tr_handle.join().unwrap()?,
            })
        })
    }
}

impl AnyDatasets for AddressDatasets {
    fn to_vec(&self) -> Vec<&(dyn AnyDataset + Send + Sync)> {
        vec![
            &self.plankton,
            &self.shrimp,
            &self.crab,
            &self.fish,
            &self.shark,
            &self.whale,
            &self.humpback,
            &self.megalodon,
            &self.p2pk,
            &self.p2pkh,
            &self.p2sh,
            &self.p2wpkh,
            &self.p2wsh,
            &self.p2tr,
        ]
    }
}
