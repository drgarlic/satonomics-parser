use chrono::NaiveDate;
use itertools::Itertools;

use crate::{
    bitcoin::{btc_to_sats, sats_to_btc},
    datasets::{AnyDataset, PricePaidState, ProcessedBlockData, UnrealizedState},
    structs::{AnyDateMap, AnyHeightMap, BiMap},
};

use super::{AddressFilter, AddressSubDataset, LiquidityClassification};

pub struct AddressDataset {
    name: String,

    min_initial_first_unsafe_date: Option<NaiveDate>,
    min_initial_first_unsafe_height: Option<usize>,

    filter: AddressFilter,

    address_count: BiMap<usize>,
    all_dataset: AddressSubDataset,
    illiquid_dataset: AddressSubDataset,
    liquid_dataset: AddressSubDataset,
    highly_liquid_dataset: AddressSubDataset,
}

impl AddressDataset {
    pub fn import(
        parent_path: &str,
        name: &str,
        filter: AddressFilter,
    ) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{name}/{s}");

        let mut s = Self {
            name: name.to_owned(),
            min_initial_first_unsafe_date: None,
            min_initial_first_unsafe_height: None,
            filter,
            address_count: BiMap::new_on_disk_bin(&f("address_count")),
            all_dataset: AddressSubDataset::import(&f("full"))?,
            illiquid_dataset: AddressSubDataset::import(&f("illiquid"))?,
            liquid_dataset: AddressSubDataset::import(&f("liquid"))?,
            highly_liquid_dataset: AddressSubDataset::import(&f("highly_liquid"))?,
        };

        s.min_initial_first_unsafe_date = s.compute_min_initial_first_unsafe_date();
        s.min_initial_first_unsafe_height = s.compute_min_initial_first_unsafe_height();

        Ok(s)
    }

    pub fn to_vec(&self) -> Vec<&AddressSubDataset> {
        vec![
            &self.all_dataset,
            &self.illiquid_dataset,
            &self.liquid_dataset,
            &self.highly_liquid_dataset,
        ]
    }

    #[inline(always)]
    pub fn needs_sorted_address_data(&self, date: NaiveDate, height: usize) -> bool {
        self.needs_price_paid(date, height)
    }

    pub fn needs_utxos_metadata(&self, date: NaiveDate, height: usize) -> bool {
        self.to_vec()
            .iter()
            .any(|sub| !sub.utxos_metadata.are_date_and_height_safe(date, height))
    }

    pub fn needs_supply(&self, date: NaiveDate, height: usize) -> bool {
        self.to_vec()
            .iter()
            .any(|sub| !sub.supply.are_date_and_height_safe(date, height))
    }

    pub fn needs_price_paid(&self, date: NaiveDate, height: usize) -> bool {
        self.to_vec()
            .iter()
            .any(|sub| !sub.price_paid.are_date_and_height_safe(date, height))
    }

    fn needs_realized_data(&self, date: NaiveDate, height: usize) -> bool {
        self.to_vec()
            .iter()
            .any(|sub| !sub.realized.are_date_and_height_safe(date, height))
    }

    fn needs_unrealized_data(&self, date: NaiveDate, height: usize) -> bool {
        self.to_vec()
            .iter()
            .any(|sub| !sub.unrealized.are_date_and_height_safe(date, height))
    }

    fn insert_realized_data(&self, processed_block_data: &ProcessedBlockData) {
        let mut all_realized_profit = 0.0;
        let mut illiquid_realized_profit = 0.0;
        let mut liquid_realized_profit = 0.0;
        let mut highly_liquid_realized_profit = 0.0;

        let mut all_realized_loss = 0.0;
        let mut illiquid_realized_loss = 0.0;
        let mut liquid_realized_loss = 0.0;
        let mut highly_liquid_realized_loss = 0.0;

        processed_block_data
            .address_index_to_address_realized_data
            .values()
            .filter(|address_realized_data| {
                self.filter.check(
                    address_realized_data.previous_amount_opt.as_ref().unwrap(),
                    &address_realized_data.address_data_opt.unwrap().address_type,
                )
            })
            .for_each(|address_realized_data| {
                let address_data = address_realized_data.address_data_opt.unwrap();

                all_realized_profit += address_realized_data.profit;
                all_realized_loss += address_realized_data.loss;

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

        self.all_dataset.realized.insert(
            processed_block_data,
            all_realized_loss,
            all_realized_profit,
        );
        self.illiquid_dataset.realized.insert(
            processed_block_data,
            illiquid_realized_loss,
            illiquid_realized_profit,
        );
        self.liquid_dataset.realized.insert(
            processed_block_data,
            liquid_realized_loss,
            liquid_realized_profit,
        );
        self.highly_liquid_dataset.realized.insert(
            processed_block_data,
            highly_liquid_realized_loss,
            highly_liquid_realized_profit,
        );
    }

    fn insert_price_paid(
        &self,
        processed_block_data: &ProcessedBlockData,
        all_total_supply: u64,
        liquid_total_supply: u64,
        illiquid_total_supply: u64,
        highly_liquid_total_supply: u64,
    ) {
        let all_total_supply_in_btc = sats_to_btc(all_total_supply);
        let illiquid_total_supply_in_btc = sats_to_btc(illiquid_total_supply);
        let liquid_total_supply_in_btc = sats_to_btc(liquid_total_supply);
        let highly_liquid_total_supply_in_btc = sats_to_btc(highly_liquid_total_supply);

        let mut all_pp_state = PricePaidState::default();
        let mut illiquid_pp_state = PricePaidState::default();
        let mut liquid_pp_state = PricePaidState::default();
        let mut highly_liquid_pp_state = PricePaidState::default();

        processed_block_data
            .sorted_address_data
            .as_ref()
            .unwrap()
            .iter()
            .filter(|address_data| {
                self.filter
                    .check(&address_data.amount, &address_data.address_type)
            })
            .for_each(|address_data| {
                let sat_amount = address_data.amount;
                let price = address_data.mean_price_paid;

                let liquidity_classification =
                    LiquidityClassification::new(address_data.sent, address_data.received);

                let btc_amount = sats_to_btc(sat_amount);

                let split_amount = liquidity_classification.split(btc_amount);

                let illiquid_btc_amount = split_amount.illiquid;
                let liquid_btc_amount = split_amount.liquid;
                let highly_liquid_btc_amount = split_amount.highly_liquid;

                let illiquid_sat_amount = btc_to_sats(illiquid_btc_amount);
                let liquid_sat_amount = btc_to_sats(liquid_btc_amount);
                let highly_liquid_sat_amount = btc_to_sats(highly_liquid_btc_amount);

                all_pp_state.iterate(price, btc_amount, sat_amount, all_total_supply);
                illiquid_pp_state.iterate(
                    price,
                    illiquid_btc_amount,
                    illiquid_sat_amount,
                    illiquid_total_supply,
                );
                liquid_pp_state.iterate(
                    price,
                    liquid_btc_amount,
                    liquid_sat_amount,
                    liquid_total_supply,
                );
                highly_liquid_pp_state.iterate(
                    price,
                    highly_liquid_btc_amount,
                    highly_liquid_sat_amount,
                    highly_liquid_total_supply,
                );
            });

        self.all_dataset.price_paid.insert(
            processed_block_data,
            all_pp_state,
            all_total_supply_in_btc,
        );
        self.illiquid_dataset.price_paid.insert(
            processed_block_data,
            illiquid_pp_state,
            illiquid_total_supply_in_btc,
        );
        self.liquid_dataset.price_paid.insert(
            processed_block_data,
            liquid_pp_state,
            liquid_total_supply_in_btc,
        );
        self.highly_liquid_dataset.price_paid.insert(
            processed_block_data,
            highly_liquid_pp_state,
            highly_liquid_total_supply_in_btc,
        );
    }
}

impl AnyDataset for AddressDataset {
    fn insert_block_data(&self, processed_block_data: &ProcessedBlockData) {
        let &ProcessedBlockData {
            height,
            date,
            is_date_last_block,
            block_price,
            date_price,
            states,
            ..
        } = processed_block_data;

        let mut all_total_supply = 0;
        let mut illiquid_total_supply = 0;
        let mut liquid_total_supply = 0;
        let mut highly_liquid_total_supply = 0;

        let mut all_utxo_count = 0;
        let mut illiquid_utxo_count = 0;
        let mut liquid_utxo_count = 0;
        let mut highly_liquid_utxo_count = 0;

        let mut all_unrealized_height_state = UnrealizedState::default();
        let mut illiquid_unrealized_height_state = UnrealizedState::default();
        let mut liquid_unrealized_height_state = UnrealizedState::default();
        let mut highly_liquid_unrealized_height_state = UnrealizedState::default();

        let mut all_unrealized_date_state = {
            if is_date_last_block {
                Some(UnrealizedState::default())
            } else {
                None
            }
        };
        let mut illiquid_unrealized_date_state = {
            if is_date_last_block {
                Some(UnrealizedState::default())
            } else {
                None
            }
        };
        let mut liquid_unrealized_date_state = {
            if is_date_last_block {
                Some(UnrealizedState::default())
            } else {
                None
            }
        };
        let mut highly_liquid_unrealized_date_state = {
            if is_date_last_block {
                Some(UnrealizedState::default())
            } else {
                None
            }
        };

        let mut address_count = 0;

        let needs_unrealized_data = self.needs_unrealized_data(date, height);
        let needs_realized = self.needs_realized_data(date, height);
        let needs_price_paid = self.needs_price_paid(date, height);
        let needs_supply = needs_price_paid || self.needs_supply(date, height);
        let needs_utxos_metadata = self.needs_utxos_metadata(date, height);

        // dbg!(
        //     needs_unrealized_data,
        //     needs_realized,
        //     needs_price_paid,
        //     needs_supply,
        //     needs_utxos_metadata
        // );

        states
            .address_index_to_address_data
            .values()
            .filter(|address_data| {
                self.filter
                    .check(&address_data.amount, &address_data.address_type)
            })
            .for_each(|address_data| {
                address_count += 1;

                if needs_supply || needs_utxos_metadata || needs_unrealized_data {
                    let sat_amount = address_data.amount;
                    let utxo_count = address_data.outputs_len as usize;
                    let price = address_data.mean_price_paid;

                    let liquidity_classification =
                        LiquidityClassification::new(address_data.sent, address_data.received);

                    if needs_utxos_metadata {
                        let split_utxo_count = liquidity_classification.split(utxo_count as f64);

                        all_utxo_count += utxo_count;
                        illiquid_utxo_count += split_utxo_count.illiquid.round() as usize;
                        liquid_utxo_count += split_utxo_count.liquid.round() as usize;
                        highly_liquid_utxo_count += split_utxo_count.highly_liquid.round() as usize;
                    }

                    if needs_supply || needs_unrealized_data {
                        let btc_amount = sats_to_btc(sat_amount);

                        let split_amount = liquidity_classification.split(btc_amount);

                        let illiquid_btc_amount = split_amount.illiquid;
                        let liquid_btc_amount = split_amount.liquid;
                        let highly_liquid_btc_amount = split_amount.highly_liquid;

                        let illiquid_sat_amount = btc_to_sats(illiquid_btc_amount);
                        let liquid_sat_amount = btc_to_sats(liquid_btc_amount);
                        let highly_liquid_sat_amount = btc_to_sats(highly_liquid_btc_amount);

                        if needs_supply {
                            all_total_supply += sat_amount;
                            illiquid_total_supply += illiquid_sat_amount;
                            liquid_total_supply += liquid_sat_amount;
                            highly_liquid_total_supply += highly_liquid_sat_amount;
                        }

                        if needs_unrealized_data {
                            all_unrealized_height_state.iterate(
                                price,
                                block_price,
                                sat_amount,
                                btc_amount,
                            );
                            illiquid_unrealized_height_state.iterate(
                                price,
                                block_price,
                                illiquid_sat_amount,
                                illiquid_btc_amount,
                            );
                            liquid_unrealized_height_state.iterate(
                                price,
                                block_price,
                                liquid_sat_amount,
                                liquid_btc_amount,
                            );
                            highly_liquid_unrealized_height_state.iterate(
                                price,
                                block_price,
                                highly_liquid_sat_amount,
                                highly_liquid_btc_amount,
                            );

                            if is_date_last_block {
                                all_unrealized_date_state
                                    .as_mut()
                                    .unwrap()
                                    .iterate(price, date_price, sat_amount, btc_amount);
                                illiquid_unrealized_date_state.as_mut().unwrap().iterate(
                                    price,
                                    date_price,
                                    illiquid_sat_amount,
                                    illiquid_btc_amount,
                                );
                                liquid_unrealized_date_state.as_mut().unwrap().iterate(
                                    price,
                                    date_price,
                                    liquid_sat_amount,
                                    liquid_btc_amount,
                                );
                                highly_liquid_unrealized_date_state
                                    .as_mut()
                                    .unwrap()
                                    .iterate(
                                        price,
                                        date_price,
                                        highly_liquid_sat_amount,
                                        highly_liquid_btc_amount,
                                    );
                            }
                        }
                    }
                }
            });

        self.address_count.height.insert(height, address_count);

        if is_date_last_block {
            self.address_count.date.insert(date, address_count);
        }

        if needs_utxos_metadata {
            self.all_dataset
                .utxos_metadata
                .insert(processed_block_data, all_utxo_count);
            self.illiquid_dataset
                .utxos_metadata
                .insert(processed_block_data, illiquid_utxo_count);
            self.liquid_dataset
                .utxos_metadata
                .insert(processed_block_data, liquid_utxo_count);
            self.highly_liquid_dataset
                .utxos_metadata
                .insert(processed_block_data, highly_liquid_utxo_count);
        }

        if needs_supply {
            self.all_dataset
                .supply
                .insert(processed_block_data, all_total_supply);
            self.illiquid_dataset
                .supply
                .insert(processed_block_data, illiquid_total_supply);
            self.liquid_dataset
                .supply
                .insert(processed_block_data, liquid_total_supply);
            self.highly_liquid_dataset
                .supply
                .insert(processed_block_data, highly_liquid_total_supply);
        }

        if needs_unrealized_data {
            self.all_dataset.unrealized.insert(
                processed_block_data,
                all_unrealized_height_state,
                all_unrealized_date_state,
            );
            self.illiquid_dataset.unrealized.insert(
                processed_block_data,
                illiquid_unrealized_height_state,
                illiquid_unrealized_date_state,
            );
            self.liquid_dataset.unrealized.insert(
                processed_block_data,
                liquid_unrealized_height_state,
                liquid_unrealized_date_state,
            );
            self.highly_liquid_dataset.unrealized.insert(
                processed_block_data,
                highly_liquid_unrealized_height_state,
                highly_liquid_unrealized_date_state,
            );
        }

        if needs_realized {
            self.insert_realized_data(processed_block_data)
        }

        if needs_price_paid {
            self.insert_price_paid(
                processed_block_data,
                all_total_supply,
                liquid_total_supply,
                illiquid_total_supply,
                highly_liquid_total_supply,
            );
        }
    }

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        [
            self.all_dataset.to_any_height_map_vec(),
            self.illiquid_dataset.to_any_height_map_vec(),
            self.liquid_dataset.to_any_height_map_vec(),
            self.highly_liquid_dataset.to_any_height_map_vec(),
            vec![&self.address_count.height],
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }

    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        [
            self.all_dataset.to_any_date_map_vec(),
            self.illiquid_dataset.to_any_date_map_vec(),
            self.liquid_dataset.to_any_date_map_vec(),
            self.highly_liquid_dataset.to_any_date_map_vec(),
            vec![&self.address_count.date],
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }

    fn get_min_initial_first_unsafe_date(&self) -> &Option<NaiveDate> {
        &self.min_initial_first_unsafe_date
    }

    fn get_min_initial_first_unsafe_height(&self) -> &Option<usize> {
        &self.min_initial_first_unsafe_height
    }

    fn name(&self) -> &str {
        &self.name
    }
}
