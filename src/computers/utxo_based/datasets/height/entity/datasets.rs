use std::thread;

use crate::computers::utxo_based::{HeightDatasetTrait, HeightDatasetsTrait, RawAddressType};

use super::{EntityDataset, EntityFilter};

pub struct EntityDatasets {
    plankton: EntityDataset,
    shrimp: EntityDataset,
    crab: EntityDataset,
    fish: EntityDataset,
    shark: EntityDataset,
    whale: EntityDataset,
    humpback: EntityDataset,
    megalodon: EntityDataset,

    p2pk: EntityDataset,
    p2pkh: EntityDataset,
    p2sh: EntityDataset,
    p2wpkh: EntityDataset,
    p2wsh: EntityDataset,
    p2tr: EntityDataset,
}

impl EntityDatasets {
    pub fn import(path: &'static str) -> color_eyre::Result<Self> {
        let plankton_handle = thread::spawn(|| {
            EntityDataset::import(path, "plankton", EntityFilter::new_from_to(0, 10_000_000))
        });
        let shrimp_handle = thread::spawn(|| {
            EntityDataset::import(
                path,
                "shrimp",
                EntityFilter::new_from_to(10_000_000, 100_000_000),
            )
        });
        let crab_handle = thread::spawn(|| {
            EntityDataset::import(
                path,
                "crab",
                EntityFilter::new_from_to(100_000_000, 1_000_000_000),
            )
        });
        let fish_handle = thread::spawn(|| {
            EntityDataset::import(
                path,
                "fish",
                EntityFilter::new_from_to(1_000_000_000, 10_000_000_000),
            )
        });
        let shark_handle = thread::spawn(|| {
            EntityDataset::import(
                path,
                "shark",
                EntityFilter::new_from_to(10_000_000_000, 100_000_000_000),
            )
        });
        let whale_handle = thread::spawn(|| {
            EntityDataset::import(
                path,
                "whale",
                EntityFilter::new_from_to(100_000_000_000, 1_000_000_000_000),
            )
        });
        let humpback_handle = thread::spawn(|| {
            EntityDataset::import(
                path,
                "humpback",
                EntityFilter::new_from_to(1_000_000_000_000, 10_000_000_000_000),
            )
        });
        let megalodon_handle = thread::spawn(|| {
            EntityDataset::import(
                path,
                "megalodon",
                EntityFilter::new_from_to(10_000_000_000_000, u64::MAX),
            )
        });

        let p2pk_handle = thread::spawn(|| {
            EntityDataset::import(
                path,
                "p2pk",
                EntityFilter::AddressType(RawAddressType::P2PK),
            )
        });
        let p2pkh_handle = thread::spawn(|| {
            EntityDataset::import(
                path,
                "p2pkh",
                EntityFilter::AddressType(RawAddressType::P2PKH),
            )
        });
        let p2sh_handle = thread::spawn(|| {
            EntityDataset::import(
                path,
                "p2sh",
                EntityFilter::AddressType(RawAddressType::P2SH),
            )
        });
        let p2wpkh_handle = thread::spawn(|| {
            EntityDataset::import(
                path,
                "p2wpkh",
                EntityFilter::AddressType(RawAddressType::P2WPKH),
            )
        });
        let p2wsh_handle = thread::spawn(|| {
            EntityDataset::import(
                path,
                "p2wsh",
                EntityFilter::AddressType(RawAddressType::P2WSH),
            )
        });
        let p2tr_handle = thread::spawn(|| {
            EntityDataset::import(
                path,
                "p2tr",
                EntityFilter::AddressType(RawAddressType::P2TR),
            )
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
    }
}

impl HeightDatasetsTrait for EntityDatasets {
    fn to_vec(&self) -> Vec<&(dyn HeightDatasetTrait + Send + Sync)> {
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
