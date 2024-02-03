pub enum RawAddressSize {
    Plankton,
    Shrimp,
    Crab,
    Fish,
    Shark,
    Whale,
    Humpback,
    Megalodon,
}

impl RawAddressSize {
    pub fn to_name(&self) -> &str {
        match &self {
            Self::Plankton => "plankton",
            Self::Shrimp => "shrimp",
            Self::Crab => "crab",
            Self::Fish => "fish",
            Self::Shark => "shark",
            Self::Whale => "whale",
            Self::Humpback => "humpback",
            Self::Megalodon => "megalodon",
        }
    }
}
