use crate::{
    bitcoin::{
        sats_to_btc, ONE_DAY_IN_BLOCK_TIME, THREE_MONTHS_IN_BLOCK_TIME, TWO_WEEKS_IN_BLOCK_TIME,
    },
    parse::{AnyBiMap, AnyHeightMap, BiMap, HeightMap},
    utils::{ONE_DAY_IN_DAYS, THREE_MONTHS_IN_DAYS, TWO_WEEK_IN_DAYS},
};

use super::{AnyDataset, ExportData, GenericDataset, MinInitialState, ProcessedBlockData};

pub struct CointimeDataset {
    min_initial_state: MinInitialState,

    pub coinblocks_destroyed: BiMap<f32>,

    pub cumulative_coinblocks_destroyed: BiMap<f32>,
    pub coinblocks_created: BiMap<f32>,
    pub cumulative_coinblocks_created: BiMap<f32>,
    pub coinblocks_stored: BiMap<f32>,
    pub cumulative_coinblocks_stored: BiMap<f32>,
    pub liveliness: BiMap<f32>,
    pub vaultedness: BiMap<f32>,
    pub activity_to_vaultedness_ratio: BiMap<f32>,
    pub concurrent_liveliness: BiMap<f32>,
    pub concurrent_liveliness_2w_median: BiMap<Option<f32>>,
    pub liveliness_net_change: BiMap<f32>,
    pub liveliness_net_change_2w_median: BiMap<Option<f32>>,
    pub vaulted_supply: BiMap<f32>,
    pub vaulting_rate: BiMap<f32>,
    pub active_supply: BiMap<f32>,
    pub active_supply_net_change: BiMap<f32>,
    pub active_supply_3m_net_change: BiMap<f32>,
    pub cointime_adjusted_yearly_inflation_rate: BiMap<f32>,
    pub cointime_adjusted_velocity: BiMap<f32>,
    pub thermo_cap: BiMap<f32>,
    pub investor_cap: BiMap<f32>,
    pub thermo_cap_to_investor_cap_ratio: BiMap<f32>,
    pub active_price: BiMap<f32>,
    pub active_cap: BiMap<f32>,
    pub vaulted_price: BiMap<f32>,
    pub vaulted_cap: BiMap<f32>,
    pub true_market_mean: BiMap<f32>,
    pub true_market_deviation: BiMap<f32>,
    pub true_market_net_unrealized_profit_and_loss: BiMap<f32>,
    pub investorness: BiMap<f32>,
    pub producerness: BiMap<f32>,
    pub cointime_value_created: BiMap<f32>,
    pub cointime_value_destroyed: BiMap<f32>,
    pub cointime_value_stored: BiMap<f32>,
    pub total_cointime_value_created: BiMap<f32>,
    pub total_cointime_value_destroyed: BiMap<f32>,
    pub total_cointime_value_stored: BiMap<f32>,
    pub cointime_price: BiMap<f32>,
    pub cointime_cap: BiMap<f32>,
}

impl CointimeDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let s = Self {
            min_initial_state: MinInitialState::default(),

            coinblocks_destroyed: BiMap::new_on_disk_bin(&f("coinblocks_destroyed")),
            cumulative_coinblocks_destroyed: BiMap::new_on_disk_bin(&f(
                "cumulative_coinblocks_destroyed",
            )),
            coinblocks_created: BiMap::new_on_disk_bin(&f("coinblocks_created")),
            cumulative_coinblocks_created: BiMap::new_on_disk_bin(&f(
                "cumulative_coinblocks_created",
            )),
            coinblocks_stored: BiMap::new_on_disk_bin(&f("coinblocks_stored")),
            cumulative_coinblocks_stored: BiMap::new_on_disk_bin(&f(
                "cumulative_coinblocks_stored",
            )),
            liveliness: BiMap::new_on_disk_bin(&f("liveliness")),
            vaultedness: BiMap::new_on_disk_bin(&f("vaultedness")),
            activity_to_vaultedness_ratio: BiMap::new_on_disk_bin(&f(
                "activity_to_vaultedness_ratio",
            )),
            concurrent_liveliness: BiMap::new_on_disk_bin(&f("concurrent_liveliness")),
            concurrent_liveliness_2w_median: BiMap::new_on_disk_bin(&f(
                "concurrent_liveliness_2w_median",
            )),
            liveliness_net_change: BiMap::new_on_disk_bin(&f("liveliness_net_change")),
            liveliness_net_change_2w_median: BiMap::new_on_disk_bin(&f(
                "liveliness_net_change_2w_median",
            )),
            vaulted_supply: BiMap::new_on_disk_bin(&f("vaulted_supply")),
            vaulting_rate: BiMap::new_on_disk_bin(&f("vaulting_rate")),
            active_supply: BiMap::new_on_disk_bin(&f("active_supply")),
            active_supply_net_change: BiMap::new_on_disk_bin(&f("active_supply_net_change")),
            active_supply_3m_net_change: BiMap::new_on_disk_bin(&f("active_supply_3m_net_change")),
            cointime_adjusted_yearly_inflation_rate: BiMap::new_on_disk_bin(&f(
                "cointime_adjusted_yearly_inflation_rate",
            )),
            cointime_adjusted_velocity: BiMap::new_on_disk_bin(&f("cointime_adjusted_velocity")),
            thermo_cap: BiMap::new_on_disk_bin(&f("thermo_cap")),
            investor_cap: BiMap::new_on_disk_bin(&f("investor_cap")),
            thermo_cap_to_investor_cap_ratio: BiMap::new_on_disk_bin(&f(
                "thermo_cap_to_investor_cap_ratio",
            )),
            active_price: BiMap::new_on_disk_bin(&f("active_price")),
            active_cap: BiMap::new_on_disk_bin(&f("active_cap")),
            vaulted_price: BiMap::new_on_disk_bin(&f("vaulted_price")),
            vaulted_cap: BiMap::new_on_disk_bin(&f("vaulted_cap")),
            true_market_mean: BiMap::new_on_disk_bin(&f("true_market_mean")),
            true_market_deviation: BiMap::new_on_disk_bin(&f("true_market_deviation")),
            true_market_net_unrealized_profit_and_loss: BiMap::new_on_disk_bin(&f(
                "true_market_net_unrealized_profit_and_loss",
            )),
            investorness: BiMap::new_on_disk_bin(&f("investorness")),
            producerness: BiMap::new_on_disk_bin(&f("producerness")),
            cointime_value_created: BiMap::new_on_disk_bin(&f("cointime_value_created")),
            cointime_value_destroyed: BiMap::new_on_disk_bin(&f("cointime_value_destroyed")),
            cointime_value_stored: BiMap::new_on_disk_bin(&f("cointime_value_stored")),
            total_cointime_value_created: BiMap::new_on_disk_bin(&f(
                "total_cointime_value_created",
            )),
            total_cointime_value_destroyed: BiMap::new_on_disk_bin(&f(
                "total_cointime_value_destroyed",
            )),
            total_cointime_value_stored: BiMap::new_on_disk_bin(&f("total_cointime_value_stored")),
            cointime_price: BiMap::new_on_disk_bin(&f("cointime_price")),
            cointime_cap: BiMap::new_on_disk_bin(&f("cointime_cap")),
        };

        s.min_initial_state.compute_from_dataset(&s);

        Ok(s)
    }
}

impl GenericDataset for CointimeDataset {
    fn insert_block_data(
        &self,
        &ProcessedBlockData {
            height,
            satblocks_destroyed,
            ..
        }: &ProcessedBlockData,
    ) {
        self.coinblocks_destroyed
            .height
            .insert(height, sats_to_btc(satblocks_destroyed));
    }
}

impl AnyDataset for CointimeDataset {
    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.coinblocks_destroyed.height]
    }

    fn compute(
        &self,
        &ExportData {
            // height,
            annualized_transaction_volume,
            circulating_supply,
            yearly_inflation_rate: inflation_rate,
            height_price,
            realized_cap,
            realized_price,
            subsidy_in_dollars,
            convert_sum_heights_to_date,
            convert_last_height_to_date,
            ..
        }: &ExportData,
    ) {
        self.coinblocks_destroyed
            .compute_date(convert_sum_heights_to_date);

        self.cumulative_coinblocks_destroyed
            .set_height_then_compute_date(
                self.coinblocks_destroyed.height.cumulate(),
                convert_last_height_to_date,
            );

        self.coinblocks_created.set_height_then_compute_date(
            circulating_supply
                .height
                .inner
                .lock()
                .as_ref()
                .unwrap()
                .clone(),
            convert_sum_heights_to_date,
        );

        self.cumulative_coinblocks_created
            .set_height_then_compute_date(
                self.coinblocks_created.height.cumulate(),
                convert_last_height_to_date,
            );

        self.coinblocks_stored.set_height_then_compute_date(
            self.coinblocks_created
                .height
                .subtract(&self.coinblocks_destroyed.height),
            convert_sum_heights_to_date,
        );

        self.cumulative_coinblocks_stored
            .set_height_then_compute_date(
                self.coinblocks_created.height.cumulate(),
                convert_last_height_to_date,
            );

        self.liveliness.set_height_then_compute_date(
            self.cumulative_coinblocks_destroyed
                .height
                .divide(&self.cumulative_coinblocks_created.height),
            convert_last_height_to_date,
        );

        self.vaultedness.set_height_then_compute_date(
            self.liveliness.height.transform(|(_, v, ..)| 1.0 - *v),
            convert_last_height_to_date,
        );

        self.activity_to_vaultedness_ratio
            .set_height_then_compute_date(
                self.liveliness.height.divide(&self.vaultedness.height),
                convert_last_height_to_date,
            );

        self.concurrent_liveliness.set_height_then_compute_date(
            self.coinblocks_destroyed
                .height
                .divide(&self.coinblocks_created.height),
            convert_last_height_to_date,
        );

        self.concurrent_liveliness_2w_median.set_height(
            self.concurrent_liveliness
                .height
                .median(TWO_WEEKS_IN_BLOCK_TIME),
        );
        self.concurrent_liveliness_2w_median
            .set_date(self.concurrent_liveliness.date.median(TWO_WEEK_IN_DAYS));

        self.liveliness_net_change
            .set_height(self.liveliness.height.net_change(ONE_DAY_IN_BLOCK_TIME));
        self.liveliness_net_change
            .set_date(self.liveliness.date.net_change(ONE_DAY_IN_DAYS));

        self.liveliness_net_change_2w_median.set_height(
            self.liveliness_net_change
                .height
                .median(TWO_WEEKS_IN_BLOCK_TIME),
        );
        self.liveliness_net_change_2w_median
            .set_date(self.liveliness_net_change.date.median(TWO_WEEK_IN_DAYS));

        self.vaulted_supply.set_height_then_compute_date(
            self.vaultedness.height.multiply(&circulating_supply.height),
            convert_last_height_to_date,
        );

        self.vaulting_rate.set_height_then_compute_date(
            self.vaulted_supply
                .height
                .transform(|(_, v, ..)| *v * 365.0),
            convert_last_height_to_date,
        );

        self.active_supply.set_height_then_compute_date(
            self.liveliness.height.multiply(&circulating_supply.height),
            convert_last_height_to_date,
        );

        self.active_supply_net_change
            .set_height(self.active_supply.height.net_change(ONE_DAY_IN_BLOCK_TIME));
        self.active_supply_net_change
            .set_date(self.active_supply.date.net_change(ONE_DAY_IN_DAYS));

        self.active_supply_3m_net_change.set_height(
            self.active_supply
                .height
                .net_change(THREE_MONTHS_IN_BLOCK_TIME),
        );
        self.active_supply_3m_net_change
            .set_date(self.active_supply.date.net_change(THREE_MONTHS_IN_DAYS));

        // TODO: Do these
        // let min_vaulted_supply = ;
        // let max_active_supply = ;

        self.cointime_adjusted_yearly_inflation_rate
            .set_height_then_compute_date(
                inflation_rate
                    .height
                    .multiply(&self.activity_to_vaultedness_ratio.height),
                convert_last_height_to_date,
            );

        self.cointime_adjusted_velocity
            .set_height_then_compute_date(
                annualized_transaction_volume
                    .height
                    .divide(&self.active_supply.height),
                convert_last_height_to_date,
            );

        // TODO:
        // const activeSupplyChangeFromTransactions90dChange =
        //     createNetChangeLazyDataset(activeSupplyChangeFromTransactions, 90);
        //   const activeSupplyChangeFromIssuance = createMultipliedLazyDataset(
        //     lastSubsidy,
        //     liveliness,
        //   );

        self.thermo_cap.set_height_then_compute_date(
            subsidy_in_dollars.height.cumulate(),
            convert_last_height_to_date,
        );

        self.investor_cap.set_height_then_compute_date(
            realized_cap.height.subtract(&self.thermo_cap.height),
            convert_last_height_to_date,
        );

        self.thermo_cap_to_investor_cap_ratio
            .set_height_then_compute_date(
                self.thermo_cap.height.divide(&self.investor_cap.height),
                convert_last_height_to_date,
            );

        // TODO:
        // const activeSupplyChangeFromIssuance90dChange = createNetChangeLazyDataset(
        //   activeSupplyChangeFromIssuance,
        //   90,
        // );

        self.active_price.set_height_then_compute_date(
            realized_price.height.divide(&self.liveliness.height),
            convert_last_height_to_date,
        );

        self.active_cap.set_height_then_compute_date(
            self.active_supply.height.multiply(height_price),
            convert_last_height_to_date,
        );

        self.vaulted_price.set_height_then_compute_date(
            realized_price.height.divide(&self.vaultedness.height),
            convert_last_height_to_date,
        );

        self.vaulted_cap.set_height_then_compute_date(
            self.vaulted_supply.height.multiply(height_price),
            convert_last_height_to_date,
        );

        self.true_market_mean.set_height_then_compute_date(
            self.investor_cap.height.divide(&self.active_supply.height),
            convert_last_height_to_date,
        );

        self.true_market_deviation.set_height_then_compute_date(
            self.active_cap.height.divide(&self.investor_cap.height),
            convert_last_height_to_date,
        );

        self.true_market_net_unrealized_profit_and_loss
            .set_height_then_compute_date(
                HeightMap::_divide(
                    &self.active_cap.height.subtract(&self.investor_cap.height),
                    self.active_cap.height.inner.lock().as_ref().unwrap(),
                ),
                convert_last_height_to_date,
            );

        self.investorness.set_height_then_compute_date(
            self.investor_cap.height.divide(&realized_cap.height),
            convert_last_height_to_date,
        );

        self.producerness.set_height_then_compute_date(
            self.thermo_cap.height.divide(&realized_cap.height),
            convert_last_height_to_date,
        );

        self.cointime_value_created.set_height_then_compute_date(
            height_price.multiply(&self.coinblocks_created.height),
            convert_last_height_to_date,
        );

        self.cointime_value_destroyed.set_height_then_compute_date(
            height_price.multiply(&self.coinblocks_destroyed.height),
            convert_last_height_to_date,
        );

        self.cointime_value_stored.set_height_then_compute_date(
            height_price.multiply(&self.coinblocks_stored.height),
            convert_last_height_to_date,
        );

        self.total_cointime_value_created
            .set_height_then_compute_date(
                self.cointime_value_created.height.cumulate(),
                convert_last_height_to_date,
            );

        self.total_cointime_value_destroyed
            .set_height_then_compute_date(
                self.cointime_value_destroyed.height.cumulate(),
                convert_last_height_to_date,
            );

        self.total_cointime_value_stored
            .set_height_then_compute_date(
                self.cointime_value_stored.height.cumulate(),
                convert_last_height_to_date,
            );

        self.cointime_price.set_height_then_compute_date(
            self.total_cointime_value_destroyed
                .height
                .divide(&self.cumulative_coinblocks_stored.height),
            convert_last_height_to_date,
        );

        self.cointime_cap.set_height_then_compute_date(
            self.cointime_price
                .height
                .multiply(&circulating_supply.height),
            convert_last_height_to_date,
        );
    }

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![
            &self.coinblocks_destroyed,
            &self.cumulative_coinblocks_destroyed,
            &self.coinblocks_created,
            &self.cumulative_coinblocks_created,
            &self.coinblocks_stored,
            &self.cumulative_coinblocks_stored,
            &self.liveliness,
            &self.vaultedness,
            &self.activity_to_vaultedness_ratio,
            &self.concurrent_liveliness,
            &self.concurrent_liveliness_2w_median,
            &self.liveliness_net_change,
            &self.liveliness_net_change_2w_median,
            &self.vaulted_supply,
            &self.vaulting_rate,
            &self.active_supply,
            &self.active_supply_net_change,
            &self.active_supply_3m_net_change,
            &self.cointime_adjusted_yearly_inflation_rate,
            &self.cointime_adjusted_velocity,
            &self.thermo_cap,
            &self.investor_cap,
            &self.thermo_cap_to_investor_cap_ratio,
            &self.active_price,
            &self.active_cap,
            &self.vaulted_price,
            &self.vaulted_cap,
            &self.true_market_mean,
            &self.true_market_deviation,
            &self.true_market_net_unrealized_profit_and_loss,
            &self.investorness,
            &self.producerness,
            &self.cointime_value_created,
            &self.cointime_value_destroyed,
            &self.cointime_value_stored,
            &self.total_cointime_value_created,
            &self.total_cointime_value_destroyed,
            &self.total_cointime_value_stored,
            &self.cointime_price,
            &self.cointime_cap,
        ]
    }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }
}
