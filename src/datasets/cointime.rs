use crate::{
    bitcoin::{
        sats_to_btc, ONE_DAY_IN_BLOCK_TIME, THREE_MONTHS_IN_BLOCK_TIME, TWO_WEEKS_IN_BLOCK_TIME,
    },
    parse::{AnyBiMap, BiMap},
    utils::{ONE_DAY_IN_DAYS, ONE_YEAR_IN_DAYS, THREE_MONTHS_IN_DAYS, TWO_WEEK_IN_DAYS},
};

use super::{
    AddressDatasets, AnyDataset, MinInitialState, MiningDataset, ProcessedBlockData,
    TransactionDataset,
};

pub struct CointimeDataset {
    min_initial_state: MinInitialState,

    pub active_cap: BiMap<f32>,
    pub active_price: BiMap<f32>,
    pub active_supply: BiMap<f32>,
    pub active_supply_3m_net_change: BiMap<f32>,
    pub active_supply_net_change: BiMap<f32>,
    pub activity_to_vaultedness_ratio: BiMap<f32>,
    pub coinblocks_created: BiMap<f32>,
    pub coinblocks_destroyed: BiMap<f32>,
    pub coinblocks_stored: BiMap<f32>,
    pub cointime_adjusted_velocity: BiMap<f32>,
    pub cointime_adjusted_yearly_inflation_rate: BiMap<f32>,
    pub cointime_cap: BiMap<f32>,
    pub cointime_price: BiMap<f32>,
    pub cointime_value_created: BiMap<f32>,
    pub cointime_value_destroyed: BiMap<f32>,
    pub cointime_value_stored: BiMap<f32>,
    pub concurrent_liveliness: BiMap<f32>,
    pub concurrent_liveliness_2w_median: BiMap<f32>,
    pub cumulative_coinblocks_created: BiMap<f32>,
    pub cumulative_coinblocks_destroyed: BiMap<f32>,
    pub cumulative_coinblocks_stored: BiMap<f32>,
    pub investor_cap: BiMap<f32>,
    pub investorness: BiMap<f32>,
    pub liveliness: BiMap<f32>,
    pub liveliness_net_change: BiMap<f32>,
    pub liveliness_net_change_2w_median: BiMap<f32>,
    pub producerness: BiMap<f32>,
    pub thermo_cap: BiMap<f32>,
    pub thermo_cap_to_investor_cap_ratio: BiMap<f32>,
    pub total_cointime_value_created: BiMap<f32>,
    pub total_cointime_value_destroyed: BiMap<f32>,
    pub total_cointime_value_stored: BiMap<f32>,
    pub true_market_deviation: BiMap<f32>,
    pub true_market_mean: BiMap<f32>,
    pub true_market_net_unrealized_profit_and_loss: BiMap<f32>,
    pub vaulted_cap: BiMap<f32>,
    pub vaulted_price: BiMap<f32>,
    pub vaulted_supply: BiMap<f32>,
    pub vaultedness: BiMap<f32>,
    pub vaulting_rate: BiMap<f32>,
}

impl CointimeDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_state: MinInitialState::default(),

            coinblocks_destroyed: BiMap::new_bin(1, &f("coinblocks_destroyed")),
            cumulative_coinblocks_destroyed: BiMap::new_bin(
                1,
                &f("cumulative_coinblocks_destroyed"),
            ),
            coinblocks_created: BiMap::new_bin(1, &f("coinblocks_created")),
            cumulative_coinblocks_created: BiMap::new_bin(1, &f("cumulative_coinblocks_created")),
            coinblocks_stored: BiMap::new_bin(1, &f("coinblocks_stored")),
            cumulative_coinblocks_stored: BiMap::new_bin(1, &f("cumulative_coinblocks_stored")),
            liveliness: BiMap::_new_bin(1, &f("liveliness"), 2),
            vaultedness: BiMap::new_bin(1, &f("vaultedness")),
            activity_to_vaultedness_ratio: BiMap::new_bin(1, &f("activity_to_vaultedness_ratio")),
            concurrent_liveliness: BiMap::_new_bin(1, &f("concurrent_liveliness"), 2),
            concurrent_liveliness_2w_median: BiMap::new_bin(
                1,
                &f("concurrent_liveliness_2w_median"),
            ),
            liveliness_net_change: BiMap::new_bin(1, &f("liveliness_net_change")),
            liveliness_net_change_2w_median: BiMap::new_bin(
                1,
                &f("liveliness_net_change_2w_median"),
            ),
            vaulted_supply: BiMap::new_bin(1, &f("vaulted_supply")),
            vaulting_rate: BiMap::new_bin(1, &f("vaulting_rate")),
            active_supply: BiMap::_new_bin(1, &f("active_supply"), 2),
            active_supply_net_change: BiMap::new_bin(1, &f("active_supply_net_change")),
            active_supply_3m_net_change: BiMap::new_bin(1, &f("active_supply_3m_net_change")),
            cointime_adjusted_yearly_inflation_rate: BiMap::new_bin(
                1,
                &f("cointime_adjusted_yearly_inflation_rate"),
            ),
            cointime_adjusted_velocity: BiMap::new_bin(1, &f("cointime_adjusted_velocity")),
            thermo_cap: BiMap::new_bin(1, &f("thermo_cap")),
            investor_cap: BiMap::new_bin(1, &f("investor_cap")),
            thermo_cap_to_investor_cap_ratio: BiMap::new_bin(
                1,
                &f("thermo_cap_to_investor_cap_ratio"),
            ),
            active_price: BiMap::new_bin(1, &f("active_price")),
            active_cap: BiMap::new_bin(1, &f("active_cap")),
            vaulted_price: BiMap::new_bin(1, &f("vaulted_price")),
            vaulted_cap: BiMap::new_bin(1, &f("vaulted_cap")),
            true_market_mean: BiMap::new_bin(1, &f("true_market_mean")),
            true_market_deviation: BiMap::new_bin(1, &f("true_market_deviation")),
            true_market_net_unrealized_profit_and_loss: BiMap::new_bin(
                1,
                &f("true_market_net_unrealized_profit_and_loss"),
            ),
            investorness: BiMap::new_bin(1, &f("investorness")),
            producerness: BiMap::new_bin(1, &f("producerness")),
            cointime_value_created: BiMap::new_bin(1, &f("cointime_value_created")),
            cointime_value_destroyed: BiMap::new_bin(1, &f("cointime_value_destroyed")),
            cointime_value_stored: BiMap::new_bin(1, &f("cointime_value_stored")),
            total_cointime_value_created: BiMap::new_bin(1, &f("total_cointime_value_created")),
            total_cointime_value_destroyed: BiMap::new_bin(1, &f("total_cointime_value_destroyed")),
            total_cointime_value_stored: BiMap::new_bin(1, &f("total_cointime_value_stored")),
            cointime_price: BiMap::new_bin(1, &f("cointime_price")),
            cointime_cap: BiMap::new_bin(1, &f("cointime_cap")),
        };

        s.min_initial_state
            .consume(MinInitialState::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn insert_data(
        &mut self,
        &ProcessedBlockData {
            height,
            date,
            satblocks_destroyed,
            block_price,
            date_price,
            date_blocks_range,
            is_date_last_block,
            ..
        }: &ProcessedBlockData,
        address_datasets: &AddressDatasets,
        mining_dataset: &MiningDataset,
        transaction_dataset: &TransactionDataset,
    ) {
        let circulating_supply_map = &address_datasets.all.all.supply.total;
        let circulating_supply = circulating_supply_map.height.get(&height).unwrap();

        let realized_cap_map = &address_datasets.all.all.price_paid.realized_cap;
        let realized_cap = realized_cap_map.height.get(&height).unwrap();

        let realized_price_map = &address_datasets.all.all.price_paid.realized_price;
        let realized_price = realized_price_map.height.get(&height).unwrap();

        let yearly_inflation_rate_map = &mining_dataset.yearly_inflation_rate;
        let yearly_inflation_rate = yearly_inflation_rate_map.height.get(&height).unwrap();

        let annualized_transaction_volume_map = &transaction_dataset.annualized_volume;
        let annualized_transaction_volume = annualized_transaction_volume_map
            .height
            .get(&height)
            .unwrap();

        let cumulative_subsidy_in_dollars_map = &mining_dataset.cumulative_subsidy_in_dollars;
        let cumulative_subsidy_in_dollars = cumulative_subsidy_in_dollars_map
            .height
            .get(&height)
            .unwrap();

        let coinblocks_destroyed = self
            .coinblocks_destroyed
            .height
            .insert(height, sats_to_btc(satblocks_destroyed));

        let cumulative_coinblocks_destroyed = self
            .cumulative_coinblocks_destroyed
            .height
            .insert_cumulative(height, &self.coinblocks_destroyed.height);

        let coinblocks_created = self
            .coinblocks_created
            .height
            .insert(height, circulating_supply);

        let cumulative_coinblocks_created = self
            .cumulative_coinblocks_created
            .height
            .insert_cumulative(height, &self.coinblocks_created.height);

        let coinblocks_stored = self
            .coinblocks_stored
            .height
            .insert(height, coinblocks_created - coinblocks_destroyed);

        let cumulative_coinblocks_stored = self
            .cumulative_coinblocks_stored
            .height
            .insert_cumulative(height, &self.coinblocks_stored.height);

        let liveliness = self.liveliness.height.insert(
            height,
            cumulative_coinblocks_destroyed / cumulative_coinblocks_created,
        );

        let vaultedness = self.vaultedness.height.insert(height, 1.0 - liveliness);

        let activity_to_vaultedness_ratio = self
            .activity_to_vaultedness_ratio
            .height
            .insert(height, liveliness / vaultedness);

        let concurrent_liveliness = self
            .concurrent_liveliness
            .height
            .insert(height, coinblocks_destroyed / coinblocks_created);

        self.concurrent_liveliness_2w_median.height.insert_median(
            height,
            &self.concurrent_liveliness.height,
            TWO_WEEKS_IN_BLOCK_TIME,
        );

        self.liveliness_net_change.height.insert_net_change(
            height,
            &self.liveliness.height,
            ONE_DAY_IN_BLOCK_TIME,
        );

        self.liveliness_net_change_2w_median
            .height
            .insert_net_change(height, &self.liveliness.height, TWO_WEEKS_IN_BLOCK_TIME);

        let vaulted_supply = self
            .vaulted_supply
            .height
            .insert(height, vaultedness * circulating_supply);

        let vaulting_rate = self
            .vaulting_rate
            .height
            .insert(height, vaulted_supply * ONE_YEAR_IN_DAYS as f32);

        let active_supply = self
            .active_supply
            .height
            .insert(height, liveliness * circulating_supply);

        self.active_supply_net_change.height.insert_net_change(
            height,
            &self.active_supply.height,
            ONE_DAY_IN_BLOCK_TIME,
        );

        self.active_supply_3m_net_change.height.insert_net_change(
            height,
            &self.active_supply.height,
            THREE_MONTHS_IN_BLOCK_TIME,
        );

        // TODO: Do these
        // let min_vaulted_supply = ;
        // let max_active_supply = ;

        self.cointime_adjusted_yearly_inflation_rate.height.insert(
            height,
            yearly_inflation_rate * activity_to_vaultedness_ratio,
        );

        self.cointime_adjusted_velocity
            .height
            .insert(height, annualized_transaction_volume / active_supply);

        // TODO:
        // const activeSupplyChangeFromTransactions90dChange =
        //     createNetChangeLazyDataset(activeSupplyChangeFromTransactions, 90);
        //   const activeSupplyChangeFromIssuance = createMultipliedLazyDataset(
        //     lastSubsidy,
        //     liveliness,
        //   );

        let thermo_cap = self
            .thermo_cap
            .height
            .insert(height, cumulative_subsidy_in_dollars);

        let investor_cap = self
            .investor_cap
            .height
            .insert(height, realized_cap - thermo_cap);

        self.thermo_cap_to_investor_cap_ratio
            .height
            .insert(height, thermo_cap / investor_cap);

        // TODO:
        // const activeSupplyChangeFromIssuance90dChange = createNetChangeLazyDataset(
        //   activeSupplyChangeFromIssuance,
        //   90,
        // );

        self.active_price
            .height
            .insert(height, realized_price / liveliness);

        let active_cap = self
            .active_cap
            .height
            .insert(height, active_supply * block_price);

        self.vaulted_price
            .height
            .insert(height, realized_price / vaultedness);

        self.vaulted_cap
            .height
            .insert(height, vaulted_supply * block_price);

        let true_market_mean = self
            .true_market_mean
            .height
            .insert(height, investor_cap / active_supply);

        let true_market_deviation = self
            .true_market_deviation
            .height
            .insert(height, active_cap / investor_cap);

        let true_market_net_unrealized_profit_and_loss = self
            .true_market_net_unrealized_profit_and_loss
            .height
            .insert(height, (active_cap - investor_cap) / active_cap);

        self.investorness
            .height
            .insert(height, investor_cap / realized_cap);

        self.producerness
            .height
            .insert(height, thermo_cap / realized_cap);

        let cointime_value_destroyed = self
            .cointime_value_destroyed
            .height
            .insert(height, block_price * coinblocks_destroyed);

        let cointime_value_created = self
            .cointime_value_created
            .height
            .insert(height, block_price * coinblocks_created);

        let cointime_value_stored = self
            .cointime_value_stored
            .height
            .insert(height, block_price * coinblocks_stored);

        let total_cointime_value_created = self
            .total_cointime_value_created
            .height
            .insert_cumulative(height, &self.cointime_value_created.height);

        let total_cointime_value_destroyed = self
            .total_cointime_value_destroyed
            .height
            .insert_cumulative(height, &self.cointime_value_destroyed.height);

        let total_cointime_value_stored = self
            .total_cointime_value_stored
            .height
            .insert_cumulative(height, &self.cointime_value_stored.height);

        let cointime_price = self.cointime_price.height.insert(
            height,
            total_cointime_value_destroyed / cumulative_coinblocks_stored,
        );

        let cointime_cap = self
            .cointime_cap
            .height
            .insert(height, cointime_price * circulating_supply);

        if is_date_last_block {
            let realized_cap = realized_cap_map.date.get(date).unwrap();
            let realized_price = realized_price_map.date.get(date).unwrap();
            let yearly_inflation_rate = yearly_inflation_rate_map.date.get(date).unwrap();
            let annualized_transaction_volume =
                annualized_transaction_volume_map.date.get(date).unwrap();
            let cumulative_subsidy_in_dollars =
                cumulative_subsidy_in_dollars_map.date.get(date).unwrap();

            self.coinblocks_destroyed
                .date_insert_sum_range(date, date_blocks_range);

            self.cumulative_coinblocks_destroyed
                .date
                .insert(date, cumulative_coinblocks_destroyed);

            self.coinblocks_created
                .date_insert_sum_range(date, date_blocks_range);

            self.cumulative_coinblocks_created
                .date
                .insert(date, cumulative_coinblocks_created);

            self.coinblocks_stored
                .date_insert_sum_range(date, date_blocks_range);

            self.cumulative_coinblocks_stored
                .date
                .insert(date, cumulative_coinblocks_stored);

            self.liveliness.date.insert(date, liveliness);

            self.vaultedness.date.insert(date, vaultedness);

            self.activity_to_vaultedness_ratio
                .date
                .insert(date, activity_to_vaultedness_ratio);

            self.concurrent_liveliness
                .date
                .insert(date, concurrent_liveliness);

            self.concurrent_liveliness_2w_median.date.insert_median(
                date,
                &self.concurrent_liveliness.date,
                TWO_WEEK_IN_DAYS,
            );

            self.liveliness_net_change.date.insert_net_change(
                date,
                &self.liveliness.date,
                ONE_DAY_IN_DAYS,
            );

            self.liveliness_net_change_2w_median.date.insert_net_change(
                date,
                &self.liveliness.date,
                TWO_WEEK_IN_DAYS,
            );

            self.vaulted_supply.date.insert(date, vaulted_supply);

            self.vaulting_rate.date.insert(date, vaulting_rate);

            self.active_supply.date.insert(date, active_supply);

            self.active_supply_net_change.date.insert_net_change(
                date,
                &self.active_supply.date,
                ONE_DAY_IN_DAYS,
            );

            self.active_supply_3m_net_change.date.insert_net_change(
                date,
                &self.active_supply.date,
                THREE_MONTHS_IN_DAYS,
            );

            self.cointime_adjusted_yearly_inflation_rate
                .date
                .insert(date, yearly_inflation_rate * activity_to_vaultedness_ratio);

            self.cointime_adjusted_velocity
                .date
                .insert(date, annualized_transaction_volume / active_supply);

            let thermo_cap = self
                .thermo_cap
                .date
                .insert(date, cumulative_subsidy_in_dollars);

            let investor_cap = self
                .investor_cap
                .date
                .insert(date, realized_cap - thermo_cap);

            self.thermo_cap_to_investor_cap_ratio
                .date
                .insert(date, thermo_cap / investor_cap);

            self.active_price
                .date
                .insert(date, realized_price / liveliness);

            self.active_cap
                .date
                .insert(date, active_supply * date_price);

            self.vaulted_price
                .date
                .insert(date, realized_price / vaultedness);

            self.vaulted_cap
                .date
                .insert(date, vaulted_supply * date_price);

            self.true_market_mean.date.insert(date, true_market_mean);

            self.true_market_deviation
                .date
                .insert(date, true_market_deviation);

            self.true_market_net_unrealized_profit_and_loss
                .date
                .insert(date, true_market_net_unrealized_profit_and_loss);

            self.investorness
                .date
                .insert(date, investor_cap / realized_cap);

            self.producerness
                .date
                .insert(date, thermo_cap / realized_cap);

            self.cointime_value_destroyed
                .date
                .insert(date, cointime_value_destroyed);

            self.cointime_value_created
                .date
                .insert(date, cointime_value_created);

            self.cointime_value_stored
                .date
                .insert(date, cointime_value_stored);

            self.total_cointime_value_created
                .date
                .insert(date, total_cointime_value_created);

            self.total_cointime_value_destroyed
                .date
                .insert(date, total_cointime_value_destroyed);

            self.total_cointime_value_stored
                .date
                .insert(date, total_cointime_value_stored);

            self.cointime_price.date.insert(date, cointime_price);

            self.cointime_cap.date.insert(date, cointime_cap);
        }
    }
}

impl AnyDataset for CointimeDataset {
    fn to_any_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![
            &self.active_cap,
            &self.active_price,
            &self.active_supply,
            &self.active_supply_3m_net_change,
            &self.active_supply_net_change,
            &self.activity_to_vaultedness_ratio,
            &self.coinblocks_created,
            &self.coinblocks_destroyed,
            &self.coinblocks_stored,
            &self.cointime_adjusted_velocity,
            &self.cointime_adjusted_yearly_inflation_rate,
            &self.cointime_cap,
            &self.cointime_price,
            &self.cointime_value_created,
            &self.cointime_value_destroyed,
            &self.cointime_value_stored,
            &self.concurrent_liveliness,
            &self.concurrent_liveliness_2w_median,
            &self.cumulative_coinblocks_created,
            &self.cumulative_coinblocks_destroyed,
            &self.cumulative_coinblocks_stored,
            &self.investor_cap,
            &self.investorness,
            &self.liveliness,
            &self.liveliness_net_change,
            &self.liveliness_net_change_2w_median,
            &self.producerness,
            &self.thermo_cap,
            &self.thermo_cap_to_investor_cap_ratio,
            &self.total_cointime_value_created,
            &self.total_cointime_value_destroyed,
            &self.total_cointime_value_stored,
            &self.true_market_deviation,
            &self.true_market_mean,
            &self.true_market_net_unrealized_profit_and_loss,
            &self.vaulted_cap,
            &self.vaulted_price,
            &self.vaulted_supply,
            &self.vaultedness,
            &self.vaulting_rate,
        ]
    }

    fn to_any_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![
            &mut self.active_cap,
            &mut self.active_price,
            &mut self.active_supply,
            &mut self.active_supply_3m_net_change,
            &mut self.active_supply_net_change,
            &mut self.activity_to_vaultedness_ratio,
            &mut self.coinblocks_created,
            &mut self.coinblocks_destroyed,
            &mut self.coinblocks_stored,
            &mut self.cointime_adjusted_velocity,
            &mut self.cointime_adjusted_yearly_inflation_rate,
            &mut self.cointime_cap,
            &mut self.cointime_price,
            &mut self.cointime_value_created,
            &mut self.cointime_value_destroyed,
            &mut self.cointime_value_stored,
            &mut self.concurrent_liveliness,
            &mut self.concurrent_liveliness_2w_median,
            &mut self.cumulative_coinblocks_created,
            &mut self.cumulative_coinblocks_destroyed,
            &mut self.cumulative_coinblocks_stored,
            &mut self.investor_cap,
            &mut self.investorness,
            &mut self.liveliness,
            &mut self.liveliness_net_change,
            &mut self.liveliness_net_change_2w_median,
            &mut self.producerness,
            &mut self.thermo_cap,
            &mut self.thermo_cap_to_investor_cap_ratio,
            &mut self.total_cointime_value_created,
            &mut self.total_cointime_value_destroyed,
            &mut self.total_cointime_value_stored,
            &mut self.true_market_deviation,
            &mut self.true_market_mean,
            &mut self.true_market_net_unrealized_profit_and_loss,
            &mut self.vaulted_cap,
            &mut self.vaulted_price,
            &mut self.vaulted_supply,
            &mut self.vaultedness,
            &mut self.vaulting_rate,
        ]
    }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }
}
