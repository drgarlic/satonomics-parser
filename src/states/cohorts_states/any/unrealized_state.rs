use std::ops::Add;

#[derive(Debug, Default)]
pub struct UnrealizedState {
    pub supply_in_profit: u64,
    pub unrealized_profit: f32,
    pub unrealized_loss: f32,
}

impl UnrealizedState {
    #[inline]
    pub fn iterate(&mut self, price_then: f32, price_now: f32, sat_amount: u64, btc_amount: f32) {
        if price_then < price_now {
            self.unrealized_profit += btc_amount * (price_now - price_then);
            self.supply_in_profit += sat_amount;
        } else if price_then > price_now {
            self.unrealized_loss += btc_amount * (price_then - price_now);
        }
    }
}

impl Add<UnrealizedState> for UnrealizedState {
    type Output = UnrealizedState;

    fn add(self, other: UnrealizedState) -> UnrealizedState {
        UnrealizedState {
            supply_in_profit: self.supply_in_profit + other.supply_in_profit,
            unrealized_profit: self.unrealized_profit + other.unrealized_profit,
            unrealized_loss: self.unrealized_loss + other.unrealized_loss,
        }
    }
}
