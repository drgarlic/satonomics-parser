use std::{
    cell::{Ref, RefCell, RefMut},
    collections::VecDeque,
    str::FromStr,
};

use bitcoin_explorer::Txid;
use rustc_hash::FxHashMap;
use serde::{de::DeserializeOwned, Serialize};

use crate::utils::{export_snapshot, import_snapshot_map};

pub type TxidHashMap<T> = FxHashMap<Txid, T>;

pub struct TxidMap<T> {
    snapshot_name: Option<String>,
    map: RefCell<TxidHashMap<T>>,
    ordered_txids: RefCell<VecDeque<Txid>>,
    max_size: Option<usize>,
}

impl<T> TxidMap<T>
where
    T: Clone + Serialize + DeserializeOwned,
{
    pub fn import(snapshot_name: &str) -> color_eyre::Result<Self> {
        let x = import_snapshot_map::<T>(snapshot_name, true)?
            .iter()
            .map(|(txid, value)| (Txid::from_str(txid).unwrap(), value.to_owned()))
            .collect::<TxidHashMap<T>>();

        Ok(Self {
            snapshot_name: Some(snapshot_name.to_string()),
            map: RefCell::new(x),
            ordered_txids: RefCell::new(VecDeque::new()),
            max_size: None,
        })
    }

    pub fn new(max_size: Option<usize>) -> Self {
        Self {
            snapshot_name: None,
            map: RefCell::new(FxHashMap::default()),
            ordered_txids: RefCell::new(VecDeque::new()),
            max_size,
        }
    }

    pub fn insert(&self, txid: Txid, value: T) -> Option<T> {
        let opt = self.map.borrow_mut().insert(txid, value);

        if let Some(max_size) = self.max_size {
            self.ordered_txids.borrow_mut().push_front(txid);

            if self.ordered_txids.borrow().len() > max_size {
                self.map.borrow_mut().remove(
                    &self
                        .ordered_txids
                        .borrow_mut()
                        .pop_back()
                        .expect("VecDecque to have a length > 1 (checked just before)"),
                );
            }
        }

        opt
    }

    pub fn borrow_map(&self) -> Ref<'_, TxidHashMap<T>> {
        Ref::map(self.map.borrow(), |map| map)
    }

    pub fn borrow_mut_map(&self) -> RefMut<'_, TxidHashMap<T>> {
        RefMut::map(self.map.borrow_mut(), |map| map)
    }

    // pub fn borrow_ordered_txids(&self) -> Ref<'_, VecDeque<Txid>> {
    //     Ref::map(self.ordered_txids.borrow(), |map| map)
    // }

    pub fn borrow_mut_ordered_txids(&self) -> RefMut<'_, VecDeque<Txid>> {
        RefMut::map(self.ordered_txids.borrow_mut(), |map| map)
    }

    pub fn is_finite(&self) -> bool {
        self.max_size.is_some()
    }

    pub fn remove(&self, txid: &Txid) -> Option<T> {
        self.borrow_mut_map().remove(txid)
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        if let Some(name) = &self.snapshot_name {
            export_snapshot(name, &self.borrow_map().clone(), false)?;
        } else {
            panic!("Can't export a nameless txid_map !");
        }

        Ok(())
    }
}
