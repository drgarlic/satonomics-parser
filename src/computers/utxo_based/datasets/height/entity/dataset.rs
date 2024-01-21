use std::{collections::BTreeMap, thread};

use itertools::Itertools;
use ordered_float::OrderedFloat;

use crate::{
    bitcoin::{btc_to_sats, sats_to_btc},
    computers::utxo_based::{
        datasets::height::{PriceDataset, PriceDatasetInsertData},
        HeightDatasetTrait, ProcessedData,
    },
    structs::{AnyHeightMap, HeightMap},
};

use super::{EntityFilter, LiquidityClassification};

pub struct EntitySubDataset {
    price_dataset: PriceDataset,
    height_to_address_count: HeightMap<usize>,
}

impl EntitySubDataset {
    pub fn import(path: &str, sub_name: &str, name: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{path}/entities/{sub_name}/ height_to_{name}_{s}.json");

        Ok(Self {
            price_dataset: PriceDataset::import(path, name)?,
            height_to_address_count: HeightMap::new(&f("address_count")),
        })
    }

    pub fn insert(&self, price_dataset_insert_data: PriceDatasetInsertData) {
        self.height_to_address_count.insert(
            price_dataset_insert_data.height,
            price_dataset_insert_data.price_to_amount.len(),
        );

        self.price_dataset.insert(price_dataset_insert_data);
    }

    pub fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        [
            self.price_dataset.to_vec(),
            vec![&self.height_to_address_count],
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }
}

pub struct EntityDataset {
    filter: EntityFilter,

    full_dataset: EntitySubDataset,
    illiquid_dataset: EntitySubDataset,
    liquid_dataset: EntitySubDataset,
    highly_liquid_dataset: EntitySubDataset,
}

impl EntityDataset {
    pub fn import(path: &str, name: &str, filter: EntityFilter) -> color_eyre::Result<Self> {
        Ok(Self {
            filter,
            full_dataset: EntitySubDataset::import(path, "full", name)?,
            illiquid_dataset: EntitySubDataset::import(path, "illiquid", name)?,
            liquid_dataset: EntitySubDataset::import(path, "liquid", name)?,
            highly_liquid_dataset: EntitySubDataset::import(path, "highly_liquid", name)?,
        })
    }
}

impl HeightDatasetTrait for EntityDataset {
    fn insert(&self, processed_data: &ProcessedData) {
        let &ProcessedData {
            states,
            price,
            height,
            address_index_to_address_realized_data,
            ..
        } = processed_data;

        let address_index_to_address_data = &states.address_index_to_address_data;

        let mut full_realized_profit = 0.0;
        let mut illiquid_realized_profit = 0.0;
        let mut liquid_realized_profit = 0.0;
        let mut highly_liquid_realized_profit = 0.0;

        let mut full_realized_loss = 0.0;
        let mut illiquid_realized_loss = 0.0;
        let mut liquid_realized_loss = 0.0;
        let mut highly_liquid_realized_loss = 0.0;

        address_index_to_address_realized_data
            .iter()
            .map(|(address_index, address_realized_data)| {
                let address_data = address_index_to_address_data.get(address_index).unwrap();

                let previous_amount = address_data.amount + address_realized_data.sent
                    - address_realized_data.received;

                (address_data, previous_amount, address_realized_data)
            })
            .filter(|(address_data, previous_amount, _)| {
                self.filter
                    .includes(previous_amount, &address_data.address_type)
            })
            .for_each(|(address_data, _, address_realized_data)| {
                full_realized_profit += address_realized_data.profit;
                full_realized_loss += address_realized_data.loss;

                // Realized == previous amount
                // If a whale sent all its sats to another address at a loss, it's the whale that realized the loss not the empty adress
                let previous_sent = address_data.sent - address_realized_data.sent;
                let previous_received = address_data.received - address_realized_data.received;

                let split_profit = LiquidityClassification::new(previous_sent, previous_received)
                    .split(address_realized_data.profit as f64);

                illiquid_realized_profit += split_profit.illiquid as f32;
                liquid_realized_profit += split_profit.liquid as f32;
                highly_liquid_realized_profit += split_profit.highly_liquid as f32;

                let split_loss = LiquidityClassification::new(previous_sent, previous_received)
                    .split(address_realized_data.loss as f64);

                illiquid_realized_loss += split_loss.illiquid as f32;
                liquid_realized_loss += split_loss.liquid as f32;
                highly_liquid_realized_loss += split_loss.highly_liquid as f32;
            });

        let mut full_total_supply = 0;
        let mut illiquid_total_supply = 0;
        let mut liquid_total_supply = 0;
        let mut highly_liquid_total_supply = 0;

        let mut full_utxo_count = 0;
        let mut illiquid_utxo_count = 0;
        let mut liquid_utxo_count = 0;
        let mut highly_liquid_utxo_count = 0;

        let (
            full_price_to_amount,
            (illiquid_price_to_amount, (liquid_price_to_amount, highly_liquid_price_to_amount)),
        ): (
            BTreeMap<_, _>,
            (BTreeMap<_, _>, (BTreeMap<_, _>, BTreeMap<_, _>)),
        ) = address_index_to_address_data
            .values()
            .filter(|address_data| {
                self.filter
                    .includes(&address_data.amount, &address_data.address_type)
            })
            .map(|address_data| {
                let amount = address_data.amount;
                let utxo_count = address_data.outputs_len as usize;
                let mean_price_paid = OrderedFloat(address_data.mean_price_paid);

                let liquidity_classification =
                    LiquidityClassification::new(address_data.sent, address_data.received);

                let split_utxo_count = liquidity_classification.split(utxo_count as f64);

                full_utxo_count += utxo_count;
                illiquid_utxo_count += split_utxo_count.illiquid.round() as usize;
                liquid_utxo_count += split_utxo_count.liquid.round() as usize;
                highly_liquid_utxo_count += split_utxo_count.highly_liquid.round() as usize;

                let split_amount = liquidity_classification.split(sats_to_btc(amount));

                let illiquid_sat_amount = btc_to_sats(split_amount.illiquid);
                let liquid_sat_amount = btc_to_sats(split_amount.liquid);
                let highly_liquid_sat_amount = btc_to_sats(split_amount.highly_liquid);

                full_total_supply += amount;
                illiquid_total_supply += illiquid_sat_amount;
                liquid_total_supply += liquid_sat_amount;
                highly_liquid_total_supply += highly_liquid_sat_amount;

                (
                    (mean_price_paid, amount),
                    (
                        (mean_price_paid, illiquid_sat_amount),
                        (
                            (mean_price_paid, liquid_sat_amount),
                            (mean_price_paid, highly_liquid_sat_amount),
                        ),
                    ),
                )
            })
            .unzip();

        thread::scope(|scope| {
            scope.spawn(|| {
                self.full_dataset.insert(PriceDatasetInsertData {
                    height,
                    price,
                    price_to_amount: full_price_to_amount,
                    realized_loss: full_realized_loss,
                    realized_profit: full_realized_profit,
                    total_supply: full_total_supply,
                    utxo_count: full_utxo_count,
                })
            });
            scope.spawn(|| {
                self.illiquid_dataset.insert(PriceDatasetInsertData {
                    height,
                    price,
                    price_to_amount: illiquid_price_to_amount,
                    realized_loss: illiquid_realized_loss,
                    realized_profit: illiquid_realized_profit,
                    total_supply: illiquid_total_supply,
                    utxo_count: illiquid_utxo_count,
                })
            });
            scope.spawn(|| {
                self.liquid_dataset.insert(PriceDatasetInsertData {
                    height,
                    price,
                    price_to_amount: liquid_price_to_amount,
                    realized_loss: liquid_realized_loss,
                    realized_profit: liquid_realized_profit,
                    total_supply: liquid_total_supply,
                    utxo_count: liquid_utxo_count,
                })
            });
            scope.spawn(|| {
                self.highly_liquid_dataset.insert(PriceDatasetInsertData {
                    height,
                    price,
                    price_to_amount: highly_liquid_price_to_amount,
                    realized_loss: highly_liquid_realized_loss,
                    realized_profit: highly_liquid_realized_profit,
                    total_supply: highly_liquid_total_supply,
                    utxo_count: highly_liquid_utxo_count,
                })
            });
        });
    }

    fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        [
            self.full_dataset.to_vec(),
            self.illiquid_dataset.to_vec(),
            self.liquid_dataset.to_vec(),
            self.highly_liquid_dataset.to_vec(),
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }
}
