use chrono::NaiveDate;

use crate::{
    bitcoin::sats_to_btc,
    datasets::{
        AnyDataset, PricePaidState, PricePaidSubDataset, ProcessedBlockData, RealizedState,
        RealizedSubDataset, SupplyState, SupplySubDataset, UTXOsMetadataState,
        UTXOsMetadataSubDataset, UnrealizedState, UnrealizedSubDataset,
    },
    parse::{reverse_date_index, AnyDateMap, AnyHeightMap, BlockData},
};

use super::UTXOFilter;

pub struct UTXODataset {
    min_initial_first_unsafe_date: Option<NaiveDate>,
    min_initial_first_unsafe_height: Option<usize>,
    filter: UTXOFilter,
    price_paid: PricePaidSubDataset,
    realized: RealizedSubDataset,
    supply: SupplySubDataset,
    unrealized: UnrealizedSubDataset,
    utxos_metadata: UTXOsMetadataSubDataset,
}

impl UTXODataset {
    pub fn import(
        parent_path: &str,
        name: Option<&str>,
        range: UTXOFilter,
    ) -> color_eyre::Result<Self> {
        let folder_path = {
            if let Some(name) = name {
                format!("{parent_path}/{name}")
            } else {
                parent_path.to_owned()
            }
        };

        let mut s = Self {
            min_initial_first_unsafe_date: None,
            min_initial_first_unsafe_height: None,
            filter: range,
            price_paid: PricePaidSubDataset::import(&folder_path)?,
            realized: RealizedSubDataset::import(&folder_path)?,
            supply: SupplySubDataset::import(&folder_path)?,
            unrealized: UnrealizedSubDataset::import(&folder_path)?,
            utxos_metadata: UTXOsMetadataSubDataset::import(&folder_path)?,
        };

        s.min_initial_first_unsafe_date = s.compute_min_initial_first_unsafe_date();
        s.min_initial_first_unsafe_height = s.compute_min_initial_first_unsafe_height();

        Ok(s)
    }

    pub fn needs_sorted_block_data_vec(&self, date: NaiveDate, height: usize) -> bool {
        self.needs_price_paid(date, height)
    }

    fn needs_price_paid(&self, date: NaiveDate, height: usize) -> bool {
        !self.price_paid.are_date_and_height_safe(date, height)
    }

    fn needs_unrealized(&self, date: NaiveDate, height: usize) -> bool {
        !self.unrealized.are_date_and_height_safe(date, height)
    }

    fn needs_realized(&self, date: NaiveDate, height: usize) -> bool {
        !self.realized.are_date_and_height_safe(date, height)
    }
}

impl AnyDataset for UTXODataset {
    fn insert_block_data(&self, processed_block_data: &ProcessedBlockData) {
        let &ProcessedBlockData {
            block_path_to_spent_value,
            block_price,
            is_date_last_block,
            date,
            height,
            date_price,
            states,
            ..
        } = processed_block_data;

        let date_data_vec = &states.date_data_vec;

        let mut supply_state = SupplyState::default();
        let mut utxos_metadata_state = UTXOsMetadataState::default();

        let mut pp_state = PricePaidState::default();

        let mut unrealized_height_state = UnrealizedState::default();
        let mut unrealized_date_state = UnrealizedState::default();

        let date_data_vec_len = states.date_data_vec.len() as u16;

        let needs_price_paid = self.needs_price_paid(date, height);
        let needs_unrealized = self.needs_unrealized(date, height);
        let needs_realized = self.needs_realized(date, height);

        date_data_vec
            .iter()
            .filter(|date_data| {
                self.filter
                    .check(&date_data.reverse_index(date_data_vec_len), &date_data.year)
            })
            .flat_map(|date_data| &date_data.blocks)
            .for_each(|block_data| {
                let price = block_data.price;
                let sat_amount = block_data.amount;
                let btc_amount = sats_to_btc(sat_amount);

                supply_state.total_supply += sat_amount;
                utxos_metadata_state.count += block_data.spendable_outputs as usize;

                if needs_unrealized {
                    unrealized_height_state.iterate(price, block_price, sat_amount, btc_amount);

                    if is_date_last_block {
                        unrealized_date_state.iterate(price, date_price, sat_amount, btc_amount);
                    }
                }
            });

        let total_supply = supply_state.total_supply;

        self.supply.insert(processed_block_data, &supply_state);

        self.utxos_metadata
            .insert(processed_block_data, &utxos_metadata_state);

        if needs_unrealized {
            self.unrealized.insert(
                processed_block_data,
                &unrealized_height_state,
                &unrealized_date_state,
            );
        }

        if needs_price_paid {
            processed_block_data
                .sorted_block_data_vec
                .as_ref()
                .unwrap()
                .iter()
                .filter(|sorted_block_data| {
                    self.filter.check(
                        &sorted_block_data.reversed_date_index,
                        &sorted_block_data.year,
                    )
                })
                .for_each(|sorted_block_data| {
                    let block_data = sorted_block_data.block_data;

                    let price = block_data.price;
                    let sat_amount = block_data.amount;
                    let btc_amount = sats_to_btc(sat_amount);

                    pp_state.iterate(price, btc_amount, sat_amount, total_supply);
                });

            self.price_paid.insert(processed_block_data, &pp_state);
        }

        if needs_realized {
            let mut realized_state = RealizedState::default();

            block_path_to_spent_value
                .iter()
                .map(|(block_path, value)| {
                    let date_data = date_data_vec.get(block_path.date_index as usize).unwrap();
                    (block_path, date_data, value)
                })
                .filter(|(block_path, date_data, _)| {
                    self.filter.check(
                        &reverse_date_index(block_path.date_index, date_data_vec_len),
                        &date_data.year,
                    )
                })
                .for_each(|(block_path, date_data, value)| {
                    let BlockData {
                        price: previous_price,
                        ..
                    } = date_data
                        .blocks
                        .get(block_path.block_index as usize)
                        .unwrap();

                    let previous_dollar_amount = *previous_price as f64 * sats_to_btc(*value);
                    let current_dollar_amount = block_price as f64 * sats_to_btc(*value);

                    if previous_dollar_amount < current_dollar_amount {
                        realized_state.realized_profit +=
                            (current_dollar_amount - previous_dollar_amount) as f32
                    } else if current_dollar_amount < previous_dollar_amount {
                        realized_state.realized_loss +=
                            (previous_dollar_amount - current_dollar_amount) as f32
                    }
                });

            self.realized.insert(processed_block_data, &realized_state);
        }
    }

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        [
            self.price_paid.to_any_height_map_vec(),
            self.realized.to_any_height_map_vec(),
            self.supply.to_any_height_map_vec(),
            self.unrealized.to_any_height_map_vec(),
            self.utxos_metadata.to_any_height_map_vec(),
        ]
        .iter()
        .flatten()
        .copied()
        .collect()
    }

    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        [
            self.price_paid.to_any_date_map_vec(),
            self.realized.to_any_date_map_vec(),
            self.supply.to_any_date_map_vec(),
            self.unrealized.to_any_date_map_vec(),
            self.utxos_metadata.to_any_date_map_vec(),
        ]
        .iter()
        .flatten()
        .copied()
        .collect()
    }

    fn get_min_initial_first_unsafe_date(&self) -> &Option<NaiveDate> {
        &self.min_initial_first_unsafe_date
    }

    fn get_min_initial_first_unsafe_height(&self) -> &Option<usize> {
        &self.min_initial_first_unsafe_height
    }
}
