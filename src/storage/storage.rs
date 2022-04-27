use core::hash::Hash;
use std::collections::{hash_map::Entry, HashMap};
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::storage::Change;

#[derive(Clone, Debug)]
pub struct RawStorage<D, U> {
    data: D,
    subscribers: Vec<Sender<U>>,
}

pub type Storage<D> = RawStorage<D, <D as StorageData>::Change>;

impl<D, U> Default for RawStorage<D, U>
where
    D: Default,
{
    fn default() -> Self {
        Self {
            data: D::default(),
            subscribers: Vec::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Table<K, V> {
    data: HashMap<K, V>,
    subscribers: Vec<Sender<(K, Change<V>)>>,
}

pub struct ValueRef<'a, U, K, V> {
    storage_subscribers: &'a mut Vec<Sender<U>>,
    table_subscribers: &'a mut Vec<Sender<(K, Change<V>)>>,
    entry: Entry<'a, K, V>,
}

impl<K, V> Default for Table<K, V> {
    fn default() -> Self {
        Self {
            data: HashMap::default(),
            subscribers: Vec::default(),
        }
    }
}

impl<K, V> Table<K, V> {
    pub fn data(&self) -> &HashMap<K, V> {
        &self.data
    }

    pub fn subscribe(&mut self) -> Receiver<(K, Change<V>)> {
        let (sender, receiver) = channel();
        self.subscribe_to(sender);
        receiver
    }

    pub fn subscribe_to(&mut self, sender: Sender<(K, Change<V>)>) {
        self.subscribers.push(sender);
    }

    pub fn send_as_changes_to<U>(&self, sender: &Sender<U>)
    where
        K: Clone,
        V: Clone,
        U: From<(K, Change<V>)>,
    {
        for (key, value) in &self.data {
            sender
                .send((key.clone(), Change::Added(value.clone())).into())
                .unwrap();
        }
    }

    pub fn update_and_subscribe(&mut self) -> Receiver<(K, Change<V>)>
    where
        K: Clone,
        V: Clone,
    {
        let (sender, receiver) = channel();
        self.send_as_changes_to(&sender);
        self.subscribe_to(sender);
        receiver
    }
}

pub trait StorageData {
    type Change;
    fn send_as_changes_to(&self, sender: &Sender<Self::Change>);
}

pub trait Key: Sized {
    type Data: StorageData;
    type Value;
    fn table_ref(data: &Self::Data) -> &Table<Self, Self::Value>;
    fn table_mut(data: &mut Self::Data) -> &mut Table<Self, Self::Value>;
}

impl<D, U> RawStorage<D, U> {
    pub fn get<K, V>(&self, key: &K) -> Option<&V>
    where
        K: 'static + Eq + Hash + Key<Data = D, Value = V>,
    {
        K::table_ref(&self.data).data.get(key)
    }

    pub fn table_ref<K, V>(&self) -> &Table<K, V>
    where
        K: 'static + Eq + Hash + Key<Data = D, Value = V>,
    {
        K::table_ref(&self.data)
    }

    pub fn table_mut<K, V>(&mut self) -> &mut Table<K, V>
    where
        K: 'static + Eq + Hash + Key<Data = D, Value = V>,
    {
        K::table_mut(&mut self.data)
    }

    pub fn subscribe(&mut self) -> Receiver<U> {
        let (sender, receiver) = channel();
        self.subscribe_to(sender);
        receiver
    }

    pub fn subscribe_to(&mut self, sender: Sender<U>) {
        self.subscribers.push(sender);
    }

    pub fn send_as_changes_to(&mut self, sender: &Sender<U>)
    where
        D: StorageData<Change = U>,
    {
        self.data.send_as_changes_to(sender);
    }

    pub fn put<K, V>(&mut self, key: K, value: V)
    where
        D: StorageData<Change = U>,
        U: Clone + From<(K, Change<V>)>,
        K: Clone + Eq + Hash + Key<Data = D, Value = V>,
        V: Clone + PartialEq,
    {
        let table = K::table_mut(&mut self.data);
        let old_value = table.data.insert(key.clone(), value.clone());
        if let Some(old_value) = old_value {
            if value != old_value {
                let change = Change::Modified(&old_value, &value);
                send_change(&mut table.subscribers, &(key.clone(), change.cloned()));
                send_change(
                    &mut self.subscribers,
                    &(key.clone(), change.cloned()).into(),
                );
            }
        } else {
            let change = Change::Added(&value);
            send_change(&mut table.subscribers, &(key.clone(), change.cloned()));
            send_change(
                &mut self.subscribers,
                &(key.clone(), change.cloned()).into(),
            );
        }
    }

    pub fn remove<K, V>(&mut self, key: &K) -> Option<V>
    where
        D: StorageData<Change = U>,
        U: Clone + From<(K, Change<V>)>,
        K: Clone + Eq + Hash + Key<Data = D, Value = V>,
        V: Clone,
    {
        let table = K::table_mut(&mut self.data);
        let value = table.data.remove(key);
        if let Some(value) = value.as_ref() {
            let change = Change::Removed(&value).cloned();
            send_change(&mut table.subscribers, &(key.clone(), change.cloned()));
            send_change(
                &mut self.subscribers,
                &(key.clone(), change.cloned()).into(),
            );
        }
        value
    }

    /*pub fn get_mut<'a, K, V>(&'a mut self, key: K) -> ValueRef<'a, U, K, V>
    where
        D: StorageData<Change = U>,
        U: Clone + From<(K, Change<V>)>,
        K: Clone + Eq + Hash + Key<Data = D, Value = V>,
    {
        let table = K::table_mut(&mut self.data);
        ValueRef {
            storage_subscribers: &mut self.subscribers,
            table_subscribers: &mut table.subscribers,
            entry: table.data.entry(key),
        }
    }*/
}

/*
impl<'a, U, K, V> ValueRef<'a, U, K, V> {
    pub fn put(self, value: V)
    where
        U: Clone + From<(K, Change<V>)>,
        K: Clone + Eq + Hash + Key<Value = V>,
        V: Clone + PartialEq,
    {
        let old_value = self.entry.insert(value.clone());
        if let Some(old_value) = old_value {
            if value != old_value {
                let change = Change::Modified(&old_value, &value);
                send_change(&mut table.subscribers, &(key.clone(), change.cloned()));
                send_change(
                    &mut self.subscribers,
                    &(key.clone(), change.cloned()).into(),
                );
            }
        } else {
            let change = Change::Removed(&value);
            send_change(&mut table.subscribers, &(key.clone(), change.cloned()));
            send_change(
                &mut self.subscribers,
                &(key.clone(), change.cloned()).into(),
            );
        }
    }
}*/

fn send_change<U>(subscribers: &mut Vec<Sender<U>>, change: &U)
where
    U: Clone,
{
    subscribers.retain(|sender| sender.send(change.clone()).is_ok());
}
