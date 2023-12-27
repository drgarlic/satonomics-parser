use bitcoin::{util::address::Payload, Address};
use itertools::Itertools;

pub fn convert_sats_to_bitcoins(sats: u64) -> f64 {
    sats as f64 / 100_000_000.0
}

// TODO: Return slice instead of vec
pub fn addresses_to_bytes(addresses: &[Address]) -> Vec<u8> {
    if addresses.is_empty() {
        panic!("addresses_to_bytes got empty addresses array");
    } else if addresses.len() == 1 {
        address_to_array(addresses.first().unwrap())
    } else {
        addresses
            .iter()
            .map(address_to_array)
            .sorted_unstable()
            .concat()
    }
}

fn address_to_array(address: &Address) -> Vec<u8> {
    match address.payload.to_owned() {
        Payload::ScriptHash(hash) => hash.to_vec(),
        Payload::PubkeyHash(hash) => hash.to_vec(),
        Payload::WitnessProgram { program, .. } => program,
    }
}
