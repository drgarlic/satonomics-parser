use crate::{
    bitcoin::{sats_to_btc, ONE_YEAR_IN_BLOCK_TIME},
    datasets::AnyDataset,
    parse::{AnyBiMap, AnyDateMap, BiMap, DateMap},
    utils::{ONE_MONTH_IN_DAYS, ONE_WEEK_IN_DAYS, ONE_YEAR_IN_DAYS},
};

use super::{AddressDatasets, MinInitialState, ProcessedBlockData};

pub struct MiningDataset {
    min_initial_state: MinInitialState,

    pub coinbase: BiMap<f32>,
    pub fees: BiMap<f32>,
    pub subsidy: BiMap<f32>,
    pub subsidy_in_dollars: BiMap<f32>,
    pub cumulative_subsidy_in_dollars: BiMap<f32>,
    pub annualized_issuance: BiMap<f32>,
    pub yearly_inflation_rate: BiMap<f32>,

    pub blocks_mined: DateMap<usize>,
    pub blocks_mined_1w_sma: DateMap<f32>,
    pub blocks_mined_1m_sma: DateMap<f32>,
    pub last_subsidy: DateMap<f32>,
    pub last_subsidy_in_dollars: DateMap<f32>,
}

impl MiningDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_state: MinInitialState::default(),

            blocks_mined: DateMap::new_bin(1, &f("blocks_mined")),
            coinbase: BiMap::new_bin(1, &f("coinbase")),
            fees: BiMap::new_bin(1, &f("fees")),

            subsidy: BiMap::_new_bin(1, &f("subsidy"), 5),
            subsidy_in_dollars: BiMap::new_bin(1, &f("subsidy_in_dollars")),
            cumulative_subsidy_in_dollars: BiMap::new_bin(1, &f("cumulative_subsidy_in_dollars")),

            annualized_issuance: BiMap::new_bin(1, &f("annualized_issuance")),
            yearly_inflation_rate: BiMap::new_bin(1, &f("yearly_inflation_rate")),

            last_subsidy: DateMap::new_bin(1, &f("last_subsidy")),
            last_subsidy_in_dollars: DateMap::new_bin(1, &f("last_subsidy_in_dollars")),

            blocks_mined_1w_sma: DateMap::new_bin(1, &f("blocks_mined_7d_sma")),
            blocks_mined_1m_sma: DateMap::new_bin(1, &f("blocks_mined_1m_sma")),
        };

        s.min_initial_state
            .consume(MinInitialState::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn insert_data(
        &mut self,
        &ProcessedBlockData {
            date_first_height,
            height,
            coinbase,
            fees,
            date_blocks_range,
            is_date_last_block,
            block_price,
            date_price,
            date,
            ..
        }: &ProcessedBlockData,
        address_datasets: &AddressDatasets,
    ) {
        let circulating_supply_map = &address_datasets.all.all.supply.total;
        let circulating_supply = circulating_supply_map.height.get(&height).unwrap();

        let coinbase = sats_to_btc(coinbase);

        self.coinbase.height.insert(height, coinbase);

        let sumed_fees = sats_to_btc(fees.iter().sum());

        self.fees.height.insert(height, sumed_fees);

        let subsidy = coinbase - sumed_fees;

        self.subsidy.height.insert(height, subsidy);

        let subsidy_in_dollars = subsidy * block_price;

        self.subsidy_in_dollars
            .height
            .insert(height, subsidy_in_dollars);

        self.cumulative_subsidy_in_dollars
            .height
            .insert_cumulative(height, &self.subsidy_in_dollars.height);

        let annualized_issuance = self.annualized_issuance.height.insert_last_x_sum(
            height,
            &self.subsidy.height,
            ONE_YEAR_IN_BLOCK_TIME,
        );

        self.yearly_inflation_rate
            .height
            .insert(height, annualized_issuance / circulating_supply);

        if is_date_last_block {
            let coinbase = self
                .coinbase
                .date
                .insert(date, self.coinbase.height.sum_range(date_blocks_range));

            let fees = self
                .fees
                .date
                .insert(date, self.fees.height.sum_range(date_blocks_range));

            let subsidy = self.subsidy.date.insert(date, coinbase - fees);

            let subsidy_in_dollars = self
                .subsidy_in_dollars
                .date
                .insert(date, subsidy * date_price);

            self.cumulative_subsidy_in_dollars
                .date
                .insert_cumulative(date, &self.subsidy_in_dollars.date);

            self.last_subsidy.insert(date, subsidy);

            self.last_subsidy_in_dollars
                .insert(date, subsidy_in_dollars);

            let annualized_issuance = self.annualized_issuance.date.insert_last_x_sum(
                date,
                &self.subsidy.date,
                ONE_YEAR_IN_DAYS,
            );

            self.yearly_inflation_rate
                .date
                .insert(date, annualized_issuance / circulating_supply);

            self.blocks_mined
                .insert(date, height + 1 - date_first_height);

            self.blocks_mined_1w_sma.insert_simple_average(
                date,
                &self.blocks_mined,
                ONE_WEEK_IN_DAYS,
            );

            self.blocks_mined_1m_sma.insert_simple_average(
                date,
                &self.blocks_mined,
                ONE_MONTH_IN_DAYS,
            );
        }
    }
}

impl AnyDataset for MiningDataset {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![
            &self.blocks_mined,
            &self.blocks_mined_1w_sma,
            &self.blocks_mined_1m_sma,
            &self.last_subsidy,
            &self.last_subsidy_in_dollars,
        ]
    }

    fn to_any_mut_date_map_vec(&mut self) -> Vec<&mut dyn AnyDateMap> {
        vec![
            &mut self.blocks_mined,
            &mut self.blocks_mined_1w_sma,
            &mut self.blocks_mined_1m_sma,
            &mut self.last_subsidy,
            &mut self.last_subsidy_in_dollars,
        ]
    }

    fn to_any_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![
            &self.coinbase,
            &self.fees,
            &self.subsidy,
            &self.subsidy_in_dollars,
            &self.cumulative_subsidy_in_dollars,
            &self.annualized_issuance,
            &self.yearly_inflation_rate,
        ]
    }

    fn to_any_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![
            &mut self.coinbase,
            &mut self.fees,
            &mut self.subsidy,
            &mut self.subsidy_in_dollars,
            &mut self.cumulative_subsidy_in_dollars,
            &mut self.annualized_issuance,
            &mut self.yearly_inflation_rate,
        ]
    }
}
