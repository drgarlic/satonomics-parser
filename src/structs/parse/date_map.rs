use std::{collections::BTreeMap, fmt::Debug, fs};

use bincode::{Decode, Encode};
use chrono::{Days, NaiveDate};
use parking_lot::{lock_api::MutexGuard, RawMutex};
use serde::{de::DeserializeOwned, Serialize};

use crate::{structs::Serialization, utils::string_to_naive_date};

use super::WMutex;

// Should use number of unsafe blocks instead of avoid useless re-computation
// Actually maybe not ?
const NUMBER_OF_UNSAFE_DATES: usize = 2;

pub struct DateMap<T> {
    batch: WMutex<Vec<(NaiveDate, T)>>,
    path: String,
    initial_last_date: Option<NaiveDate>,
    initial_first_unsafe_date: Option<NaiveDate>,
    inner: Option<WMutex<BTreeMap<String, T>>>,
    called_insert: WMutex<bool>,
    serialization: Serialization,
}

impl<T> DateMap<T>
where
    T: Clone + Default + Encode + Decode + Debug + Serialize + DeserializeOwned,
{
    #[allow(unused)]
    pub fn new_on_disk_bin(path: &str) -> Self {
        Self::new(path, false, Serialization::Binary)
    }

    #[allow(unused)]
    pub fn new_in_memory_bin(path: &str) -> Self {
        Self::new(path, true, Serialization::Binary)
    }

    #[allow(unused)]
    pub fn new_on_disk_json(path: &str) -> Self {
        Self::new(path, false, Serialization::Json)
    }

    #[allow(unused)]
    pub fn new_in_memory_json(path: &str) -> Self {
        Self::new(path, true, Serialization::Json)
    }

    fn new(path: &str, in_memory: bool, serialization: Serialization) -> Self {
        fs::create_dir_all(path).unwrap();

        let mut s = Self {
            batch: WMutex::new(vec![]),
            initial_last_date: None,
            initial_first_unsafe_date: None,
            path: serialization.append_extension(&format!("{path}/date")),
            inner: None,
            called_insert: WMutex::new(false),
            serialization,
        };

        if in_memory {
            s.inner.replace(WMutex::new(s.import()));
        }

        s.initial_last_date = s.get_last_date();
        s.initial_first_unsafe_date = last_date_to_first_unsafe_date(s.initial_last_date);

        // dbg!(&s.path, &s.initial_first_unsafe_date);

        s
    }

    pub fn insert(&self, date: NaiveDate, value: T) {
        // dbg!(&self.path);

        if !self.is_date_safe(date) {
            *self.called_insert.lock() = true;

            if let Some(map) = &self.inner {
                map.lock().insert(date.to_string(), value);
            } else {
                self.batch.lock().push((date, value));
            }
        }
    }

    #[allow(unused)]
    pub fn insert_default(&self, date: NaiveDate) {
        self.insert(date, T::default())
    }

    #[inline(always)]
    pub fn is_date_safe(&self, date: NaiveDate) -> bool {
        self.initial_first_unsafe_date
            .map_or(false, |initial_first_unsafe_date| {
                initial_first_unsafe_date > date
            })
    }

    pub fn unsafe_inner(&self) -> MutexGuard<'_, RawMutex, BTreeMap<String, T>> {
        self.inner.as_ref().unwrap().lock()
    }

    pub fn import(&self) -> BTreeMap<String, T> {
        self.serialization.import(&self.path).unwrap_or_default()
    }

    fn get_first_unsafe_date(&self) -> Option<NaiveDate> {
        last_date_to_first_unsafe_date(self.get_last_date())
    }

    fn get_last_date(&self) -> Option<NaiveDate> {
        if self.inner.is_some() {
            self.unsafe_inner()
                .keys()
                .map(|date| string_to_naive_date(date))
                .max()
        } else {
            self.import()
                .keys()
                .map(|date| string_to_naive_date(date))
                .max()
        }
    }
}

pub trait AnyDateMap {
    fn get_initial_first_unsafe_date(&self) -> Option<NaiveDate>;

    fn get_initial_last_date(&self) -> Option<NaiveDate>;

    fn get_last_date(&self) -> Option<NaiveDate>;

    fn get_first_unsafe_date(&self) -> Option<NaiveDate>;

    fn export(&self) -> color_eyre::Result<()>;

    fn path(&self) -> &str;

    fn t_name(&self) -> &str;

    fn reset(&mut self) -> color_eyre::Result<()>;
}

impl<T> AnyDateMap for DateMap<T>
where
    T: Clone + Default + Encode + Decode + Debug + Serialize + DeserializeOwned,
{
    #[inline(always)]
    fn get_last_date(&self) -> Option<NaiveDate> {
        self.get_last_date()
    }

    #[inline(always)]
    fn get_first_unsafe_date(&self) -> Option<NaiveDate> {
        self.get_first_unsafe_date()
    }

    #[inline(always)]
    fn get_initial_first_unsafe_date(&self) -> Option<NaiveDate> {
        self.initial_first_unsafe_date
    }

    #[inline(always)]
    fn get_initial_last_date(&self) -> Option<NaiveDate> {
        self.initial_last_date
    }

    fn export(&self) -> color_eyre::Result<()> {
        if !self.called_insert.lock().to_owned() {
            return Ok(());
        }

        *self.called_insert.lock() = false;

        if let Some(inner) = self.inner.as_ref() {
            self.serialization.export(&self.path, inner)
        } else {
            if self.batch.lock().is_empty() {
                return Ok(());
            }

            let mut map = self.import();

            self.batch.lock().drain(..).for_each(|(date, value)| {
                map.insert(date.to_string(), value);
            });

            self.serialization.export(&self.path, &map)
        }
    }

    fn path(&self) -> &str {
        &self.path
    }

    fn t_name(&self) -> &str {
        std::any::type_name::<T>()
    }

    fn reset(&mut self) -> color_eyre::Result<()> {
        fs::remove_dir(&self.path)?;

        self.batch.lock().clear();
        self.initial_last_date = None;
        self.initial_first_unsafe_date = None;

        if let Some(vec) = self.inner.as_ref() {
            vec.lock().clear()
        }

        *self.called_insert.lock() = false;

        Ok(())
    }
}

fn last_date_to_first_unsafe_date(last_date: Option<NaiveDate>) -> Option<NaiveDate> {
    last_date.and_then(|last_date| {
        let offset = NUMBER_OF_UNSAFE_DATES - 1;

        last_date.checked_sub_days(Days::new(offset as u64))
    })
}
