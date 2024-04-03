use derive_deref::{Deref, DerefMut};

use crate::{datasets::PricePaidState, parse::SplitByLiquidity};

use super::SplitByCohort;

#[derive(Deref, DerefMut, Default)]
pub struct SplitPricePaidStates(pub SplitByCohort<SplitByLiquidity<PricePaidState>>);
