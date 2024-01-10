use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
};

use chrono::{Days, NaiveDate};
use itertools::Itertools;
use serde::{de::DeserializeOwned, Serialize};

use crate::utils::{string_to_naive_date, Json, EXPORTS_FOLDER_PATH};

// Should use number of unsafe blocks instead of avoid useless re-computation
const NUMBER_OF_UNSAFE_DATES: usize = 2;

pub struct DateMap<T> {
    map: RefCell<HashMap<String, T>>,
    path: PathBuf,
}

impl<T> DateMap<T>
where
    T: Clone + DeserializeOwned + Serialize,
{
    pub fn import(path: &str) -> Self {
        let path = Path::new(EXPORTS_FOLDER_PATH).join(path);

        Self {
            map: RefCell::new(Json::import_map(&path)),
            path: path.to_owned(),
        }
    }

    pub fn insert(&self, date: &NaiveDate, value: T) -> Option<T> {
        // TODO: Insert only if needed, check safe date, do the same in HeightMap

        let opt = self.map.borrow_mut().insert(date.to_string(), value);

        if date.format("%d").to_string().parse::<u32>().unwrap() == 1 {
            self.export().expect("JSON export to work");
        }

        opt
    }

    pub fn get(&self, date: &NaiveDate) -> Option<T> {
        self.map.borrow().get(&date.to_string()).cloned()
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        Json::export(
            &self.path,
            &self
                .map
                .borrow()
                .iter()
                .sorted_unstable_by(|a, b| {
                    string_to_naive_date(a.0)
                        .partial_cmp(&string_to_naive_date(b.0))
                        .unwrap()
                })
                .collect::<BTreeMap<_, _>>(),
        )
    }

    pub fn get_last_date(&self) -> Option<NaiveDate> {
        self.map
            .borrow()
            .keys()
            .map(|date| string_to_naive_date(date))
            .max()
    }

    pub fn get_first_unsafe_date(&self) -> Option<NaiveDate> {
        self.get_last_date().and_then(|last_date| {
            let offset = NUMBER_OF_UNSAFE_DATES - 1;

            last_date.checked_sub_days(Days::new(offset as u64))
        })
    }

    pub fn get_min_first_unsafe_date(list: &[&Self]) -> Option<NaiveDate> {
        let first_unsafe_date_opts = list
            .iter()
            .map(|map| map.get_first_unsafe_date())
            .collect_vec();

        if first_unsafe_date_opts.iter().all(|opt| opt.is_some()) {
            first_unsafe_date_opts
                .iter()
                .map(|first_unsafe_date_opt| first_unsafe_date_opt.unwrap())
                .min()
        } else {
            None
        }
    }
}
