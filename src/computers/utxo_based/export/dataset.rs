use super::DatasetInsertData;

pub trait Dataset {
    fn get_min_last_height(&self) -> Option<usize>;

    fn get_min_initial_first_unsafe_height(&self) -> Option<usize>;

    fn export(&self) -> color_eyre::Result<()>;

    fn insert(&self, insert_data: &DatasetInsertData);
}
