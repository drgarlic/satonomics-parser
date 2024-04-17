mod all_metadata;
mod cohort;
mod cohort_metadata;

use std::thread;

use crate::parse::{AddressSize, AddressSplit, AddressType};

use self::{all_metadata::AllAddressesMetadataDataset, cohort::CohortDataset};

use super::{AnyDataset, AnyDatasets, MinInitialState, ProcessedBlockData};

pub struct AddressDatasets {
    min_initial_state: MinInitialState,

    metadata: AllAddressesMetadataDataset,

    pub all: CohortDataset,

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
            let all_handle =
                scope.spawn(|| CohortDataset::import(parent_path, None, AddressSplit::All));

            let plankton_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    Some("plankton"),
                    AddressSplit::Size(AddressSize::Plankton),
                )
            });
            let shrimp_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    Some("shrimp"),
                    AddressSplit::Size(AddressSize::Shrimp),
                )
            });
            let crab_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    Some("crab"),
                    AddressSplit::Size(AddressSize::Crab),
                )
            });
            let fish_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    Some("fish"),
                    AddressSplit::Size(AddressSize::Fish),
                )
            });
            let shark_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    Some("shark"),
                    AddressSplit::Size(AddressSize::Shark),
                )
            });
            let whale_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    Some("whale"),
                    AddressSplit::Size(AddressSize::Whale),
                )
            });
            let humpback_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    Some("humpback"),
                    AddressSplit::Size(AddressSize::Humpback),
                )
            });
            let megalodon_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    Some("megalodon"),
                    AddressSplit::Size(AddressSize::Megalodon),
                )
            });

            let p2pk_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    Some("p2pk"),
                    AddressSplit::Type(AddressType::P2PK),
                )
            });
            let p2pkh_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    Some("p2pkh"),
                    AddressSplit::Type(AddressType::P2PKH),
                )
            });
            let p2sh_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    Some("p2sh"),
                    AddressSplit::Type(AddressType::P2SH),
                )
            });
            let p2wpkh_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    Some("p2wpkh"),
                    AddressSplit::Type(AddressType::P2WPKH),
                )
            });
            let p2wsh_handle = scope.spawn(|| {
                CohortDataset::import(
                    parent_path,
                    Some("p2wsh"),
                    AddressSplit::Type(AddressType::P2WSH),
                )
            });

            let p2tr = CohortDataset::import(
                parent_path,
                Some("p2tr"),
                AddressSplit::Type(AddressType::P2TR),
            )?;

            let mut s = Self {
                min_initial_state: MinInitialState::default(),

                metadata: AllAddressesMetadataDataset::import(parent_path)?,

                all: all_handle.join().unwrap()?,

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

            s.min_initial_state
                .consume(MinInitialState::compute_from_datasets(&s));

            Ok(s)
        })
    }

    pub fn insert_data(&mut self, processed_block_data: &ProcessedBlockData) {
        self.metadata.insert_data(processed_block_data);

        self.all.insert_data(processed_block_data);

        self.plankton.insert_data(processed_block_data);
        self.shrimp.insert_data(processed_block_data);
        self.crab.insert_data(processed_block_data);
        self.fish.insert_data(processed_block_data);
        self.shark.insert_data(processed_block_data);
        self.whale.insert_data(processed_block_data);
        self.humpback.insert_data(processed_block_data);
        self.megalodon.insert_data(processed_block_data);

        self.p2pk.insert_data(processed_block_data);
        self.p2pkh.insert_data(processed_block_data);
        self.p2sh.insert_data(processed_block_data);
        self.p2wpkh.insert_data(processed_block_data);
        self.p2wsh.insert_data(processed_block_data);
        self.p2tr.insert_data(processed_block_data);
    }
}

impl AnyDatasets for AddressDatasets {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_dataset_vec(&self) -> Vec<&(dyn AnyDataset + Send + Sync)> {
        vec![
            &self.all,
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

    fn to_mut_any_dataset_vec(&mut self) -> Vec<&mut dyn AnyDataset> {
        vec![
            &mut self.all,
            &mut self.plankton,
            &mut self.shrimp,
            &mut self.crab,
            &mut self.fish,
            &mut self.shark,
            &mut self.whale,
            &mut self.humpback,
            &mut self.megalodon,
            &mut self.p2pk,
            &mut self.p2pkh,
            &mut self.p2sh,
            &mut self.p2wpkh,
            &mut self.p2wsh,
            &mut self.p2tr,
            &mut self.metadata,
        ]
    }
}
