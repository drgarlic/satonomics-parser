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
