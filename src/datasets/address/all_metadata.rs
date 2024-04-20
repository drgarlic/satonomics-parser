use crate::{
    datasets::{AnyDataset, MinInitialState, ProcessedBlockData},
    parse::{AnyBiMap, BiMap},
};

pub struct AllAddressesMetadataDataset {
    min_initial_state: MinInitialState,

    total_addresses_created: BiMap<u32>,
    total_empty_addresses: BiMap<u32>,
}

impl AllAddressesMetadataDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_state: MinInitialState::default(),

            total_addresses_created: BiMap::new_bin(1, &f("total_addresses_created")),
            total_empty_addresses: BiMap::new_bin(1, &f("total_empty_addresses")),
        };

        s.min_initial_state
            .consume(MinInitialState::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn insert_data(&mut self, processed_block_data: &ProcessedBlockData) {
        let &ProcessedBlockData {
            databases,
            height,
            date,
            is_date_last_block,
            ..
        } = processed_block_data;

        let total_addresses_created = self
            .total_addresses_created
            .height
            .insert(height, *databases.address_to_address_index.metadata.len);

        let total_empty_addresses = self.total_empty_addresses.height.insert(
            height,
            *databases.address_index_to_empty_address_data.metadata.len,
        );

        if is_date_last_block {
            self.total_addresses_created
                .date
                .insert(date, total_addresses_created);

            self.total_empty_addresses
                .date
                .insert(date, total_empty_addresses);
        }
    }
}

impl AnyDataset for AllAddressesMetadataDataset {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.total_addresses_created, &self.total_empty_addresses]
    }

    fn to_any_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![
            &mut self.total_addresses_created,
            &mut self.total_empty_addresses,
        ]
    }
}
