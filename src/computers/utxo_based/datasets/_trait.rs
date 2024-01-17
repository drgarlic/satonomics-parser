use crate::structs::AnyHeightMap;

pub trait HeightDataset<InsertData> {
    fn get_min_last_height(&self) -> Option<usize> {
        self.to_vec()
            .iter()
            .map(|dataset| dataset.get_last_height())
            .min()
            .and_then(|opt| opt)
    }

    fn get_min_initial_first_unsafe_height(&self) -> Option<usize> {
        self.to_vec()
            .iter()
            .map(|dataset| dataset.get_initial_first_unsafe_height())
            .min()
            .and_then(|opt| opt)
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.to_vec()
            .iter()
            .try_for_each(|dataset| dataset.export())
    }

    fn insert(&self, insert_data: &InsertData);

    fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)>;
}
