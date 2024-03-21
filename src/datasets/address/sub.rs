use itertools::Itertools;

use crate::{
    datasets::{
        PricePaidSubDataset, RealizedSubDataset, SupplySubDataset, UTXOsMetadataSubDataset,
        UnrealizedSubDataset,
    },
    parse::{AnyDateMap, AnyHeightMap},
};

pub struct AddressSubDataset {
    pub price_paid: PricePaidSubDataset,
    pub realized: RealizedSubDataset,
    pub supply: SupplySubDataset,
    pub unrealized: UnrealizedSubDataset,
    pub utxos_metadata: UTXOsMetadataSubDataset,
}

impl AddressSubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        Ok(Self {
            price_paid: PricePaidSubDataset::import(parent_path)?,
            realized: RealizedSubDataset::import(parent_path)?,
            supply: SupplySubDataset::import(parent_path)?,
            unrealized: UnrealizedSubDataset::import(parent_path)?,
            utxos_metadata: UTXOsMetadataSubDataset::import(parent_path)?,
        })
    }

    pub fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        [
            self.price_paid.to_any_height_map_vec(),
            self.realized.to_any_height_map_vec(),
            self.supply.to_any_height_map_vec(),
            self.unrealized.to_any_height_map_vec(),
            self.utxos_metadata.to_any_height_map_vec(),
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }

    pub fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        [
            self.price_paid.to_any_date_map_vec(),
            self.realized.to_any_date_map_vec(),
            self.supply.to_any_date_map_vec(),
            self.unrealized.to_any_date_map_vec(),
            self.utxos_metadata.to_any_date_map_vec(),
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }
}
