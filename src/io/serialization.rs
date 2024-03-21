use std::{fmt::Debug, path::Path};

use bincode::{Decode, Encode};
use serde::{de::DeserializeOwned, Serialize};

use crate::io::{Binary, Json};

#[derive(PartialEq, PartialOrd, Ord, Eq)]
pub enum Serialization {
    Binary,
    Json,
}

impl Serialization {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Binary => "bin",
            Self::Json => "json",
        }
    }

    pub fn append_extension(&self, path: &str) -> String {
        format!("{path}.{}", self.to_str())
    }

    pub fn import<T, P>(&self, path: P) -> color_eyre::Result<T>
    where
        T: Decode + DeserializeOwned + Debug,
        P: AsRef<Path> + Debug,
    {
        match self {
            Serialization::Binary => Binary::import(path),
            Serialization::Json => Json::import(&path),
        }
    }

    pub fn export<T, P>(&self, path: P, value: &T) -> color_eyre::Result<()>
    where
        T: Encode + Serialize,
        P: AsRef<Path> + Debug,
    {
        match self {
            Serialization::Binary => Binary::export(path, value),
            Serialization::Json => Json::export(&path, value),
        }
    }
}
