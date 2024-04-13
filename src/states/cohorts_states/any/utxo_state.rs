#[derive(Debug, Default)]
pub struct UTXOState {
    pub count: usize,
}

impl UTXOState {
    pub fn increment(&mut self, utxo_count: usize) {
        self.count += utxo_count;
    }

    pub fn decrement(&mut self, utxo_count: usize) {
        self.count -= utxo_count;
    }
}
