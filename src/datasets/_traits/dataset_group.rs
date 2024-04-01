use super::AnyDataset;

pub trait DatasetGroup {
    fn to_vec(&self) -> Vec<&(dyn AnyDataset + Send + Sync)>;
}
