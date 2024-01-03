use bitcoin::{address::Payload, Address, Network, Script, TxOut};
use bitcoin_hashes::{hash160, Hash};

use super::multisig_addresses;

pub fn txout_to_addresses(txout: &TxOut) -> Box<[Address]> {
    let script = &txout.script_pubkey;

    // if script.is_p2pk() {
    //     println!("p2pk");
    // } else if script.is_p2pkh() {
    //     println!("p2pkh");
    // } else if script.is_p2sh() {
    //     println!("p2sh");
    // } else if script.is_v0_p2wpkh() {
    //     println!("p2wpkh");
    // } else if script.is_v0_p2wsh() {
    //     println!("p2wsh");
    // } else if script.is_witness_program() {
    //     println!("witness");
    // } else if script.is_op_return() {
    //     println!("opreturn");
    // } else if script.is_provably_unspendable() {
    //     println!("unspendable");
    // } else if is_multisig(script) {
    //     println!("mutli");
    // } else {
    //     println!("unknown");
    // }

    match Address::from_script(script, Network::Bitcoin) {
        Ok(address) => Box::new([address]),
        Err(_) => {
            if script.is_p2pk() {
                Box::new([p2pk_script_to_address(script)])
            } else if script.is_multisig() {
                multisig_addresses(script).into_boxed_slice()
            } else {
                Box::new([])
            }
        }
    }
}

fn p2pk_script_to_address(script: &Script) -> Address {
    let pk = match script.as_bytes().len() {
        67 => &script.as_bytes()[1..66],
        35 => &script.as_bytes()[1..34],
        _ => unreachable!(),
    };

    let pkh = hash160::Hash::hash(pk);

    Address::new(Network::Bitcoin, Payload::PubkeyHash(pkh.into()))
}
