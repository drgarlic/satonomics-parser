use std::thread;

use crate::{
    datasets::{AnyDataset, AnyDatasets},
    structs::{RawAddressSize, RawAddressSplit, RawAddressType},
};

use super::{AddressDataset, RawAddressFilter};

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
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        thread::scope(|scope| {
            let plankton_handle = scope.spawn(|| {
                AddressDataset::import(
                    parent_path,
                    "plankton",
                    RawAddressFilter::new_from_to(0, 10_000_000),
                    RawAddressSplit::Size(RawAddressSize::Plankton),
                )
            });
            let shrimp_handle = scope.spawn(|| {
                AddressDataset::import(
                    parent_path,
                    "shrimp",
                    RawAddressFilter::new_from_to(10_000_000, 100_000_000),
                    RawAddressSplit::Size(RawAddressSize::Shrimp),
                )
            });
            let crab_handle = scope.spawn(|| {
                AddressDataset::import(
                    parent_path,
                    "crab",
                    RawAddressFilter::new_from_to(100_000_000, 1_000_000_000),
                    RawAddressSplit::Size(RawAddressSize::Crab),
                )
            });
            let fish_handle = scope.spawn(|| {
                AddressDataset::import(
                    parent_path,
                    "fish",
                    RawAddressFilter::new_from_to(1_000_000_000, 10_000_000_000),
                    RawAddressSplit::Size(RawAddressSize::Fish),
                )
            });
            let shark_handle = scope.spawn(|| {
                AddressDataset::import(
                    parent_path,
                    "shark",
                    RawAddressFilter::new_from_to(10_000_000_000, 100_000_000_000),
                    RawAddressSplit::Size(RawAddressSize::Shark),
                )
            });
            let whale_handle = scope.spawn(|| {
                AddressDataset::import(
                    parent_path,
                    "whale",
                    RawAddressFilter::new_from_to(100_000_000_000, 1_000_000_000_000),
                    RawAddressSplit::Size(RawAddressSize::Whale),
                )
            });
            let humpback_handle = scope.spawn(|| {
                AddressDataset::import(
                    parent_path,
                    "humpback",
                    RawAddressFilter::new_from_to(1_000_000_000_000, 10_000_000_000_000),
                    RawAddressSplit::Size(RawAddressSize::Humpback),
                )
            });
            let megalodon_handle = scope.spawn(|| {
                AddressDataset::import(
                    parent_path,
                    "megalodon",
                    RawAddressFilter::new_from_to(10_000_000_000_000, u64::MAX),
                    RawAddressSplit::Size(RawAddressSize::Megalodon),
                )
            });

            let p2pk_handle = scope.spawn(|| {
                AddressDataset::import(
                    parent_path,
                    "p2pk",
                    RawAddressFilter::AddressType(RawAddressType::P2PK),
                    RawAddressSplit::Type(RawAddressType::P2PK),
                )
            });
            let p2pkh_handle = scope.spawn(|| {
                AddressDataset::import(
                    parent_path,
                    "p2pkh",
                    RawAddressFilter::AddressType(RawAddressType::P2PKH),
                    RawAddressSplit::Type(RawAddressType::P2PKH),
                )
            });
            let p2sh_handle = scope.spawn(|| {
                AddressDataset::import(
                    parent_path,
                    "p2sh",
                    RawAddressFilter::AddressType(RawAddressType::P2SH),
                    RawAddressSplit::Type(RawAddressType::P2SH),
                )
            });
            let p2wpkh_handle = scope.spawn(|| {
                AddressDataset::import(
                    parent_path,
                    "p2wpkh",
                    RawAddressFilter::AddressType(RawAddressType::P2WPKH),
                    RawAddressSplit::Type(RawAddressType::P2WPKH),
                )
            });
            let p2wsh_handle = scope.spawn(|| {
                AddressDataset::import(
                    parent_path,
                    "p2wsh",
                    RawAddressFilter::AddressType(RawAddressType::P2WSH),
                    RawAddressSplit::Type(RawAddressType::P2WSH),
                )
            });

            let p2tr = AddressDataset::import(
                parent_path,
                "p2tr",
                RawAddressFilter::AddressType(RawAddressType::P2TR),
                RawAddressSplit::Type(RawAddressType::P2TR),
            )?;

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
                p2tr,
            })
        })
    }

    // pub fn needs_sorted_address_data(&self, date: NaiveDate, height: usize) -> bool {
    //     [
    //         &self.plankton,
    //         &self.shrimp,
    //         &self.crab,
    //         &self.fish,
    //         &self.shark,
    //         &self.whale,
    //         &self.humpback,
    //         &self.megalodon,
    //         &self.p2pk,
    //         &self.p2pkh,
    //         &self.p2sh,
    //         &self.p2wpkh,
    //         &self.p2wsh,
    //         &self.p2tr,
    //     ]
    //     .iter()
    //     .any(|dataset| dataset.needs_sorted_address_data(date, height))
    // }
}

impl AnyDatasets for AddressDatasets {
    fn to_any_dataset_vec(&self) -> Vec<&(dyn AnyDataset + Send + Sync)> {
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

    fn name<'a>() -> &'a str {
        "address"
    }
}
