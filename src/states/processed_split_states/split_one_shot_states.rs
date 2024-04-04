use derive_deref::{Deref, DerefMut};

use crate::parse::SplitByLiquidity;

use super::{OneShotStates, SplitByCohort};

#[derive(Deref, DerefMut, Default)]
pub struct SplitOneShotStates(pub SplitByCohort<SplitByLiquidity<OneShotStates>>);
