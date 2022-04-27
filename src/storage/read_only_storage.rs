use core::borrow::{Borrow, BorrowMut};
use core::hash::Hash;
use std::sync::mpsc::{Receiver, Sender};

use crate::storage::{Key, RawStorage, StorageData, Table};

#[derive(Clone, Debug)]
pub struct ReadOnlyStorage<T>(T);

impl<T> ReadOnlyStorage<T> {
    pub fn new(storage: T) -> Self {
        Self(storage)
    }

    pub fn get<'a, D, U, K, V>(&'a self, key: &K) -> Option<&'a V>
    where
        D: 'a,
        U: 'a,
        T: Borrow<RawStorage<D, U>>,
        K: 'static + Eq + Hash + Key<Data = D, Value = V>,
    {
        self.0.borrow().get(key)
    }

    pub fn table_ref<'a, D, U, K, V>(&'a self) -> &'a Table<K, V>
    where
        D: 'a,
        U: 'a,
        T: Borrow<RawStorage<D, U>>,
        K: 'static + Eq + Hash + Key<Data = D, Value = V>,
    {
        self.0.borrow().table_ref()
    }

    pub fn table_mut<'a, D, U, K, V>(&'a mut self) -> &'a Table<K, V>
    where
        D: 'a,
        U: 'a,
        T: BorrowMut<RawStorage<D, U>>,
        K: 'static + Eq + Hash + Key<Data = D, Value = V>,
    {
        self.0.borrow_mut().table_mut()
    }

    pub fn subscribe<D, U>(&mut self) -> Receiver<U>
    where
        T: BorrowMut<RawStorage<D, U>>,
    {
        self.0.borrow_mut().subscribe()
    }

    pub fn subscribe_to<D, U>(&mut self, sender: Sender<U>)
    where
        T: BorrowMut<RawStorage<D, U>>,
    {
        self.0.borrow_mut().subscribe_to(sender)
    }

    pub fn send_as_changes<D, U>(&mut self, sender: &Sender<U>)
    where
        T: BorrowMut<RawStorage<D, U>>,
        D: StorageData<Change = U>,
    {
        self.0.borrow_mut().send_as_changes_to(sender)
    }
}
