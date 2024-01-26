use std::{fs, thread};

use chrono::Datelike;
use itertools::Itertools;
use rayon::prelude::*;

use crate::datasets::{AnyHeightDataset, AnyHeightDatasets};

use super::{UTXODataset, UTXOFilter};

pub struct UTXODatasets {
    all: UTXODataset,

    up_to_1d: UTXODataset,
    up_to_7d: UTXODataset,
    up_to_1m: UTXODataset,
    up_to_2m: UTXODataset,
    up_to_3m: UTXODataset,
    up_to_4m: UTXODataset,
    up_to_5m: UTXODataset,
    up_to_6m: UTXODataset,
    up_to_1y: UTXODataset,
    up_to_2y: UTXODataset,
    up_to_3y: UTXODataset,
    up_to_5y: UTXODataset,
    up_to_7y: UTXODataset,
    up_to_10y: UTXODataset,

    from_1d_to_7d: UTXODataset,
    from_7d_to_1m: UTXODataset,
    from_1m_to_3m: UTXODataset,
    from_3m_to_6m: UTXODataset,
    from_6m_to_1y: UTXODataset,
    from_1y_to_2y: UTXODataset,
    from_2y_to_3y: UTXODataset,
    from_3y_to_5y: UTXODataset,
    from_5y_to_7y: UTXODataset,
    from_7y_to_10y: UTXODataset,
    from_10y_to_end: UTXODataset,

    sth: UTXODataset,
    lth: UTXODataset,

    yearly: Vec<UTXODataset>,
}

impl UTXODatasets {
    pub fn import(path: &str) -> color_eyre::Result<Self> {
        let path_string = format!("{path}/utxo");
        let path = path_string.as_str();

        fs::create_dir_all(path)?;

        thread::scope(|scope| {
            let all_handle = scope.spawn(|| UTXODataset::import(path, "all", UTXOFilter::Full));

            let up_to_1d_handle =
                scope.spawn(|| UTXODataset::import(path, "up_to_1d", UTXOFilter::To(1)));
            let up_to_7d_handle =
                scope.spawn(|| UTXODataset::import(path, "up_to_7d", UTXOFilter::To(7)));
            let up_to_1m_handle =
                scope.spawn(|| UTXODataset::import(path, "up_to_1m", UTXOFilter::To(30)));
            let up_to_2m_handle =
                scope.spawn(|| UTXODataset::import(path, "up_to_2m", UTXOFilter::To(2 * 30)));
            let up_to_3m_handle =
                scope.spawn(|| UTXODataset::import(path, "up_to_3m", UTXOFilter::To(3 * 30)));
            let up_to_4m_handle =
                scope.spawn(|| UTXODataset::import(path, "up_to_4m", UTXOFilter::To(4 * 30)));
            let up_to_5m_handle =
                scope.spawn(|| UTXODataset::import(path, "up_to_5m", UTXOFilter::To(5 * 30)));
            let up_to_6m_handle =
                scope.spawn(|| UTXODataset::import(path, "up_to_6m", UTXOFilter::To(6 * 30)));
            let up_to_1y_handle =
                scope.spawn(|| UTXODataset::import(path, "up_to_1y", UTXOFilter::To(365)));
            let up_to_2y_handle =
                scope.spawn(|| UTXODataset::import(path, "up_to_2y", UTXOFilter::To(2 * 365)));
            let up_to_3y_handle =
                scope.spawn(|| UTXODataset::import(path, "up_to_3y", UTXOFilter::To(3 * 365)));
            let up_to_5y_handle =
                scope.spawn(|| UTXODataset::import(path, "up_to_5y", UTXOFilter::To(5 * 365)));
            let up_to_7y_handle =
                scope.spawn(|| UTXODataset::import(path, "up_to_7y", UTXOFilter::To(7 * 365)));
            let up_to_10y_handle =
                scope.spawn(|| UTXODataset::import(path, "up_to_10y", UTXOFilter::To(10 * 365)));

            let from_1d_to_7d_handle = scope.spawn(|| {
                UTXODataset::import(path, "from_1d_to_7d", UTXOFilter::new_from_to(1, 7))
            });
            let from_7d_to_1m_handle = scope.spawn(|| {
                UTXODataset::import(path, "from_7d_to_1m", UTXOFilter::new_from_to(7, 30))
            });
            let from_1m_to_3m_handle = scope.spawn(|| {
                UTXODataset::import(path, "from_1m_to_3m", UTXOFilter::new_from_to(30, 3 * 30))
            });
            let from_3m_to_6m_handle = scope.spawn(|| {
                UTXODataset::import(
                    path,
                    "from_3m_to_6m",
                    UTXOFilter::new_from_to(3 * 30, 6 * 30),
                )
            });
            let from_6m_to_1y_handle = scope.spawn(|| {
                UTXODataset::import(path, "from_6m_to_1y", UTXOFilter::new_from_to(6 * 30, 365))
            });
            let from_1y_to_2y_handle = scope.spawn(|| {
                UTXODataset::import(path, "from_1y_to_2y", UTXOFilter::new_from_to(365, 2 * 365))
            });
            let from_2y_to_3y_handle = scope.spawn(|| {
                UTXODataset::import(
                    path,
                    "from_2y_to_3y",
                    UTXOFilter::new_from_to(2 * 365, 3 * 365),
                )
            });
            let from_3y_to_5y_handle = scope.spawn(|| {
                UTXODataset::import(
                    path,
                    "from_3y_to_5y",
                    UTXOFilter::new_from_to(3 * 365, 5 * 365),
                )
            });
            let from_5y_to_7y_handle = scope.spawn(|| {
                UTXODataset::import(
                    path,
                    "from_5y_to_7y",
                    UTXOFilter::new_from_to(5 * 365, 7 * 365),
                )
            });
            let from_7y_to_10y_handle = scope.spawn(|| {
                UTXODataset::import(
                    path,
                    "from_7y_to_10y",
                    UTXOFilter::new_from_to(7 * 365, 10 * 365),
                )
            });
            let from_10y_to_end_handle = scope
                .spawn(|| UTXODataset::import(path, "from_10y_to_end", UTXOFilter::From(10 * 365)));

            let sth_handle = scope.spawn(|| UTXODataset::import(path, "sth", UTXOFilter::To(155)));
            let lth_handle =
                scope.spawn(|| UTXODataset::import(path, "lth", UTXOFilter::From(155)));

            let yearly_handles = (2009..=(chrono::Utc::now().year() as usize))
                .map(|year| {
                    scope.spawn(move || {
                        UTXODataset::import(path, &year.to_string(), UTXOFilter::Year(year))
                    })
                })
                .collect_vec();

            Ok(Self {
                all: all_handle.join().unwrap()?,

                up_to_1d: up_to_1d_handle.join().unwrap()?,
                up_to_7d: up_to_7d_handle.join().unwrap()?,
                up_to_1m: up_to_1m_handle.join().unwrap()?,
                up_to_2m: up_to_2m_handle.join().unwrap()?,
                up_to_3m: up_to_3m_handle.join().unwrap()?,
                up_to_4m: up_to_4m_handle.join().unwrap()?,
                up_to_5m: up_to_5m_handle.join().unwrap()?,
                up_to_6m: up_to_6m_handle.join().unwrap()?,
                up_to_1y: up_to_1y_handle.join().unwrap()?,
                up_to_2y: up_to_2y_handle.join().unwrap()?,
                up_to_3y: up_to_3y_handle.join().unwrap()?,
                up_to_5y: up_to_5y_handle.join().unwrap()?,
                up_to_7y: up_to_7y_handle.join().unwrap()?,
                up_to_10y: up_to_10y_handle.join().unwrap()?,

                from_1d_to_7d: from_1d_to_7d_handle.join().unwrap()?,
                from_7d_to_1m: from_7d_to_1m_handle.join().unwrap()?,
                from_1m_to_3m: from_1m_to_3m_handle.join().unwrap()?,
                from_3m_to_6m: from_3m_to_6m_handle.join().unwrap()?,
                from_6m_to_1y: from_6m_to_1y_handle.join().unwrap()?,
                from_1y_to_2y: from_1y_to_2y_handle.join().unwrap()?,
                from_2y_to_3y: from_2y_to_3y_handle.join().unwrap()?,
                from_3y_to_5y: from_3y_to_5y_handle.join().unwrap()?,
                from_5y_to_7y: from_5y_to_7y_handle.join().unwrap()?,
                from_7y_to_10y: from_7y_to_10y_handle.join().unwrap()?,
                from_10y_to_end: from_10y_to_end_handle.join().unwrap()?,

                sth: sth_handle.join().unwrap()?,
                lth: lth_handle.join().unwrap()?,

                yearly: yearly_handles
                    .into_par_iter()
                    .map(|handle| handle.join().unwrap().unwrap())
                    .collect::<Vec<_>>(),
            })
        })
    }
}

impl AnyHeightDatasets for UTXODatasets {
    fn to_vec(&self) -> Vec<&(dyn AnyHeightDataset + Send + Sync)> {
        let flats: Vec<&(dyn AnyHeightDataset + Send + Sync)> = vec![
            &self.all,
            &self.up_to_1d,
            &self.up_to_7d,
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
            &self.from_1d_to_7d,
            &self.from_7d_to_1m,
            &self.from_1m_to_3m,
            &self.from_3m_to_6m,
            &self.from_6m_to_1y,
            &self.from_1y_to_2y,
            &self.from_2y_to_3y,
            &self.from_3y_to_5y,
            &self.from_5y_to_7y,
            &self.from_7y_to_10y,
            &self.from_10y_to_end,
            &self.sth,
            &self.lth,
        ];

        let yearly = self
            .yearly
            .iter()
            .map(|dataset| dataset as &(dyn AnyHeightDataset + Send + Sync))
            .collect_vec();

        [flats, yearly].iter().flatten().copied().collect()
    }
}
