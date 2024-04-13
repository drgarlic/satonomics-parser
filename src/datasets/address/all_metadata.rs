use crate::{
    datasets::{AnyDataset, GenericDataset, MinInitialState, ProcessedBlockData},
    parse::{AnyBiMap, AnyHeightMap, BiMap},
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

            total_addresses_created: BiMap::new_bin(&f("total_addresses_created")),
            total_empty_addresses: BiMap::new_bin(&f("total_empty_addresses")),
        };

        s.min_initial_state
            .eat(MinInitialState::compute_from_dataset(&s));

        Ok(s)
    }
}

impl GenericDataset for AllAddressesMetadataDataset {
    fn insert_block_data(&self, processed_block_data: &ProcessedBlockData) {
        let &ProcessedBlockData {
            databases, height, ..
        } = processed_block_data;

        self.total_addresses_created
            .height
            .insert(height, *databases.address_to_address_index.metadata.len);

        self.total_empty_addresses.height.insert(
            height,
            *databases.address_index_to_empty_address_data.metadata.len,
        );
    }
}

impl AnyDataset for AllAddressesMetadataDataset {
    // fn compute(
    //     &self,
    //     &ExportData {
    //         convert_last_height_to_date,
    //         ..
    //     }: &ExportData,
    // ) {
    // self.total_addresses_created
    //     .compute_date(convert_last_height_to_date);
    // self.total_empty_addresses
    //     .compute_date(convert_last_height_to_date);
    // }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![
            &self.total_addresses_created.height,
            &self.total_empty_addresses.height,
        ]
    }

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.total_addresses_created, &self.total_empty_addresses]
    }
}
