mod all_metadata;
mod cohort;
mod cohort_metadata;

use std::thread;

use crate::parse::{RawAddressSize, RawAddressSplit, RawAddressType};

use self::{all_metadata::AllAddressesMetadataDataset, cohort::CohortDataset};

use super::{AnyDatasets, GenericDataset, MinInitialState};

pub struct AddressDatasets {
    min_initial_state: MinInitialState,

    metadata: AllAddressesMetadataDataset,

    plankton: CohortDataset,
    shrimp: CohortDataset,
    crab: CohortDataset,
    fish: CohortDataset,
    shark: CohortDataset,
    whale: CohortDataset,
    humpback: CohortDataset,
    megalodon: CohortDataset,

    p2pk: CohortDataset,
    p2pkh: CohortDataset,
    p2sh: CohortDataset,
    p2wpkh: CohortDataset,
    p2wsh: CohortDataset,
    p2tr: CohortDataset,
}

impl AddressDatasets {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        thread::scope(|scope| {
            let plankton_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    "plankton",
                    RawAddressSplit::Size(RawAddressSize::Plankton),
                )
            });
            let shrimp_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    "shrimp",
                    RawAddressSplit::Size(RawAddressSize::Shrimp),
                )
            });
            let crab_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    "crab",
                    RawAddressSplit::Size(RawAddressSize::Crab),
                )
            });
            let fish_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    "fish",
                    RawAddressSplit::Size(RawAddressSize::Fish),
                )
            });
            let shark_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    "shark",
                    RawAddressSplit::Size(RawAddressSize::Shark),
                )
            });
            let whale_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    "whale",
                    RawAddressSplit::Size(RawAddressSize::Whale),
                )
            });
            let humpback_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    "humpback",
                    RawAddressSplit::Size(RawAddressSize::Humpback),
                )
            });
            let megalodon_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    "megalodon",
                    RawAddressSplit::Size(RawAddressSize::Megalodon),
                )
            });

            let p2pk_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    "p2pk",
                    RawAddressSplit::Type(RawAddressType::P2PK),
                )
            });
            let p2pkh_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    "p2pkh",
                    RawAddressSplit::Type(RawAddressType::P2PKH),
                )
            });
            let p2sh_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    "p2sh",
                    RawAddressSplit::Type(RawAddressType::P2SH),
                )
            });
            let p2wpkh_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    "p2wpkh",
                    RawAddressSplit::Type(RawAddressType::P2WPKH),
                )
            });
            let p2wsh_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    "p2wsh",
                    RawAddressSplit::Type(RawAddressType::P2WSH),
                )
            });

            let p2tr = CohortDataset::import(
                parent_path,
                "p2tr",
                RawAddressSplit::Type(RawAddressType::P2TR),
            )?;

            let s = Self {
                min_initial_state: MinInitialState::default(),

                metadata: AllAddressesMetadataDataset::import(parent_path)?,

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
            };

            s.min_initial_state.compute_from_datasets(&s);

            Ok(s)
        })
    }
}

impl AnyDatasets for AddressDatasets {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_generic_dataset_vec(&self) -> Vec<&(dyn GenericDataset + Send + Sync)> {
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
            &self.metadata,
        ]
    }
}
