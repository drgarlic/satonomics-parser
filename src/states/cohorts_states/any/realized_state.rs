#[derive(Debug, Default)]
pub struct RealizedState {
    pub realized_profit: f32,
    pub realized_loss: f32,
}

impl RealizedState {
    pub fn iterate(&mut self, realized_profit: f32, realized_loss: f32) {
        self.realized_profit += realized_profit;
        self.realized_loss += realized_loss;
    }
}
