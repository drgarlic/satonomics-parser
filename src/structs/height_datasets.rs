use super::HeightDataset;

pub trait HeightDatasets<InsertData> {
    fn get_min_last_height(&self) -> Option<usize>;

    fn export_if_needed(&self, height: Option<usize>) -> color_eyre::Result<()>;

    fn export(&self) -> color_eyre::Result<()>;

    fn insert(&self, insert_data: InsertData);

    fn to_vec(&self) -> Vec<&(dyn HeightDataset<InsertData> + Send + Sync)>;
}
