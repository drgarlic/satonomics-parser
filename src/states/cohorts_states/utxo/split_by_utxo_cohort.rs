use std::collections::BTreeSet;

use super::{UTXOCheck, UTXOCohortId, UTXO_FILTERS};

#[derive(Default)]
pub struct SplitByUTXOCohort<T> {
    pub sth: T,
    pub lth: T,

    pub up_to_1d: T,
    pub up_to_1w: T,
    pub up_to_1m: T,
    pub up_to_2m: T,
    pub up_to_3m: T,
    pub up_to_4m: T,
    pub up_to_5m: T,
    pub up_to_6m: T,
    pub up_to_1y: T,
    pub up_to_2y: T,
    pub up_to_3y: T,
    pub up_to_5y: T,
    pub up_to_7y: T,
    pub up_to_10y: T,

    pub from_1d_to_1w: T,
    pub from_1w_to_1m: T,
    pub from_1m_to_3m: T,
    pub from_3m_to_6m: T,
    pub from_6m_to_1y: T,
    pub from_1y_to_2y: T,
    pub from_2y_to_3y: T,
    pub from_3y_to_5y: T,
    pub from_5y_to_7y: T,
    pub from_7y_to_10y: T,

    pub from_1y: T,
    pub from_10y: T,

    pub year_2009: T,
    pub year_2010: T,
    pub year_2011: T,
    pub year_2012: T,
    pub year_2013: T,
    pub year_2014: T,
    pub year_2015: T,
    pub year_2016: T,
    pub year_2017: T,
    pub year_2018: T,
    pub year_2019: T,
    pub year_2020: T,
    pub year_2021: T,
    pub year_2022: T,
    pub year_2023: T,
    pub year_2024: T,
}

impl<T> SplitByUTXOCohort<T> {
    pub fn get(&self, id: &UTXOCohortId) -> &T {
        match id {
            UTXOCohortId::UpTo1d => &self.up_to_1d,
            UTXOCohortId::UpTo1w => &self.up_to_1w,
            UTXOCohortId::UpTo1m => &self.up_to_1m,
            UTXOCohortId::UpTo2m => &self.up_to_2m,
            UTXOCohortId::UpTo3m => &self.up_to_3m,
            UTXOCohortId::UpTo4m => &self.up_to_4m,
            UTXOCohortId::UpTo5m => &self.up_to_5m,
            UTXOCohortId::UpTo6m => &self.up_to_6m,
            UTXOCohortId::UpTo1y => &self.up_to_1y,
            UTXOCohortId::UpTo2y => &self.up_to_2y,
            UTXOCohortId::UpTo3y => &self.up_to_3y,
            UTXOCohortId::UpTo5y => &self.up_to_5y,
            UTXOCohortId::UpTo7y => &self.up_to_7y,
            UTXOCohortId::UpTo10y => &self.up_to_10y,
            UTXOCohortId::From1dTo1w => &self.from_1d_to_1w,
            UTXOCohortId::From1wTo1m => &self.from_1w_to_1m,
            UTXOCohortId::From1mTo3m => &self.from_1m_to_3m,
            UTXOCohortId::From3mTo6m => &self.from_3m_to_6m,
            UTXOCohortId::From6mTo1y => &self.from_6m_to_1y,
            UTXOCohortId::From1yTo2y => &self.from_1y_to_2y,
            UTXOCohortId::From2yTo3y => &self.from_2y_to_3y,
            UTXOCohortId::From3yTo5y => &self.from_3y_to_5y,
            UTXOCohortId::From5yTo7y => &self.from_5y_to_7y,
            UTXOCohortId::From7yTo10y => &self.from_7y_to_10y,
            UTXOCohortId::From1y => &self.from_1y,
            UTXOCohortId::From10y => &self.from_10y,
            UTXOCohortId::Year2009 => &self.year_2009,
            UTXOCohortId::Year2010 => &self.year_2010,
            UTXOCohortId::Year2011 => &self.year_2011,
            UTXOCohortId::Year2012 => &self.year_2012,
            UTXOCohortId::Year2013 => &self.year_2013,
            UTXOCohortId::Year2014 => &self.year_2014,
            UTXOCohortId::Year2015 => &self.year_2015,
            UTXOCohortId::Year2016 => &self.year_2016,
            UTXOCohortId::Year2017 => &self.year_2017,
            UTXOCohortId::Year2018 => &self.year_2018,
            UTXOCohortId::Year2019 => &self.year_2019,
            UTXOCohortId::Year2020 => &self.year_2020,
            UTXOCohortId::Year2021 => &self.year_2021,
            UTXOCohortId::Year2022 => &self.year_2022,
            UTXOCohortId::Year2023 => &self.year_2023,
            UTXOCohortId::Year2024 => &self.year_2024,
            UTXOCohortId::ShortTermHolders => &self.sth,
            UTXOCohortId::LongTermHolders => &self.lth,
        }
    }

    pub fn get_mut(&mut self, id: &UTXOCohortId) -> &mut T {
        match id {
            UTXOCohortId::UpTo1d => &mut self.up_to_1d,
            UTXOCohortId::UpTo1w => &mut self.up_to_1w,
            UTXOCohortId::UpTo1m => &mut self.up_to_1m,
            UTXOCohortId::UpTo2m => &mut self.up_to_2m,
            UTXOCohortId::UpTo3m => &mut self.up_to_3m,
            UTXOCohortId::UpTo4m => &mut self.up_to_4m,
            UTXOCohortId::UpTo5m => &mut self.up_to_5m,
            UTXOCohortId::UpTo6m => &mut self.up_to_6m,
            UTXOCohortId::UpTo1y => &mut self.up_to_1y,
            UTXOCohortId::UpTo2y => &mut self.up_to_2y,
            UTXOCohortId::UpTo3y => &mut self.up_to_3y,
            UTXOCohortId::UpTo5y => &mut self.up_to_5y,
            UTXOCohortId::UpTo7y => &mut self.up_to_7y,
            UTXOCohortId::UpTo10y => &mut self.up_to_10y,
            UTXOCohortId::From1dTo1w => &mut self.from_1d_to_1w,
            UTXOCohortId::From1wTo1m => &mut self.from_1w_to_1m,
            UTXOCohortId::From1mTo3m => &mut self.from_1m_to_3m,
            UTXOCohortId::From3mTo6m => &mut self.from_3m_to_6m,
            UTXOCohortId::From6mTo1y => &mut self.from_6m_to_1y,
            UTXOCohortId::From1yTo2y => &mut self.from_1y_to_2y,
            UTXOCohortId::From2yTo3y => &mut self.from_2y_to_3y,
            UTXOCohortId::From3yTo5y => &mut self.from_3y_to_5y,
            UTXOCohortId::From5yTo7y => &mut self.from_5y_to_7y,
            UTXOCohortId::From7yTo10y => &mut self.from_7y_to_10y,
            UTXOCohortId::From1y => &mut self.from_1y,
            UTXOCohortId::From10y => &mut self.from_10y,
            UTXOCohortId::Year2009 => &mut self.year_2009,
            UTXOCohortId::Year2010 => &mut self.year_2010,
            UTXOCohortId::Year2011 => &mut self.year_2011,
            UTXOCohortId::Year2012 => &mut self.year_2012,
            UTXOCohortId::Year2013 => &mut self.year_2013,
            UTXOCohortId::Year2014 => &mut self.year_2014,
            UTXOCohortId::Year2015 => &mut self.year_2015,
            UTXOCohortId::Year2016 => &mut self.year_2016,
            UTXOCohortId::Year2017 => &mut self.year_2017,
            UTXOCohortId::Year2018 => &mut self.year_2018,
            UTXOCohortId::Year2019 => &mut self.year_2019,
            UTXOCohortId::Year2020 => &mut self.year_2020,
            UTXOCohortId::Year2021 => &mut self.year_2021,
            UTXOCohortId::Year2022 => &mut self.year_2022,
            UTXOCohortId::Year2023 => &mut self.year_2023,
            UTXOCohortId::Year2024 => &mut self.year_2024,
            UTXOCohortId::ShortTermHolders => &mut self.sth,
            UTXOCohortId::LongTermHolders => &mut self.lth,
        }
    }

    pub fn filtered_ids(&mut self, days_old: &u32, year: &u32) -> BTreeSet<UTXOCohortId> {
        let mut set = BTreeSet::new();

        if UTXO_FILTERS.up_to_1d.check(days_old, year) {
            set.insert(UTXOCohortId::UpTo1d);
        }

        if UTXO_FILTERS.up_to_1w.check(days_old, year) {
            set.insert(UTXOCohortId::UpTo1w);
        }

        if UTXO_FILTERS.up_to_1m.check(days_old, year) {
            set.insert(UTXOCohortId::UpTo1m);
        }

        if UTXO_FILTERS.up_to_2m.check(days_old, year) {
            set.insert(UTXOCohortId::UpTo2m);
        }

        if UTXO_FILTERS.up_to_3m.check(days_old, year) {
            set.insert(UTXOCohortId::UpTo3m);
        }

        if UTXO_FILTERS.up_to_4m.check(days_old, year) {
            set.insert(UTXOCohortId::UpTo4m);
        }

        if UTXO_FILTERS.up_to_5m.check(days_old, year) {
            set.insert(UTXOCohortId::UpTo5m);
        }

        if UTXO_FILTERS.up_to_6m.check(days_old, year) {
            set.insert(UTXOCohortId::UpTo6m);
        }

        if UTXO_FILTERS.up_to_1y.check(days_old, year) {
            set.insert(UTXOCohortId::UpTo1y);
        }

        if UTXO_FILTERS.up_to_2y.check(days_old, year) {
            set.insert(UTXOCohortId::UpTo2y);
        }

        if UTXO_FILTERS.up_to_3y.check(days_old, year) {
            set.insert(UTXOCohortId::UpTo3y);
        }

        if UTXO_FILTERS.up_to_5y.check(days_old, year) {
            set.insert(UTXOCohortId::UpTo5y);
        }

        if UTXO_FILTERS.up_to_7y.check(days_old, year) {
            set.insert(UTXOCohortId::UpTo7y);
        }

        if UTXO_FILTERS.up_to_10y.check(days_old, year) {
            set.insert(UTXOCohortId::UpTo10y);
        }

        if UTXO_FILTERS.from_1d_to_1w.check(days_old, year) {
            set.insert(UTXOCohortId::From1dTo1w);
        } else if UTXO_FILTERS.from_1w_to_1m.check(days_old, year) {
            set.insert(UTXOCohortId::From1wTo1m);
        } else if UTXO_FILTERS.from_1m_to_3m.check(days_old, year) {
            set.insert(UTXOCohortId::From1mTo3m);
        } else if UTXO_FILTERS.from_3m_to_6m.check(days_old, year) {
            set.insert(UTXOCohortId::From3mTo6m);
        } else if UTXO_FILTERS.from_6m_to_1y.check(days_old, year) {
            set.insert(UTXOCohortId::From6mTo1y);
        } else if UTXO_FILTERS.from_1y_to_2y.check(days_old, year) {
            set.insert(UTXOCohortId::From1yTo2y);
        } else if UTXO_FILTERS.from_2y_to_3y.check(days_old, year) {
            set.insert(UTXOCohortId::From2yTo3y);
        } else if UTXO_FILTERS.from_3y_to_5y.check(days_old, year) {
            set.insert(UTXOCohortId::From3yTo5y);
        } else if UTXO_FILTERS.from_5y_to_7y.check(days_old, year) {
            set.insert(UTXOCohortId::From5yTo7y);
        } else if UTXO_FILTERS.from_7y_to_10y.check(days_old, year) {
            set.insert(UTXOCohortId::From7yTo10y);
        }

        if UTXO_FILTERS.from_1y.check(days_old, year) {
            set.insert(UTXOCohortId::From1y);
        }

        if UTXO_FILTERS.from_10y.check(days_old, year) {
            set.insert(UTXOCohortId::From10y);
        }

        if UTXO_FILTERS.year_2009.check(days_old, year) {
            set.insert(UTXOCohortId::Year2009);
        } else if UTXO_FILTERS.year_2010.check(days_old, year) {
            set.insert(UTXOCohortId::Year2010);
        } else if UTXO_FILTERS.year_2011.check(days_old, year) {
            set.insert(UTXOCohortId::Year2011);
        } else if UTXO_FILTERS.year_2012.check(days_old, year) {
            set.insert(UTXOCohortId::Year2012);
        } else if UTXO_FILTERS.year_2013.check(days_old, year) {
            set.insert(UTXOCohortId::Year2013);
        } else if UTXO_FILTERS.year_2014.check(days_old, year) {
            set.insert(UTXOCohortId::Year2014);
        } else if UTXO_FILTERS.year_2015.check(days_old, year) {
            set.insert(UTXOCohortId::Year2015);
        } else if UTXO_FILTERS.year_2016.check(days_old, year) {
            set.insert(UTXOCohortId::Year2016);
        } else if UTXO_FILTERS.year_2017.check(days_old, year) {
            set.insert(UTXOCohortId::Year2017);
        } else if UTXO_FILTERS.year_2018.check(days_old, year) {
            set.insert(UTXOCohortId::Year2018);
        } else if UTXO_FILTERS.year_2019.check(days_old, year) {
            set.insert(UTXOCohortId::Year2019);
        } else if UTXO_FILTERS.year_2020.check(days_old, year) {
            set.insert(UTXOCohortId::Year2020);
        } else if UTXO_FILTERS.year_2021.check(days_old, year) {
            set.insert(UTXOCohortId::Year2021);
        } else if UTXO_FILTERS.year_2022.check(days_old, year) {
            set.insert(UTXOCohortId::Year2022);
        } else if UTXO_FILTERS.year_2023.check(days_old, year) {
            set.insert(UTXOCohortId::Year2023);
        } else if UTXO_FILTERS.year_2024.check(days_old, year) {
            set.insert(UTXOCohortId::Year2024);
        }

        if UTXO_FILTERS.sth.check(days_old, year) {
            set.insert(UTXOCohortId::ShortTermHolders);
        } else {
            // } else if UTXO_FILTERS.lth.check(days_old, year) {
            set.insert(UTXOCohortId::LongTermHolders);
        }

        set
    }

    pub fn filtered_apply(&mut self, days_old: &u32, year: &u32, apply: impl Fn(&mut T)) {
        if UTXO_FILTERS.up_to_1d.check(days_old, year) {
            apply(&mut self.up_to_1d);
        } else if UTXO_FILTERS.from_1d_to_1w.check(days_old, year) {
            apply(&mut self.from_1d_to_1w);
        } else if UTXO_FILTERS.from_1w_to_1m.check(days_old, year) {
            apply(&mut self.from_1w_to_1m);
        } else if UTXO_FILTERS.from_1m_to_3m.check(days_old, year) {
            apply(&mut self.from_1m_to_3m);
        } else if UTXO_FILTERS.from_3m_to_6m.check(days_old, year) {
            apply(&mut self.from_3m_to_6m);
        } else if UTXO_FILTERS.from_6m_to_1y.check(days_old, year) {
            apply(&mut self.from_6m_to_1y);
        } else if UTXO_FILTERS.from_1y_to_2y.check(days_old, year) {
            apply(&mut self.from_1y_to_2y);
        } else if UTXO_FILTERS.from_2y_to_3y.check(days_old, year) {
            apply(&mut self.from_2y_to_3y);
        } else if UTXO_FILTERS.from_3y_to_5y.check(days_old, year) {
            apply(&mut self.from_3y_to_5y);
        } else if UTXO_FILTERS.from_5y_to_7y.check(days_old, year) {
            apply(&mut self.from_5y_to_7y);
        } else if UTXO_FILTERS.from_7y_to_10y.check(days_old, year) {
            apply(&mut self.from_7y_to_10y);
        }

        if UTXO_FILTERS.year_2009.check(days_old, year) {
            apply(&mut self.year_2009);
        } else if UTXO_FILTERS.year_2010.check(days_old, year) {
            apply(&mut self.year_2010);
        } else if UTXO_FILTERS.year_2011.check(days_old, year) {
            apply(&mut self.year_2011);
        } else if UTXO_FILTERS.year_2012.check(days_old, year) {
            apply(&mut self.year_2012);
        } else if UTXO_FILTERS.year_2013.check(days_old, year) {
            apply(&mut self.year_2013);
        } else if UTXO_FILTERS.year_2014.check(days_old, year) {
            apply(&mut self.year_2014);
        } else if UTXO_FILTERS.year_2015.check(days_old, year) {
            apply(&mut self.year_2015);
        } else if UTXO_FILTERS.year_2016.check(days_old, year) {
            apply(&mut self.year_2016);
        } else if UTXO_FILTERS.year_2017.check(days_old, year) {
            apply(&mut self.year_2017);
        } else if UTXO_FILTERS.year_2018.check(days_old, year) {
            apply(&mut self.year_2018);
        } else if UTXO_FILTERS.year_2019.check(days_old, year) {
            apply(&mut self.year_2019);
        } else if UTXO_FILTERS.year_2020.check(days_old, year) {
            apply(&mut self.year_2020);
        } else if UTXO_FILTERS.year_2021.check(days_old, year) {
            apply(&mut self.year_2021);
        } else if UTXO_FILTERS.year_2022.check(days_old, year) {
            apply(&mut self.year_2022);
        } else if UTXO_FILTERS.year_2023.check(days_old, year) {
            apply(&mut self.year_2023);
        } else if UTXO_FILTERS.year_2024.check(days_old, year) {
            apply(&mut self.year_2024);
        }

        if UTXO_FILTERS.sth.check(days_old, year) {
            apply(&mut self.sth);
        } else {
            // } else if UTXO_FILTERS.lth.check(days_old, year) {
            apply(&mut self.lth);
        }

        if UTXO_FILTERS.from_1y.check(days_old, year) {
            apply(&mut self.from_1y);
        }

        if UTXO_FILTERS.from_10y.check(days_old, year) {
            apply(&mut self.from_10y);
        }

        if UTXO_FILTERS.up_to_10y.check(days_old, year) {
            apply(&mut self.up_to_10y);
        } else {
            return;
        }

        if UTXO_FILTERS.up_to_7y.check(days_old, year) {
            apply(&mut self.up_to_7y);
        } else {
            return;
        }

        if UTXO_FILTERS.up_to_5y.check(days_old, year) {
            apply(&mut self.up_to_5y);
        } else {
            return;
        }

        if UTXO_FILTERS.up_to_3y.check(days_old, year) {
            apply(&mut self.up_to_3y);
        } else {
            return;
        }

        if UTXO_FILTERS.up_to_2y.check(days_old, year) {
            apply(&mut self.up_to_2y);
        } else {
            return;
        }

        if UTXO_FILTERS.up_to_1y.check(days_old, year) {
            apply(&mut self.up_to_1y);
        } else {
            return;
        }

        if UTXO_FILTERS.up_to_6m.check(days_old, year) {
            apply(&mut self.up_to_6m);
        } else {
            return;
        }

        if UTXO_FILTERS.up_to_5m.check(days_old, year) {
            apply(&mut self.up_to_5m);
        } else {
            return;
        }

        if UTXO_FILTERS.up_to_4m.check(days_old, year) {
            apply(&mut self.up_to_4m);
        } else {
            return;
        }

        if UTXO_FILTERS.up_to_3m.check(days_old, year) {
            apply(&mut self.up_to_3m);
        } else {
            return;
        }

        if UTXO_FILTERS.up_to_2m.check(days_old, year) {
            apply(&mut self.up_to_2m);
        } else {
            return;
        }

        if UTXO_FILTERS.up_to_1m.check(days_old, year) {
            apply(&mut self.up_to_1m);
        } else {
            return;
        }

        if UTXO_FILTERS.up_to_1w.check(days_old, year) {
            apply(&mut self.up_to_1w);
        }
    }

    pub fn as_vec(&self) -> Vec<&T> {
        vec![
            &self.up_to_1d,
            &self.up_to_1w,
            &self.up_to_1m,
            &self.up_to_2m,
            &self.up_to_3m,
            &self.up_to_4m,
            &self.up_to_5m,
            &self.up_to_6m,
            &self.up_to_1y,
            &self.up_to_2y,
            &self.up_to_3y,
            &self.up_to_5y,
            &self.up_to_7y,
            &self.up_to_10y,
            &self.from_1d_to_1w,
            &self.from_1w_to_1m,
            &self.from_1m_to_3m,
            &self.from_3m_to_6m,
            &self.from_6m_to_1y,
            &self.from_1y_to_2y,
            &self.from_2y_to_3y,
            &self.from_3y_to_5y,
            &self.from_5y_to_7y,
            &self.from_7y_to_10y,
            &self.from_1y,
            &self.from_10y,
            &self.year_2009,
            &self.year_2010,
            &self.year_2011,
            &self.year_2012,
            &self.year_2013,
            &self.year_2014,
            &self.year_2015,
            &self.year_2016,
            &self.year_2017,
            &self.year_2018,
            &self.year_2019,
            &self.year_2020,
            &self.year_2021,
            &self.year_2022,
            &self.year_2023,
            &self.year_2024,
            &self.sth,
            &self.lth,
        ]
    }

    pub fn as_mut_vec(&mut self) -> Vec<&mut T> {
        vec![
            &mut self.up_to_1d,
            &mut self.up_to_1w,
            &mut self.up_to_1m,
            &mut self.up_to_2m,
            &mut self.up_to_3m,
            &mut self.up_to_4m,
            &mut self.up_to_5m,
            &mut self.up_to_6m,
            &mut self.up_to_1y,
            &mut self.up_to_2y,
            &mut self.up_to_3y,
            &mut self.up_to_5y,
            &mut self.up_to_7y,
            &mut self.up_to_10y,
            &mut self.from_1d_to_1w,
            &mut self.from_1w_to_1m,
            &mut self.from_1m_to_3m,
            &mut self.from_3m_to_6m,
            &mut self.from_6m_to_1y,
            &mut self.from_1y_to_2y,
            &mut self.from_2y_to_3y,
            &mut self.from_3y_to_5y,
            &mut self.from_5y_to_7y,
            &mut self.from_7y_to_10y,
            &mut self.from_1y,
            &mut self.from_10y,
            &mut self.year_2009,
            &mut self.year_2010,
            &mut self.year_2011,
            &mut self.year_2012,
            &mut self.year_2013,
            &mut self.year_2014,
            &mut self.year_2015,
            &mut self.year_2016,
            &mut self.year_2017,
            &mut self.year_2018,
            &mut self.year_2019,
            &mut self.year_2020,
            &mut self.year_2021,
            &mut self.year_2022,
            &mut self.year_2023,
            &mut self.year_2024,
            &mut self.sth,
            &mut self.lth,
        ]
    }
}
