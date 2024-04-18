use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

use crate::io::{Binary, Json};

#[derive(PartialEq, PartialOrd, Ord, Eq)]
pub enum Serialization {
    Binary,
    Json,
}

impl Serialization {
    pub fn to_extension(&self) -> &str {
        match self {
            Self::Binary => "bin",
            Self::Json => "json",
        }
    }

    pub fn from_extension(extension: &str) -> Self {
        match extension {
            "bin" => Self::Binary,
            "json" => Self::Json,
            _ => panic!("Extension \"{extension}\" isn't supported"),
        }
    }

    pub fn append_extension(&self, path: &str) -> String {
        format!("{path}.{}", self.to_extension())
    }

    pub fn import<T>(&self, path: &str) -> color_eyre::Result<T>
    where
        T: savefile::Deserialize + DeserializeOwned + Debug,
    {
        match self {
            Serialization::Binary => Binary::import(path),
            Serialization::Json => Json::import(path),
        }
    }

    pub fn export<T>(&self, path: &str, value: &T) -> color_eyre::Result<()>
    where
        T: savefile::Serialize + Serialize,
    {
        match self {
            Serialization::Binary => Binary::export(path, value),
            Serialization::Json => Json::export(path, value),
        }
    }
}
