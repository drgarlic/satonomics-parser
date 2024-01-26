use std::{collections::BTreeMap, path::Path, sync::RwLock};

use chrono::{Days, NaiveDate};
// use itertools::Itertools;
use serde::{de::DeserializeOwned, Serialize};

use crate::{structs::Json, utils::string_to_naive_date};

// Should use number of unsafe blocks instead of avoid useless re-computation
// Actually maybe not ?
const NUMBER_OF_UNSAFE_DATES: usize = 2;

pub struct DateMap<T> {
    batch: RwLock<Vec<(NaiveDate, T)>>,
    path: String,
    initial_first_unsafe_date: Option<NaiveDate>,
}

impl<T> DateMap<T>
where
    T: Clone + DeserializeOwned + Serialize + Default,
{
    pub fn new(path: &str) -> Self {
        Self {
            batch: RwLock::new(vec![]),
            initial_first_unsafe_date: get_first_unsafe_date::<T, _>(&path),
            path: path.to_string(),
        }
    }

    pub fn insert(&self, date: NaiveDate, value: T) {
        if self
            .initial_first_unsafe_date
            .map_or(true, |initial_first_unsafe_date| {
                initial_first_unsafe_date <= date
            })
        {
            self.batch.write().unwrap().push((date, value));
        }
    }

    // pub fn get_min_first_unsafe_date(list: &[&Self]) -> Option<NaiveDate> {
    //     let first_unsafe_date_opts = list
    //         .iter()
    //         .map(|map| map.get_first_unsafe_date())
    //         .collect_vec();

    //     if first_unsafe_date_opts.iter().all(|opt| opt.is_some()) {
    //         first_unsafe_date_opts
    //             .iter()
    //             .map(|first_unsafe_date_opt| first_unsafe_date_opt.unwrap())
    //             .min()
    //     } else {
    //         None
    //     }
    // }

    pub fn import(&self) -> BTreeMap<String, T> {
        Json::import_map(&self.path)
    }
}

pub trait AnyDateMap {
    fn get_initial_first_unsafe_date(&self) -> Option<NaiveDate>;

    fn get_last_date(&self) -> Option<NaiveDate>;

    fn get_first_unsafe_date(&self) -> Option<NaiveDate>;

    fn export(&self) -> color_eyre::Result<()>;
}

impl<T> AnyDateMap for DateMap<T>
where
    T: Clone + DeserializeOwned + Serialize + Default,
{
    fn get_last_date(&self) -> Option<NaiveDate> {
        get_last_date::<T, _>(&self.path)
    }

    fn get_first_unsafe_date(&self) -> Option<NaiveDate> {
        get_first_unsafe_date::<T, _>(&self.path)
    }

    fn get_initial_first_unsafe_date(&self) -> Option<NaiveDate> {
        self.initial_first_unsafe_date
    }

    fn export(&self) -> color_eyre::Result<()> {
        let mut map = self.import();

        self.batch
            .write()
            .unwrap()
            .drain(..)
            .for_each(|(date, value)| {
                map.insert(date.to_string(), value);
            });

        Json::export(&self.path, &map)
    }
}

fn get_first_unsafe_date<T, P>(path: P) -> Option<NaiveDate>
where
    T: Clone + DeserializeOwned + Serialize,
    P: AsRef<Path>,
{
    get_last_date::<T, P>(path).and_then(|last_date| {
        let offset = NUMBER_OF_UNSAFE_DATES - 1;

        last_date.checked_sub_days(Days::new(offset as u64))
    })
}

fn get_last_date<T, P>(path: P) -> Option<NaiveDate>
where
    T: Clone + DeserializeOwned + Serialize,
    P: AsRef<Path>,
{
    Json::import_map::<T, P>(path)
        .keys()
        .map(|date| string_to_naive_date(date))
        .max()
}
