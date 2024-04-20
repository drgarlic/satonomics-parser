use chrono::NaiveDate;
use itertools::Itertools;

use crate::{
    datasets::{AnyDataset, MinInitialState, ProcessedBlockData},
    parse::{AnyBiMap, BiMap},
    states::PricePaidState,
};

pub struct PricePaidSubDataset {
    min_initial_state: MinInitialState,

    pub realized_cap: BiMap<f32>,
    pub realized_price: BiMap<f32>,

    pp_median: BiMap<f32>,
    pp_95p: BiMap<f32>,
    pp_90p: BiMap<f32>,
    pp_85p: BiMap<f32>,
    pp_80p: BiMap<f32>,
    pp_75p: BiMap<f32>,
    pp_70p: BiMap<f32>,
    pp_65p: BiMap<f32>,
    pp_60p: BiMap<f32>,
    pp_55p: BiMap<f32>,
    pp_45p: BiMap<f32>,
    pp_40p: BiMap<f32>,
    pp_35p: BiMap<f32>,
    pp_30p: BiMap<f32>,
    pp_25p: BiMap<f32>,
    pp_20p: BiMap<f32>,
    pp_15p: BiMap<f32>,
    pp_10p: BiMap<f32>,
    pp_05p: BiMap<f32>,
}

impl PricePaidSubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_state: MinInitialState::default(),

            realized_cap: BiMap::new_bin(1, &f("realized_cap")),
            realized_price: BiMap::new_bin(1, &f("realized_price")),

            pp_median: BiMap::new_bin(1, &f("median_price_paid")),
            pp_95p: BiMap::new_bin(1, &f("95p_price_paid")),
            pp_90p: BiMap::new_bin(1, &f("90p_price_paid")),
            pp_85p: BiMap::new_bin(1, &f("85p_price_paid")),
            pp_80p: BiMap::new_bin(1, &f("80p_price_paid")),
            pp_75p: BiMap::new_bin(1, &f("75p_price_paid")),
            pp_70p: BiMap::new_bin(1, &f("70p_price_paid")),
            pp_65p: BiMap::new_bin(1, &f("65p_price_paid")),
            pp_60p: BiMap::new_bin(1, &f("60p_price_paid")),
            pp_55p: BiMap::new_bin(1, &f("55p_price_paid")),
            pp_45p: BiMap::new_bin(1, &f("45p_price_paid")),
            pp_40p: BiMap::new_bin(1, &f("40p_price_paid")),
            pp_35p: BiMap::new_bin(1, &f("35p_price_paid")),
            pp_30p: BiMap::new_bin(1, &f("30p_price_paid")),
            pp_25p: BiMap::new_bin(1, &f("25p_price_paid")),
            pp_20p: BiMap::new_bin(1, &f("20p_price_paid")),
            pp_15p: BiMap::new_bin(1, &f("15p_price_paid")),
            pp_10p: BiMap::new_bin(1, &f("10p_price_paid")),
            pp_05p: BiMap::new_bin(1, &f("05p_price_paid")),
        };

        s.min_initial_state
            .consume(MinInitialState::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn insert(
        &mut self,
        &ProcessedBlockData {
            height,
            is_date_last_block,
            date,
            ..
        }: &ProcessedBlockData,
        state: &PricePaidState,
        cohort_supply: f32,
    ) {
        let PricePaidState {
            realized_cap,
            pp_05p,
            pp_10p,
            pp_15p,
            pp_20p,
            pp_25p,
            pp_30p,
            pp_35p,
            pp_40p,
            pp_45p,
            pp_median,
            pp_55p,
            pp_60p,
            pp_65p,
            pp_70p,
            pp_75p,
            pp_80p,
            pp_85p,
            pp_90p,
            pp_95p,
            ..
        } = state;

        let realized_cap = self.realized_cap.height.insert(height, *realized_cap);

        if is_date_last_block {
            self.realized_cap.date.insert(date, realized_cap);
        }

        let realized_price = self
            .realized_price
            .height
            .insert(height, cohort_supply / realized_cap);

        if is_date_last_block {
            self.realized_price.date.insert(date, realized_price);
        }

        // Check if iter was empty
        if pp_05p.is_none() {
            self.insert_height_default(height);

            if is_date_last_block {
                self.insert_date_default(date);
            }

            return;
        }

        let pp_05p = self.pp_05p.height.insert(height, pp_05p.unwrap());
        let pp_10p = self.pp_10p.height.insert(height, pp_10p.unwrap());
        let pp_15p = self.pp_15p.height.insert(height, pp_15p.unwrap());
        let pp_20p = self.pp_20p.height.insert(height, pp_20p.unwrap());
        let pp_25p = self.pp_25p.height.insert(height, pp_25p.unwrap());
        let pp_30p = self.pp_30p.height.insert(height, pp_30p.unwrap());
        let pp_35p = self.pp_35p.height.insert(height, pp_35p.unwrap());
        let pp_40p = self.pp_40p.height.insert(height, pp_40p.unwrap());
        let pp_45p = self.pp_45p.height.insert(height, pp_45p.unwrap());
        let pp_median = self.pp_median.height.insert(height, pp_median.unwrap());
        let pp_55p = self.pp_55p.height.insert(height, pp_55p.unwrap());
        let pp_60p = self.pp_60p.height.insert(height, pp_60p.unwrap());
        let pp_65p = self.pp_65p.height.insert(height, pp_65p.unwrap());
        let pp_70p = self.pp_70p.height.insert(height, pp_70p.unwrap());
        let pp_75p = self.pp_75p.height.insert(height, pp_75p.unwrap());
        let pp_80p = self.pp_80p.height.insert(height, pp_80p.unwrap());
        let pp_85p = self.pp_85p.height.insert(height, pp_85p.unwrap());
        let pp_90p = self.pp_90p.height.insert(height, pp_90p.unwrap());
        let pp_95p = self.pp_95p.height.insert(height, pp_95p.unwrap());

        if is_date_last_block {
            self.pp_05p.date.insert(date, pp_05p);
            self.pp_10p.date.insert(date, pp_10p);
            self.pp_15p.date.insert(date, pp_15p);
            self.pp_20p.date.insert(date, pp_20p);
            self.pp_25p.date.insert(date, pp_25p);
            self.pp_30p.date.insert(date, pp_30p);
            self.pp_35p.date.insert(date, pp_35p);
            self.pp_40p.date.insert(date, pp_40p);
            self.pp_45p.date.insert(date, pp_45p);
            self.pp_median.date.insert(date, pp_median);
            self.pp_55p.date.insert(date, pp_55p);
            self.pp_60p.date.insert(date, pp_60p);
            self.pp_65p.date.insert(date, pp_65p);
            self.pp_70p.date.insert(date, pp_70p);
            self.pp_75p.date.insert(date, pp_75p);
            self.pp_80p.date.insert(date, pp_80p);
            self.pp_85p.date.insert(date, pp_85p);
            self.pp_90p.date.insert(date, pp_90p);
            self.pp_95p.date.insert(date, pp_95p);
        }
    }

    fn insert_height_default(&mut self, height: usize) {
        self.as_mut_vec().into_iter().for_each(|bi| {
            bi.height.insert_default(height);
        })
    }

    fn insert_date_default(&mut self, date: NaiveDate) {
        self.as_mut_vec().into_iter().for_each(|bi| {
            bi.date.insert_default(date);
        })
    }

    pub fn as_vec(&self) -> Vec<&BiMap<f32>> {
        vec![
            &self.realized_cap,
            &self.realized_price,
            &self.pp_95p,
            &self.pp_90p,
            &self.pp_85p,
            &self.pp_80p,
            &self.pp_75p,
            &self.pp_70p,
            &self.pp_65p,
            &self.pp_60p,
            &self.pp_55p,
            &self.pp_median,
            &self.pp_45p,
            &self.pp_40p,
            &self.pp_35p,
            &self.pp_30p,
            &self.pp_25p,
            &self.pp_20p,
            &self.pp_15p,
            &self.pp_10p,
            &self.pp_05p,
        ]
    }

    pub fn as_mut_vec(&mut self) -> Vec<&mut BiMap<f32>> {
        vec![
            &mut self.realized_cap,
            &mut self.realized_price,
            &mut self.pp_95p,
            &mut self.pp_90p,
            &mut self.pp_85p,
            &mut self.pp_80p,
            &mut self.pp_75p,
            &mut self.pp_70p,
            &mut self.pp_65p,
            &mut self.pp_60p,
            &mut self.pp_55p,
            &mut self.pp_median,
            &mut self.pp_45p,
            &mut self.pp_40p,
            &mut self.pp_35p,
            &mut self.pp_30p,
            &mut self.pp_25p,
            &mut self.pp_20p,
            &mut self.pp_15p,
            &mut self.pp_10p,
            &mut self.pp_05p,
        ]
    }
}

impl AnyDataset for PricePaidSubDataset {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        self.as_vec()
            .into_iter()
            .map(|dataset| dataset as &(dyn AnyBiMap + Send + Sync))
            .collect_vec()
    }

    fn to_any_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        self.as_mut_vec()
            .into_iter()
            .map(|dataset| dataset as &mut dyn AnyBiMap)
            .collect_vec()
    }
}
