use bitcoin_explorer::FTransaction;

use crate::structs::Outputs;

use super::convert_sats_to_bitcoins;

pub fn ftransaction_to_outputs(tx: &FTransaction) -> Outputs {
    tx.output
        .iter()
        .enumerate()
        .map(|(index, txout)| (index as u32, convert_sats_to_bitcoins(txout.value)))
        .collect()
}
