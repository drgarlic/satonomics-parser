use crate::structs::HeightMap;

pub struct HeightToRewards {
    pub height_to_fees: HeightMap<f64>,
    pub height_to_subsidy: HeightMap<f64>,
}

impl HeightToRewards {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self {
            height_to_fees: HeightMap::new("height_to_fees.json"),
            height_to_subsidy: HeightMap::new("height_to_subsidy.json"),
        })
    }

    pub fn get_min_unsafe_height(&self) -> Option<usize> {
        [
            &self.height_to_fees.get_first_unsafe_height(),
            &self.height_to_subsidy.get_first_unsafe_height(),
        ]
        .iter()
        .min()
        .and_then(|opt| **opt)
    }

    pub fn insert(&self, height: usize, coinbase: f64, fees: f64) {
        let subsidy = coinbase - fees;

        self.height_to_fees.insert(height, fees);
        self.height_to_subsidy.insert(height, subsidy);
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        self.height_to_fees.export()?;
        self.height_to_subsidy.export()?;

        Ok(())
    }
}
