use std::thread;

use chrono::NaiveDate;

use crate::{
    datasets::{AnyDataset, AnyDatasets, ProcessedBlockData},
    parse::{AnyDateMap, AnyHeightMap, BiMap, RawAddressSize, RawAddressSplit, RawAddressType},
};

use super::CohortDataset;

pub struct AllAddressesMetadataDataset {
    min_initial_first_unsafe_date: Option<NaiveDate>,
    min_initial_first_unsafe_height: Option<usize>,

    total_addresses_created: BiMap<usize>,
    total_empty_addresses: BiMap<usize>,
}

impl AllAddressesMetadataDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            total_addresses_created: BiMap::new_on_disk_bin(&f("total_addresses_created")),
            total_empty_addresses: BiMap::new_on_disk_bin(&f("total_empty_addresses")),

            min_initial_first_unsafe_date: None,
            min_initial_first_unsafe_height: None,
        };

        s.min_initial_first_unsafe_date = s.compute_min_initial_first_unsafe_date();
        s.min_initial_first_unsafe_height = s.compute_min_initial_first_unsafe_height();

        Ok(s)
    }
}

impl AnyDataset for AllAddressesMetadataDataset {
    fn insert_block_data(&self, processed_block_data: &ProcessedBlockData) {
        let &ProcessedBlockData {
            databases,
            height,
            date,
            is_date_last_block,
            ..
        } = processed_block_data;

        let total_addresses_created = *databases.raw_address_to_address_index.metadata.len as usize;
        let total_empty_addresses =
            *databases.address_index_to_empty_address_data.metadata.len as usize;

        self.total_addresses_created
            .height
            .insert(height, total_addresses_created);

        self.total_empty_addresses
            .height
            .insert(height, total_empty_addresses);

        if is_date_last_block {
            self.total_addresses_created
                .date
                .insert(date, total_addresses_created);

            self.total_empty_addresses
                .date
                .insert(date, total_empty_addresses);
        }
    }

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![
            &self.total_addresses_created.height,
            &self.total_empty_addresses.height,
        ]
    }

    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![
            &self.total_addresses_created.date,
            &self.total_empty_addresses.date,
        ]
    }

    fn get_min_initial_first_unsafe_date(&self) -> &Option<NaiveDate> {
        &self.min_initial_first_unsafe_date
    }

    fn get_min_initial_first_unsafe_height(&self) -> &Option<usize> {
        &self.min_initial_first_unsafe_height
    }
}

pub struct AddressDatasets {
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

            Ok(Self {
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
            &self.metadata,
        ]
    }

    fn name<'a>() -> &'a str {
        "address"
    }
}
