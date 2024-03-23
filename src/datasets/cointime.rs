use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Sub},
};

use chrono::NaiveDate;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use rayon::prelude::*;

use crate::{
    bitcoin::{
        sats_to_btc, ONE_DAY_IN_BLOCK_TIME, THREE_MONTHS_IN_BLOCK_TIME, TWO_WEEKS_IN_BLOCK_TIME,
    },
    parse::{AnyDateMap, AnyHeightMap, BiMap},
};

use super::{AnyDataset2, ProcessedBlockData};

pub struct CointimeDataset {
    min_initial_first_unsafe_date: Option<NaiveDate>,
    min_initial_first_unsafe_height: Option<usize>,

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

pub struct PostParseData<'a> {
    // pub height: usize,
    pub circulating_supply: &'a [f32],
    pub price: &'a [f32],
    // pub date_to_last_height: &'a ,
    pub inflation_rate: &'a [f32],
    pub annualized_transaction_volume: &'a [f32],
    pub subsidy_in_dollars: &'a [f32],
    pub realized_cap: &'a [f32],
    pub realized_price: &'a [f32],
}

impl CointimeDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_first_unsafe_date: None,
            min_initial_first_unsafe_height: None,

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

        s.min_initial_first_unsafe_date = s.compute_min_initial_first_unsafe_date();
        s.min_initial_first_unsafe_height = s.compute_min_initial_first_unsafe_height();

        Ok(s)
    }

    fn post_parse(
        &mut self,
        &PostParseData {
            // height,
            price,
            circulating_supply,
            inflation_rate,
            annualized_transaction_volume,
            // date_to_last_height,
            subsidy_in_dollars,
            realized_cap,
            realized_price,
        }: &PostParseData,
    ) {
        // ---
        // Compute
        // ---

        let coinblocks_destroyed = self.coinblocks_destroyed.height.import();

        let cumulative_coinblocks_destroyed = cumulate(&coinblocks_destroyed);

        let coinblocks_created = circulating_supply.to_vec();

        let cumulative_coinblocks_created = cumulate(&coinblocks_created);

        let coinblocks_stored = subtract(&coinblocks_created, &coinblocks_destroyed);

        let cumulative_coinblocks_stored = cumulate(&coinblocks_stored);

        let liveliness = divide(
            &cumulative_coinblocks_destroyed,
            &cumulative_coinblocks_created,
        );

        let vaultedness = transform(&liveliness, |(_, v)| 1.0 - *v);

        let activity_to_vaultedness_ratio = divide(&liveliness, &vaultedness);

        let concurrent_liveliness = divide(&coinblocks_destroyed, &coinblocks_created);

        let concurrent_liveliness_2w_median =
            median(&concurrent_liveliness, TWO_WEEKS_IN_BLOCK_TIME);
        // TODO: DateMap => median = 14

        let liveliness_net_change = net_change(&liveliness, ONE_DAY_IN_BLOCK_TIME);
        // TODO: DateMap => offset = 1

        let liveliness_net_change_2w_median =
            median(&liveliness_net_change, TWO_WEEKS_IN_BLOCK_TIME);

        let vaulted_supply = multiply(&vaultedness, circulating_supply);

        let vaulting_rate = divide(
            &transform(&vaulted_supply, |(_, v)| *v * 365.0),
            circulating_supply,
        );

        let active_supply = multiply(&liveliness, circulating_supply);

        let active_supply_net_change = net_change(&active_supply, ONE_DAY_IN_BLOCK_TIME);

        let active_supply_3m_net_change = net_change(&active_supply, THREE_MONTHS_IN_BLOCK_TIME);

        // TODO: Do these
        // let min_vaulted_supply = 0;
        // let max_active_supply = 0;

        let cointime_adjusted_yearly_inflation_rate =
            multiply(inflation_rate, &activity_to_vaultedness_ratio);

        let cointime_adjusted_velocity = divide(annualized_transaction_volume, &active_supply);

        // TODO:
        // const activeSupplyChangeFromTransactions90dChange =
        //     createNetChangeLazyDataset(activeSupplyChangeFromTransactions, 90);
        //   const activeSupplyChangeFromIssuance = createMultipliedLazyDataset(
        //     lastSubsidy,
        //     liveliness,
        //   );

        let thermo_cap = cumulate(subsidy_in_dollars);

        let investor_cap = subtract(realized_cap, &thermo_cap);

        let thermo_cap_to_investor_cap_ratio = divide(&thermo_cap, &investor_cap);

        // TODO:
        // const activeSupplyChangeFromIssuance90dChange = createNetChangeLazyDataset(
        //   activeSupplyChangeFromIssuance,
        //   90,
        // );

        let active_price = divide(realized_price, &liveliness);

        let active_cap = multiply(&active_supply, price);

        let vaulted_price = divide(realized_price, &vaultedness);

        let true_market_mean = divide(&investor_cap, &active_supply);

        let true_market_deviation = divide(&active_cap, &investor_cap);

        let true_market_net_unrealized_profit_and_loss =
            divide(&subtract(&active_cap, &investor_cap), &active_cap);

        let investorness = divide(&investor_cap, realized_cap);

        let producerness = divide(&thermo_cap, realized_cap);

        let cointime_value_created = multiply(price, &coinblocks_created);

        let cointime_value_destroyed = multiply(price, &coinblocks_destroyed);

        let cointime_value_stored = multiply(price, &coinblocks_stored);

        let total_cointime_value_created = cumulate(&cointime_value_created);

        let total_cointime_value_destroyed = cumulate(&cointime_value_destroyed);

        let total_cointime_value_stored = cumulate(&cointime_value_stored);

        let cointime_price = divide(
            &total_cointime_value_destroyed,
            &cumulative_coinblocks_stored,
        );

        let cointime_cap = multiply(&cointime_price, circulating_supply);

        // ---
        // Set
        // ---

        self.cumulative_coinblocks_destroyed
            .set(cumulative_coinblocks_destroyed);

        self.coinblocks_created.set(coinblocks_created);

        self.cumulative_coinblocks_created
            .set(cumulative_coinblocks_created);

        self.coinblocks_stored.set(coinblocks_stored);

        self.cumulative_coinblocks_stored
            .set(cumulative_coinblocks_stored);

        self.liveliness.set(liveliness);

        self.vaultedness.set(vaultedness);

        self.activity_to_vaultedness_ratio
            .set(activity_to_vaultedness_ratio);

        self.concurrent_liveliness.set(concurrent_liveliness);

        self.concurrent_liveliness_2w_median
            .set(concurrent_liveliness_2w_median);

        self.liveliness_net_change.set(liveliness_net_change);

        self.liveliness_net_change_2w_median
            .set(liveliness_net_change_2w_median);

        self.vaulted_supply.set(vaulted_supply);

        self.vaulting_rate.set(vaulting_rate);

        self.active_supply.set(active_supply);

        self.active_supply_net_change.set(active_supply_net_change);

        self.active_supply_3m_net_change
            .set(active_supply_3m_net_change);

        self.cointime_adjusted_yearly_inflation_rate
            .set(cointime_adjusted_yearly_inflation_rate);

        self.cointime_adjusted_velocity
            .set(cointime_adjusted_velocity);

        self.thermo_cap.set(thermo_cap);

        self.investor_cap.set(investor_cap);

        self.thermo_cap_to_investor_cap_ratio
            .set(thermo_cap_to_investor_cap_ratio);

        self.active_price.set(active_price);

        self.active_cap.set(active_cap);

        self.vaulted_price.set(vaulted_price);

        self.true_market_mean.set(true_market_mean);

        self.true_market_deviation.set(true_market_deviation);

        self.true_market_net_unrealized_profit_and_loss
            .set(true_market_net_unrealized_profit_and_loss);

        self.investorness.set(investorness);

        self.producerness.set(producerness);

        self.cointime_value_created.set(cointime_value_created);

        self.cointime_value_destroyed.set(cointime_value_destroyed);

        self.cointime_value_stored.set(cointime_value_stored);

        self.total_cointime_value_created
            .set(total_cointime_value_created);

        self.total_cointime_value_destroyed
            .set(total_cointime_value_destroyed);

        self.total_cointime_value_stored
            .set(total_cointime_value_stored);

        self.cointime_price.set(cointime_price);

        self.cointime_cap.set(cointime_cap);
    }
}

fn transform<T, F>(arr: &[T], transform: F) -> Vec<T>
where
    T: Copy + Default,
    F: Fn((usize, &T)) -> T,
{
    arr.iter().enumerate().map(transform).collect_vec()
}

fn add<T>(arr1: &[T], arr2: &[T]) -> Vec<T>
where
    T: Add<Output = T> + Copy + Default,
{
    if arr1.len() != arr2.len() {
        panic!("Can't add two arrays with a different length");
    }

    transform(arr1, |(index, value)| *value + *arr2.get(index).unwrap())
}

fn subtract<T>(arr1: &[T], arr2: &[T]) -> Vec<T>
where
    T: Sub<Output = T> + Copy + Default,
{
    if arr1.len() != arr2.len() {
        panic!("Can't subtract two arrays with a different length");
    }

    transform(arr1, |(index, value)| *value - *arr2.get(index).unwrap())
}

fn multiply<T>(arr1: &[T], arr2: &[T]) -> Vec<T>
where
    T: Mul<Output = T> + Copy + Default,
{
    if arr1.len() != arr2.len() {
        panic!("Can't multiply two arrays with a different length");
    }

    transform(arr1, |(index, value)| *value * *arr2.get(index).unwrap())
}

fn divide<T>(arr1: &[T], arr2: &[T]) -> Vec<T>
where
    T: Div<Output = T> + Copy + Default,
{
    if arr1.len() != arr2.len() {
        panic!("Can't divide two arrays with a different length");
    }

    transform(arr1, |(index, value)| *value / *arr2.get(index).unwrap())
}

fn cumulate<T>(arr: &[T]) -> Vec<T>
where
    T: Sum + Copy + Default + AddAssign,
{
    let mut sum = T::default();

    arr.iter()
        .map(|value| {
            sum += *value;
            sum
        })
        .collect_vec()
}

fn net_change<T>(arr: &[T], offset: usize) -> Vec<T>
where
    T: Copy + Default + Sub<Output = T>,
{
    transform(arr, |(index, value)| {
        let previous = {
            if let Some(previous_index) = index.checked_sub(offset) {
                *arr.get(previous_index).unwrap()
            } else {
                T::default()
            }
        };

        *value - previous
    })
}

fn median(arr: &[f32], size: usize) -> Vec<Option<f32>>
// where
//     T: Copy + Default + Add<Output = T> + Ord,
{
    let even = size % 2 == 0;
    let median_index = size / 2;

    if size < 3 {
        panic!("Computing a median for a size lower than 3 is useless");
    }

    arr.par_iter()
        .enumerate()
        .map(|(index, _)| {
            if index >= size - 1 {
                let mut arr = arr[index - (size - 1)..index + 1]
                    .iter()
                    .map(|value| OrderedFloat(*value))
                    .collect_vec();

                arr.sort_unstable();

                if even {
                    Some(
                        **arr.get(median_index).unwrap()
                            + **arr.get(median_index - 1).unwrap() / 2.0,
                    )
                } else {
                    Some(**arr.get(median_index).unwrap())
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

impl AnyDataset2 for CointimeDataset {
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

        // if is_date_last_block {
        //     self.coinblocks_destroyed
        //         .date
        //         .insert(date, coinblocks_destroyed_vec.iter().sum())
        // }
    }

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.coinblocks_destroyed.height]
    }

    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![&self.coinblocks_destroyed.date]
    }

    fn get_min_initial_first_unsafe_date(&self) -> &Option<NaiveDate> {
        &self.min_initial_first_unsafe_date
    }

    fn get_min_initial_first_unsafe_height(&self) -> &Option<usize> {
        &self.min_initial_first_unsafe_height
    }
}
