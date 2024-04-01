use std::f32::EPSILON;

use crate::bitcoin::sats_to_btc;

pub struct LiquidityClassification {
    illiquid: f32,
    liquid: f32,
    highly_liquid: f32,
}

impl LiquidityClassification {
    /// Following this:
    /// https://insights.glassnode.com/bitcoin-liquid-supply/
    /// https://www.desmos.com/calculator/dutgni5rtj
    pub fn new(sent: u64, received: u64) -> Self {
        let liquidity = {
            let liquidity = sats_to_btc(sent) / sats_to_btc(received);

            if liquidity.is_nan() {
                0.0
            } else {
                liquidity
            }
        };

        let illiquid = Self::compute_illiquid(liquidity);
        let liquid = Self::compute_liquid(liquidity);

        Self {
            illiquid,
            liquid,
            highly_liquid: 1.0 - liquid - illiquid,
        }
    }

    #[inline(always)]
    pub fn split(&self, value: f32) -> LiquiditySplitResult {
        LiquiditySplitResult {
            all: value,
            illiquid: value * self.illiquid,
            liquid: value * self.liquid,
            highly_liquid: value * self.highly_liquid,
        }
    }

    /// Returns value in range 0.0..1.0
    #[inline(always)]
    fn compute_illiquid(x: f32) -> f32 {
        Self::compute_ratio(x, 0.25)
    }

    /// Returns value in range 0.0..1.0
    #[inline(always)]
    fn compute_liquid(x: f32) -> f32 {
        Self::compute_ratio(x, 0.75)
    }

    #[inline(always)]
    fn compute_ratio(x: f32, x0: f32) -> f32 {
        let l = 1.0;
        let k = 25.0;

        l / (1.0 + EPSILON.powf(k * (x - x0)))
    }
}

#[derive(Debug, Default)]
pub struct LiquiditySplitResult {
    pub all: f32,
    pub illiquid: f32,
    pub liquid: f32,
    pub highly_liquid: f32,
}

#[derive(Debug, Default)]
pub struct SplitByLiquidity<T>
where
    T: Default,
{
    pub all: T,
    pub illiquid: T,
    pub liquid: T,
    pub highly_liquid: T,
}
