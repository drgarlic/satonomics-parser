use derive_deref::{Deref, DerefMut};

use crate::{datasets::UnrealizedState, parse::SplitByLiquidity};

use super::SplitByCohort;

#[derive(Deref, DerefMut, Default)]
pub struct SplitUnrealizedStates(pub SplitByCohort<SplitByLiquidity<UnrealizedState>>);
