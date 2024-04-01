use itertools::Itertools;

use crate::{
    datasets::{AnyDataset, ExportData, MinInitialState, ProcessedBlockData},
    parse::{AnyExportableMap, AnyHeightMap, AnyMap, BiMap},
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

        let s = Self {
            min_initial_state: MinInitialState::default(),

            realized_cap: BiMap::new_in_memory_bin(&f("realized_cap")),
            realized_price: BiMap::new_in_memory_bin(&f("realized_price")),

            pp_median: BiMap::new_on_disk_bin(&f("median_price_paid")),
            pp_95p: BiMap::new_on_disk_bin(&f("95p_price_paid")),
            pp_90p: BiMap::new_on_disk_bin(&f("90p_price_paid")),
            pp_85p: BiMap::new_on_disk_bin(&f("85p_price_paid")),
            pp_80p: BiMap::new_on_disk_bin(&f("80p_price_paid")),
            pp_75p: BiMap::new_on_disk_bin(&f("75p_price_paid")),
            pp_70p: BiMap::new_on_disk_bin(&f("70p_price_paid")),
            pp_65p: BiMap::new_on_disk_bin(&f("65p_price_paid")),
            pp_60p: BiMap::new_on_disk_bin(&f("60p_price_paid")),
            pp_55p: BiMap::new_on_disk_bin(&f("55p_price_paid")),
            pp_45p: BiMap::new_on_disk_bin(&f("45p_price_paid")),
            pp_40p: BiMap::new_on_disk_bin(&f("40p_price_paid")),
            pp_35p: BiMap::new_on_disk_bin(&f("35p_price_paid")),
            pp_30p: BiMap::new_on_disk_bin(&f("30p_price_paid")),
            pp_25p: BiMap::new_on_disk_bin(&f("25p_price_paid")),
            pp_20p: BiMap::new_on_disk_bin(&f("20p_price_paid")),
            pp_15p: BiMap::new_on_disk_bin(&f("15p_price_paid")),
            pp_10p: BiMap::new_on_disk_bin(&f("10p_price_paid")),
            pp_05p: BiMap::new_on_disk_bin(&f("05p_price_paid")),
        };

        s.min_initial_state.compute_from_dataset(&s);

        Ok(s)
    }

    pub fn insert(
        &self,
        &ProcessedBlockData { height, .. }: &ProcessedBlockData,
        state: &PricePaidState,
        cohort_supply: &BiMap<f32>,
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

        self.realized_cap.height.insert(height, *realized_cap);

        // TODO: Move to prepare function instead of inserting
        {
            let supply = cohort_supply.height.inner.lock();

            self.realized_price.height.insert(
                height,
                supply
                    .as_ref()
                    .unwrap_or_else(|| {
                        panic!("Cannot unwrap None: {}", &self.realized_price.height.path())
                    })
                    .get(height)
                    .unwrap_or_else(|| {
                        panic!(
                            "Can't find height {}: {}",
                            height,
                            &self.realized_price.height.path()
                        )
                    })
                    / realized_cap,
            );
        }

        // Check if iter was empty
        if pp_05p.is_none() {
            self.insert_height_default(height);

            return;
        }

        self.pp_05p.height.insert(height, pp_05p.unwrap());
        self.pp_10p.height.insert(height, pp_10p.unwrap());
        self.pp_15p.height.insert(height, pp_15p.unwrap());
        self.pp_20p.height.insert(height, pp_20p.unwrap());
        self.pp_25p.height.insert(height, pp_25p.unwrap());
        self.pp_30p.height.insert(height, pp_30p.unwrap());
        self.pp_35p.height.insert(height, pp_35p.unwrap());
        self.pp_40p.height.insert(height, pp_40p.unwrap());
        self.pp_45p.height.insert(height, pp_45p.unwrap());
        self.pp_median.height.insert(height, pp_median.unwrap());
        self.pp_55p.height.insert(height, pp_55p.unwrap());
        self.pp_60p.height.insert(height, pp_60p.unwrap());
        self.pp_65p.height.insert(height, pp_65p.unwrap());
        self.pp_70p.height.insert(height, pp_70p.unwrap());
        self.pp_75p.height.insert(height, pp_75p.unwrap());
        self.pp_80p.height.insert(height, pp_80p.unwrap());
        self.pp_85p.height.insert(height, pp_85p.unwrap());
        self.pp_90p.height.insert(height, pp_90p.unwrap());
        self.pp_95p.height.insert(height, pp_95p.unwrap());
    }

    fn insert_height_default(&self, height: usize) {
        self.to_vec()
            .iter()
            .for_each(|bi| bi.height.insert_default(height))
    }

    pub fn to_vec(&self) -> Vec<&BiMap<f32>> {
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
}

impl AnyDataset for PricePaidSubDataset {
    fn compute(
        &self,
        &ExportData {
            convert_last_height_to_date,
            ..
        }: &ExportData,
    ) {
        self.to_vec()
            .into_iter()
            .for_each(|dataset| dataset.compute_date(convert_last_height_to_date));
    }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        self.to_vec()
            .iter()
            .map(|dataset| &dataset.height as &(dyn AnyHeightMap + Send + Sync))
            .collect_vec()
    }

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyExportableMap + Send + Sync)> {
        self.to_vec()
            .iter()
            .map(|dataset| *dataset as &(dyn AnyExportableMap + Send + Sync))
            .collect_vec()
    }
}

// ---
// STATE
// ---

#[derive(Default, Debug)]
pub struct PricePaidState {
    pub realized_cap: f32,

    pub pp_05p: Option<f32>,
    pub pp_10p: Option<f32>,
    pub pp_15p: Option<f32>,
    pub pp_20p: Option<f32>,
    pub pp_25p: Option<f32>,
    pub pp_30p: Option<f32>,
    pub pp_35p: Option<f32>,
    pub pp_40p: Option<f32>,
    pub pp_45p: Option<f32>,
    pub pp_median: Option<f32>,
    pub pp_55p: Option<f32>,
    pub pp_60p: Option<f32>,
    pub pp_65p: Option<f32>,
    pub pp_70p: Option<f32>,
    pub pp_75p: Option<f32>,
    pub pp_80p: Option<f32>,
    pub pp_85p: Option<f32>,
    pub pp_90p: Option<f32>,
    pub pp_95p: Option<f32>,

    pub processed_amount: u64,
}

impl PricePaidState {
    pub fn iterate(&mut self, price: f32, btc_amount: f32, sat_amount: u64, total_supply: u64) {
        let PricePaidState {
            processed_amount,
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
        } = self;

        *realized_cap += btc_amount * price;

        *processed_amount += sat_amount;

        if pp_95p.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.95 {
            pp_95p.replace(price);
        }

        if pp_90p.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.9 {
            pp_90p.replace(price);
        }

        if pp_85p.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.85 {
            pp_85p.replace(price);
        }

        if pp_80p.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.8 {
            pp_80p.replace(price);
        }

        if pp_75p.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.75 {
            pp_75p.replace(price);
        }

        if pp_70p.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.7 {
            pp_70p.replace(price);
        }

        if pp_65p.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.65 {
            pp_65p.replace(price);
        }

        if pp_60p.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.6 {
            pp_60p.replace(price);
        }

        if pp_55p.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.55 {
            pp_55p.replace(price);
        }

        if pp_median.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.5 {
            pp_median.replace(price);
        }

        if pp_45p.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.45 {
            pp_45p.replace(price);
        }

        if pp_40p.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.4 {
            pp_40p.replace(price);
        }

        if pp_35p.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.35 {
            pp_35p.replace(price);
        }

        if pp_30p.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.3 {
            pp_30p.replace(price);
        }

        if pp_25p.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.25 {
            pp_25p.replace(price);
        }

        if pp_20p.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.2 {
            pp_20p.replace(price);
        }

        if pp_15p.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.15 {
            pp_15p.replace(price);
        }

        if pp_10p.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.1 {
            pp_10p.replace(price);
        }

        if pp_05p.is_some() {
            return;
        }

        if *processed_amount as f32 >= total_supply as f32 * 0.05 {
            pp_05p.replace(price);
        }
    }
}
