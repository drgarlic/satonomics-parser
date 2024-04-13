#[derive(Debug, Default)]
pub struct SupplyState {
    pub supply: u64,
}

impl SupplyState {
    pub fn increment(&mut self, amount: u64) {
        self.supply += amount;
    }

    pub fn decrement(&mut self, amount: u64) {
        self.supply -= amount;
    }
}
