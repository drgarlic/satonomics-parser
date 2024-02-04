use std::{
    collections::BTreeMap,
    sync::{Arc, Weak},
};

use derive_deref::{Deref, DerefMut};
use ordered_float::OrderedFloat;
use parking_lot::{lock_api::MutexGuard, Mutex, RawMutex};

use crate::{states::AddressIndexToAddressData, structs::AddressData};

use super::{RawAddressSize, RawAddressType, WMutex};

type BTree = BTreeMap<(OrderedFloat<f32>, u32), Weak<WMutex<AddressData>>>;

// TODO: Move to states
// Add everything in order to not loop in adress
// For each liquidity
#[derive(Default, Deref, DerefMut, Debug)]
pub struct AddressIndexToAddressDataRef(BTree);

#[derive(Default)]
pub struct SplitAddressIndexToAddressDataRef {
    plankton: Mutex<AddressIndexToAddressDataRef>,
    shrimp: Mutex<AddressIndexToAddressDataRef>,
    crab: Mutex<AddressIndexToAddressDataRef>,
    fish: Mutex<AddressIndexToAddressDataRef>,
    shark: Mutex<AddressIndexToAddressDataRef>,
    whale: Mutex<AddressIndexToAddressDataRef>,
    humpback: Mutex<AddressIndexToAddressDataRef>,
    megalodon: Mutex<AddressIndexToAddressDataRef>,

    p2pk: Mutex<AddressIndexToAddressDataRef>,
    p2pkh: Mutex<AddressIndexToAddressDataRef>,
    p2sh: Mutex<AddressIndexToAddressDataRef>,
    p2wpkh: Mutex<AddressIndexToAddressDataRef>,
    p2wsh: Mutex<AddressIndexToAddressDataRef>,
    p2tr: Mutex<AddressIndexToAddressDataRef>,
}

pub enum RawAddressSplit {
    Type(RawAddressType),
    Size(RawAddressSize),
}

impl SplitAddressIndexToAddressDataRef {
    pub fn init(index_to_address_data: &AddressIndexToAddressData) -> Self {
        let s = Self::default();

        index_to_address_data
            .iter()
            .for_each(|(index, address_data)| {
                let (mean_price_paid, address_type, address_size) = {
                    let address_data = address_data.lock();
                    (
                        address_data.mean_price_paid,
                        address_data.address_type,
                        RawAddressSize::from_amount(address_data.amount),
                    )
                };

                s.insert(
                    *index,
                    mean_price_paid,
                    &address_type,
                    &address_size,
                    address_data,
                );
            });

        s
    }

    pub fn get(
        &self,
        split: &RawAddressSplit,
    ) -> MutexGuard<'_, RawMutex, AddressIndexToAddressDataRef> {
        match &split {
            RawAddressSplit::Type(t) => match t {
                RawAddressType::P2PK => self.p2pk.lock(),
                RawAddressType::P2PKH => self.p2pkh.lock(),
                RawAddressType::P2SH => self.p2sh.lock(),
                RawAddressType::P2WPKH => self.p2wpkh.lock(),
                RawAddressType::P2WSH => self.p2wsh.lock(),
                RawAddressType::P2TR => self.p2tr.lock(),
                _ => panic!(),
            },
            RawAddressSplit::Size(address_size) => match address_size {
                RawAddressSize::Empty => panic!("Cannot insert to empty"),
                RawAddressSize::Plankton => self.plankton.lock(),
                RawAddressSize::Shrimp => self.shrimp.lock(),
                RawAddressSize::Crab => self.crab.lock(),
                RawAddressSize::Fish => self.fish.lock(),
                RawAddressSize::Shark => self.shark.lock(),
                RawAddressSize::Whale => self.whale.lock(),
                RawAddressSize::Humpback => self.humpback.lock(),
                RawAddressSize::Megalodon => self.megalodon.lock(),
            },
        }
    }

    pub fn insert(
        &self,
        index: u32,
        mean_price_paid: f32,
        address_type: &RawAddressType,
        address_size: &RawAddressSize,
        address_data: &Arc<WMutex<AddressData>>,
    ) {
        self.insert_to_type(index, mean_price_paid, address_type, address_data);
        self.insert_to_size(index, mean_price_paid, address_size, address_data);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn replace(
        &self,
        index: u32,
        previous_mean_price_paid: f32,
        current_mean_price_paid: f32,
        previous_size: &RawAddressSize,
        current_size: &RawAddressSize,
        address_type: &RawAddressType,
        address_data: &Arc<WMutex<AddressData>>,
        was_removed: bool,
    ) {
        // if index == 73228051 {
        //     println!("73228051 index = {index}, previous_mean_price_paid = {previous_mean_price_paid}, current_mean_price_paid = {current_mean_price_paid}, previous_size = {previous_size:?}, current_size = {current_size:?}, address_type = {address_type:?}, was_removed = {was_removed}")
        // }

        if was_removed
            || current_size != previous_size
            || current_mean_price_paid != previous_mean_price_paid
        {
            self.remove_from_size(index, previous_mean_price_paid, previous_size)
                .unwrap_or_else(|| {
                    panic!("Fail: index = {index}, previous_mean_price_paid = {previous_mean_price_paid}, current_mean_price_paid = {current_mean_price_paid}, previous_size = {previous_size:?}, current_size = {current_size:?}, address_type = {address_type:?}")
                });
            self.insert_to_size(index, current_mean_price_paid, current_size, address_data);
        }

        if was_removed || current_mean_price_paid != previous_mean_price_paid {
            self.remove_from_type(index, previous_mean_price_paid, address_type)
                .unwrap_or_else(|| {
                    panic!("Fail: index = {index}, previous_mean_price_paid = {previous_mean_price_paid}, current_mean_price_paid = {current_mean_price_paid}, previous_size = {previous_size:?}, current_size = {current_size:?}, address_type = {address_type:?}")
                });
            self.insert_to_type(index, current_mean_price_paid, address_type, address_data);
        }
    }

    pub fn remove(
        &self,
        index: u32,
        mean_price_paid: f32,
        address_size: &RawAddressSize,
        address_type: &RawAddressType,
    ) {
        self.remove_from_size(index, mean_price_paid, address_size)
            .unwrap_or_else(|| {
                panic!("Fail: index = {index}, mean_price_paid = {mean_price_paid}, address_size = {address_size:?}, address_type = {address_type:?}")
            });
        self.remove_from_type(index, mean_price_paid, address_type)
            .unwrap_or_else(|| {
                panic!("Fail: index = {index}, mean_price_paid = {mean_price_paid}, address_size = {address_size:?}, address_type = {address_type:?}")
            });
    }

    pub fn insert_to_type(
        &self,
        index: u32,
        mean_price_paid: f32,
        address_type: &RawAddressType,
        address_data: &Arc<WMutex<AddressData>>,
    ) -> Option<Weak<WMutex<AddressData>>> {
        let key = (OrderedFloat(mean_price_paid), index);

        // if index == 73228051 {
        //     println!("73228051 address_type: {address_type:?}")
        // }

        let weak_address_data = Arc::downgrade(address_data);

        match address_type {
            RawAddressType::P2PK => self.p2pk.lock().insert(key, weak_address_data),
            RawAddressType::P2PKH => self.p2pkh.lock().insert(key, weak_address_data),
            RawAddressType::P2SH => self.p2sh.lock().insert(key, weak_address_data),
            RawAddressType::P2WPKH => self.p2wpkh.lock().insert(key, weak_address_data),
            RawAddressType::P2WSH => self.p2wsh.lock().insert(key, weak_address_data),
            RawAddressType::P2TR => self.p2tr.lock().insert(key, weak_address_data),
            RawAddressType::Empty | RawAddressType::MultiSig | RawAddressType::Unknown => {
                Some(Weak::default())
            }
        }
    }

    fn insert_to_size(
        &self,
        index: u32,
        mean_price_paid: f32,
        address_size: &RawAddressSize,
        address_data: &Arc<WMutex<AddressData>>,
    ) -> Option<Weak<WMutex<AddressData>>> {
        let key = (OrderedFloat(mean_price_paid), index);

        // if index == 73228051 {
        //     println!("73228051 address_size: {address_size:?}")
        // }

        let weak_address_data = Arc::downgrade(address_data);

        match address_size {
            RawAddressSize::Empty => panic!("Cannot insert to empty"),
            RawAddressSize::Plankton => self.plankton.lock().insert(key, weak_address_data),
            RawAddressSize::Shrimp => self.shrimp.lock().insert(key, weak_address_data),
            RawAddressSize::Crab => self.crab.lock().insert(key, weak_address_data),
            RawAddressSize::Fish => self.fish.lock().insert(key, weak_address_data),
            RawAddressSize::Shark => self.shark.lock().insert(key, weak_address_data),
            RawAddressSize::Whale => self.whale.lock().insert(key, weak_address_data),
            RawAddressSize::Humpback => self.humpback.lock().insert(key, weak_address_data),
            RawAddressSize::Megalodon => self.megalodon.lock().insert(key, weak_address_data),
        }
    }

    fn remove_from_type(
        &self,
        index: u32,
        mean_price_paid: f32,
        address_type: &RawAddressType,
    ) -> Option<Weak<WMutex<AddressData>>> {
        let key = (OrderedFloat(mean_price_paid), index);

        match address_type {
            RawAddressType::P2PK => self.p2pk.lock().remove(&key),
            RawAddressType::P2PKH => self.p2pkh.lock().remove(&key),
            RawAddressType::P2SH => self.p2sh.lock().remove(&key),
            RawAddressType::P2WPKH => self.p2wpkh.lock().remove(&key),
            RawAddressType::P2WSH => self.p2wsh.lock().remove(&key),
            RawAddressType::P2TR => self.p2tr.lock().remove(&key),
            RawAddressType::Empty | RawAddressType::MultiSig | RawAddressType::Unknown => {
                Some(Weak::default())
            }
        }
    }

    fn remove_from_size(
        &self,
        index: u32,
        mean_price_paid: f32,
        address_size: &RawAddressSize,
    ) -> Option<Weak<WMutex<AddressData>>> {
        let key = (OrderedFloat(mean_price_paid), index);

        match address_size {
            RawAddressSize::Empty => panic!("Cannot remove empty"),
            RawAddressSize::Plankton => self.plankton.lock().remove(&key),
            RawAddressSize::Shrimp => self.shrimp.lock().remove(&key),
            RawAddressSize::Crab => self.crab.lock().remove(&key),
            RawAddressSize::Fish => self.fish.lock().remove(&key),
            RawAddressSize::Shark => self.shark.lock().remove(&key),
            RawAddressSize::Whale => self.whale.lock().remove(&key),
            RawAddressSize::Humpback => self.humpback.lock().remove(&key),
            RawAddressSize::Megalodon => self.megalodon.lock().remove(&key),
        }
    }

    #[allow(unused)]
    pub fn debug_lens(&self) {
        let plankton_len = self.plankton.lock().len();
        let shrimp_len = self.shrimp.lock().len();
        let crab_len = self.crab.lock().len();
        let fish_len = self.fish.lock().len();
        let shark_len = self.shark.lock().len();
        let whale_len = self.whale.lock().len();
        let humpback_len = self.humpback.lock().len();
        let megalodon_len = self.megalodon.lock().len();
        let total_size_len = plankton_len
            + shrimp_len
            + crab_len
            + fish_len
            + shark_len
            + whale_len
            + humpback_len
            + megalodon_len;

        let p2pk_len = self.p2pk.lock().len();
        let p2pkh_len = self.p2pkh.lock().len();
        let p2sh_len = self.p2sh.lock().len();
        let p2wpkh_len = self.p2wpkh.lock().len();
        let p2wsh_len = self.p2wsh.lock().len();
        let p2tr_len = self.p2tr.lock().len();
        let total_type_len = p2pk_len + p2pkh_len + p2sh_len + p2wpkh_len + p2wsh_len + p2tr_len;

        dbg!(
            plankton_len,
            shrimp_len,
            crab_len,
            fish_len,
            shark_len,
            whale_len,
            humpback_len,
            megalodon_len,
            total_size_len,
            "",
            p2pk_len,
            p2pkh_len,
            p2sh_len,
            p2wpkh_len,
            p2wsh_len,
            p2tr_len,
            total_type_len
        );
    }
}
