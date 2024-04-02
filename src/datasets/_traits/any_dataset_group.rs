use super::AnyDataset;

pub trait AnyDatasetGroup {
    fn to_vec(&self) -> Vec<&(dyn AnyDataset + Send + Sync)>;
}
