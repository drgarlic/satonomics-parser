use crate::parse::{AddressData, AddressSize, AddressSplit, AddressType};

#[derive(Default)]
pub struct SplitByAddressCohort<T> {
    pub all: T,

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

impl<T> SplitByAddressCohort<T> {
    pub fn get_state(&self, split: &AddressSplit) -> Option<&T> {
        match &split {
            AddressSplit::All => Some(&self.all),

            AddressSplit::Type(address_type) => match address_type {
                AddressType::P2PK => Some(&self.p2pk),
                AddressType::P2PKH => Some(&self.p2pkh),
                AddressType::P2SH => Some(&self.p2sh),
                AddressType::P2WPKH => Some(&self.p2wpkh),
                AddressType::P2WSH => Some(&self.p2wsh),
                AddressType::P2TR => Some(&self.p2tr),
                AddressType::MultiSig => None,
                AddressType::Unknown => None,
                AddressType::Empty => None,
            },

            AddressSplit::Size(address_size) => match address_size {
                AddressSize::Plankton => Some(&self.plankton),
                AddressSize::Shrimp => Some(&self.shrimp),
                AddressSize::Crab => Some(&self.crab),
                AddressSize::Fish => Some(&self.fish),
                AddressSize::Shark => Some(&self.shark),
                AddressSize::Whale => Some(&self.whale),
                AddressSize::Humpback => Some(&self.humpback),
                AddressSize::Megalodon => Some(&self.megalodon),
                AddressSize::Empty => None,
            },
        }
    }

    pub fn iterate(&mut self, address_data: &AddressData, iterate: impl Fn(&mut T)) {
        if let Some(state) = self.get_mut_state(&AddressSplit::All) {
            iterate(state);
        }

        if let Some(state) = self.get_mut_state(&AddressSplit::Type(address_data.address_type)) {
            iterate(state);
        }

        if let Some(state) = self.get_mut_state(&AddressSplit::Size(AddressSize::from_amount(
            address_data.amount,
        ))) {
            iterate(state);
        }
    }

    fn get_mut_state(&mut self, split: &AddressSplit) -> Option<&mut T> {
        match &split {
            AddressSplit::All => Some(&mut self.all),

            AddressSplit::Type(address_type) => match address_type {
                AddressType::P2PK => Some(&mut self.p2pk),
                AddressType::P2PKH => Some(&mut self.p2pkh),
                AddressType::P2SH => Some(&mut self.p2sh),
                AddressType::P2WPKH => Some(&mut self.p2wpkh),
                AddressType::P2WSH => Some(&mut self.p2wsh),
                AddressType::P2TR => Some(&mut self.p2tr),
                AddressType::MultiSig => None,
                AddressType::Unknown => None,
                AddressType::Empty => None,
            },

            AddressSplit::Size(address_size) => match address_size {
                AddressSize::Plankton => Some(&mut self.plankton),
                AddressSize::Shrimp => Some(&mut self.shrimp),
                AddressSize::Crab => Some(&mut self.crab),
                AddressSize::Fish => Some(&mut self.fish),
                AddressSize::Shark => Some(&mut self.shark),
                AddressSize::Whale => Some(&mut self.whale),
                AddressSize::Humpback => Some(&mut self.humpback),
                AddressSize::Megalodon => Some(&mut self.megalodon),
                AddressSize::Empty => None,
            },
        }
    }
}
