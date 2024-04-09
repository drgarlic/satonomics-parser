use crate::{
    bitcoin::{sats_to_btc, ONE_YEAR_IN_BLOCK_TIME},
    datasets::AnyDataset,
    parse::{AnyBiMap, AnyDateMap, AnyHeightMap, BiMap, DateMap},
    utils::{ONE_MONTH_IN_DAYS, ONE_WEEK_IN_DAYS, ONE_YEAR_IN_DAYS},
};

use super::{GenericDataset, MinInitialState, ProcessedBlockData, ProcessedDateData};

pub struct MiningDataset {
    min_initial_state: MinInitialState,

    pub blocks_mined: DateMap<usize>,
    pub coinbase: BiMap<f32>,
    pub fees: BiMap<f32>,

    pub subsidy: BiMap<f32>,
    pub subsidy_in_dollars: BiMap<f32>,
    pub annualized_issuance: BiMap<f32>,
    pub yearly_inflation_rate: BiMap<f32>,
    pub last_subsidy: DateMap<f32>,
    pub last_subsidy_in_dollars: DateMap<f32>,
    pub blocks_mined_1w_sma: DateMap<f32>,
    pub blocks_mined_1m_sma: DateMap<f32>,
}

impl MiningDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let s = Self {
            min_initial_state: MinInitialState::default(),

            blocks_mined: DateMap::new_bin(&f("blocks_mined")),
            coinbase: BiMap::new_bin(&f("coinbase")),
            fees: BiMap::new_bin(&f("fees")),

            subsidy: BiMap::new_bin(&f("subsidy")),
            subsidy_in_dollars: BiMap::new_bin(&f("subsidy_in_dollars")),

            annualized_issuance: BiMap::new_bin(&f("annualized_issuance")),
            yearly_inflation_rate: BiMap::new_bin(&f("yearly_inflation_rate")),

            last_subsidy: DateMap::new_bin(&f("last_subsidy")),
            last_subsidy_in_dollars: DateMap::new_bin(&f("last_subsidy_in_dollars")),

            blocks_mined_1w_sma: DateMap::new_bin(&f("blocks_mined_7d_sma")),
            blocks_mined_1m_sma: DateMap::new_bin(&f("blocks_mined_1m_sma")),
        };

        s.min_initial_state.compute_from_dataset(&s);

        Ok(s)
    }
}

impl GenericDataset for MiningDataset {
    fn insert_date_data(
        &self,
        &ProcessedDateData {
            date,
            first_height,
            height,
            ..
        }: &ProcessedDateData,
    ) {
        self.blocks_mined.insert(date, height + 1 - first_height);
    }

    fn insert_block_data(
        &self,
        &ProcessedBlockData {
            height,
            coinbase,
            fees,
            ..
        }: &ProcessedBlockData,
    ) {
        let sumed_fees = fees.iter().sum();

        self.coinbase.height.insert(height, sats_to_btc(coinbase));

        self.fees.height.insert(height, sats_to_btc(sumed_fees));
    }
}

impl AnyDataset for MiningDataset {
    fn to_any_inserted_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![&self.blocks_mined]
    }

    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![
            &self.coinbase.height,
            &self.fees.height,
            &self.subsidy.height,
            &self.subsidy_in_dollars.height,
        ]
    }

    // fn prepare(
    //     &self,
    //     &ExportData {
    //         circulating_supply,
    //         convert_sum_heights_to_date,
    //         height_price,
    //         ..
    //     }: &ExportData,
    // ) {
    // self.coinbase.compute_date(convert_sum_heights_to_date);

    // self.fees.compute_date(convert_sum_heights_to_date);

    // self.subsidy.set_height_then_compute_date(
    //     self.coinbase.height.subtract(&self.fees.height),
    //     convert_sum_heights_to_date,
    // );

    // self.subsidy_in_dollars.set_height_then_compute_date(
    //     self.subsidy.height.multiply(height_price),
    //     convert_sum_heights_to_date,
    // );

    // self.annualized_issuance
    //     .set_height(self.subsidy.height.last_x_sum(ONE_YEAR_IN_BLOCK_TIME));
    // self.annualized_issuance
    //     .set_date(self.subsidy.date.last_x_sum(ONE_YEAR_IN_DAYS));

    // self.yearly_inflation_rate.set_height(
    //     self.annualized_issuance
    //         .height
    //         .divide(&circulating_supply.height),
    // );
    // self.yearly_inflation_rate.set_date(
    //     self.annualized_issuance
    //         .date
    //         .divide(&circulating_supply.date),
    // );
    // }

    // fn compute(
    //     &self,
    //     &ExportData {
    //         convert_last_height_to_date,
    //         ..
    //     }: &ExportData,
    // ) {
    // self.last_subsidy.compute_from_height_map(
    //     self.subsidy.height.imported.lock().as_ref().unwrap(),
    //     convert_last_height_to_date,
    // );
    // self.last_subsidy_in_dollars.compute_from_height_map(
    //     self.subsidy_in_dollars
    //         .height
    //         .imported
    //         .lock()
    //         .as_ref()
    //         .unwrap(),
    //     convert_last_height_to_date,
    // );

    // self.blocks_mined_1w_sma
    //     .set_inner(self.blocks_mined.simple_moving_average(ONE_WEEK_IN_DAYS));
    // self.blocks_mined_1m_sma
    //     .set_inner(self.blocks_mined.simple_moving_average(ONE_MONTH_IN_DAYS));
    // }

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![
            &self.coinbase,
            &self.fees,
            &self.subsidy,
            &self.subsidy_in_dollars,
            &self.annualized_issuance,
            &self.yearly_inflation_rate,
        ]
    }

    fn to_any_exported_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![
            &self.last_subsidy,
            &self.last_subsidy_in_dollars,
            &self.blocks_mined,
            &self.blocks_mined_1w_sma,
            &self.blocks_mined_1m_sma,
        ]
    }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }
}
