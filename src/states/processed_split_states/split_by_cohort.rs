use crate::parse::{RawAddressSize, RawAddressSplit, RawAddressType};

#[derive(Default)]
pub struct SplitByCohort<T> {
    pub plankton: T,
    pub shrimp: T,
    pub crab: T,
    pub fish: T,
    pub shark: T,
    pub whale: T,
    pub humpback: T,
    pub megalodon: T,

    pub p2pk: T,
    pub p2pkh: T,
    pub p2sh: T,
    pub p2wpkh: T,
    pub p2wsh: T,
    pub p2tr: T,
}

impl<T> SplitByCohort<T> {
    pub fn get_state(&self, split: &RawAddressSplit) -> Option<&T> {
        match &split {
            RawAddressSplit::Type(address_type) => match address_type {
                RawAddressType::P2PK => Some(&self.p2pk),
                RawAddressType::P2PKH => Some(&self.p2pkh),
                RawAddressType::P2SH => Some(&self.p2sh),
                RawAddressType::P2WPKH => Some(&self.p2wpkh),
                RawAddressType::P2WSH => Some(&self.p2wsh),
                RawAddressType::P2TR => Some(&self.p2tr),
                _ => None,
            },
            RawAddressSplit::Size(address_size) => match address_size {
                RawAddressSize::Empty => None,
                RawAddressSize::Plankton => Some(&self.plankton),
                RawAddressSize::Shrimp => Some(&self.shrimp),
                RawAddressSize::Crab => Some(&self.crab),
                RawAddressSize::Fish => Some(&self.fish),
                RawAddressSize::Shark => Some(&self.shark),
                RawAddressSize::Whale => Some(&self.whale),
                RawAddressSize::Humpback => Some(&self.humpback),
                RawAddressSize::Megalodon => Some(&self.megalodon),
            },
        }
    }

    pub fn get_mut_state(&mut self, split: &RawAddressSplit) -> Option<&mut T> {
        match &split {
            RawAddressSplit::Type(address_type) => match address_type {
                RawAddressType::P2PK => Some(&mut self.p2pk),
                RawAddressType::P2PKH => Some(&mut self.p2pkh),
                RawAddressType::P2SH => Some(&mut self.p2sh),
                RawAddressType::P2WPKH => Some(&mut self.p2wpkh),
                RawAddressType::P2WSH => Some(&mut self.p2wsh),
                RawAddressType::P2TR => Some(&mut self.p2tr),
                _ => None,
            },
            RawAddressSplit::Size(address_size) => match address_size {
                RawAddressSize::Empty => None,
                RawAddressSize::Plankton => Some(&mut self.plankton),
                RawAddressSize::Shrimp => Some(&mut self.shrimp),
                RawAddressSize::Crab => Some(&mut self.crab),
                RawAddressSize::Fish => Some(&mut self.fish),
                RawAddressSize::Shark => Some(&mut self.shark),
                RawAddressSize::Whale => Some(&mut self.whale),
                RawAddressSize::Humpback => Some(&mut self.humpback),
                RawAddressSize::Megalodon => Some(&mut self.megalodon),
            },
        }
    }
}
