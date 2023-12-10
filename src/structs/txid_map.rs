use std::{
    cell::{Ref, RefCell, RefMut},
    collections::{BTreeMap, VecDeque},
};

use bitcoin_explorer::Txid;

pub type TxidHashMap<T> = BTreeMap<Txid, T>;

pub struct TxidMap<T> {
    map: RefCell<TxidHashMap<T>>,
    ordered_txids: RefCell<VecDeque<Txid>>,
    max_size: Option<usize>,
}

impl<T> TxidMap<T> {
    pub fn new(max_size: Option<usize>) -> Self {
        Self {
            map: RefCell::new(BTreeMap::default()),
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

    // pub fn remove(&self, txid: &Txid) -> Option<T> {
    //     self.borrow_mut_map().remove(txid)
    // }
}
