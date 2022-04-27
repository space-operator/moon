use std::collections::HashMap;
use std::hash::Hash;
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Clone, Debug)]
pub struct RawStorage<T, U> {
    data: T,
    subscribers: Vec<Sender<U>>,
}

pub type Storage<T> = RawStorage<T, <T as StorageData>::Change>;

impl<T, U> Default for RawStorage<T, U>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            data: T::default(),
            subscribers: Vec::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Table<K, V> {
    data: HashMap<K, V>,
    subscribers: Vec<Sender<(K, Change<V>)>>,
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
    // FIXME: pub?
    pub(crate) fn send_as_changes<U>(&self, sender: &Sender<U>)
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
}

#[derive(Clone, Debug)]
pub enum Change<T> {
    Added(T),
    Modified(T, T),
    Removed(T),
}

impl<T: Clone> Change<&T> {
    pub fn cloned(&self) -> Change<T> {
        match self {
            Self::Added(value) => Change::Added((*value).clone()),
            Self::Modified(old, new) => Change::Modified((*old).clone(), (*new).clone()),
            Self::Removed(value) => Change::Removed((*value).clone()),
        }
    }
}

pub trait StorageData {
    type Change;
    fn send_as_changes(&self, sender: &Sender<Self::Change>);
}

pub trait Key: Sized {
    type Data: StorageData;
    type Value;
    fn table_ref(data: &Self::Data) -> &Table<Self, Self::Value>;
    fn table_mut(data: &mut Self::Data) -> &mut Table<Self, Self::Value>;
    fn make_change(key: Self, change: Change<Self::Value>) -> <Self::Data as StorageData>::Change;
}

impl<T, U> RawStorage<T, U> {
    pub fn get<K, V>(&self, key: &K) -> Option<&V>
    where
        K: 'static + Eq + Hash + Key<Data = T, Value = V>,
    {
        K::table_ref(&self.data).data.get(key)
    }

    pub fn subscribe_all(&mut self) -> Receiver<U>
    where
        T: StorageData<Change = U>,
    {
        let (sender, receiver) = channel();
        self.subscribers.push(sender.clone());
        receiver
    }

    pub fn subscribe_all_with_sender(&mut self) -> (Sender<U>, Receiver<U>) {
        let (sender, receiver) = channel();
        self.subscribers.push(sender.clone());
        (sender, receiver)
    }

    pub fn update_and_subscribe_all(&mut self) -> Receiver<U>
    where
        T: StorageData<Change = U>,
    {
        let (sender, receiver) = self.subscribe_all_with_sender();
        self.data.send_as_changes(&sender);
        receiver
    }

    pub fn subscribe<K, V>(&mut self) -> Receiver<(K, Change<V>)>
    where
        K: Key<Data = T, Value = V>,
    {
        let table = K::table_mut(&mut self.data);
        let (sender, receiver) = channel();
        table.subscribers.push(sender);
        receiver
    }

    pub fn subscribe_with_sender<K, V>(
        &mut self,
    ) -> (Sender<(K, Change<V>)>, Receiver<(K, Change<V>)>)
    where
        K: Key<Data = T, Value = V>,
    {
        let table = K::table_mut(&mut self.data);
        let (sender, receiver) = channel();
        table.subscribers.push(sender.clone());
        (sender, receiver)
    }

    pub fn update_and_subscribe<K, V>(&mut self) -> Receiver<(K, Change<V>)>
    where
        K: Clone + Key<Data = T, Value = V>,
        V: Clone,
    {
        let (sender, recv) = self.subscribe_with_sender();
        let table = K::table_ref(&self.data);
        for (key, value) in &table.data {
            sender
                .send((key.clone(), Change::Added(value.clone())))
                .unwrap();
        }
        recv
    }

    pub fn put<K, V>(&mut self, key: K, value: V)
    where
        T: StorageData<Change = U>,
        U: Clone,
        K: Clone + Eq + Hash + Key<Data = T, Value = V>,
        V: Clone + PartialEq,
    {
        let table = K::table_mut(&mut self.data);
        let old_value = table.data.insert(key.clone(), value.clone());
        if let Some(old_value) = old_value {
            if value != old_value {
                let change = Change::Modified(&old_value, &value);
                table.send_change(&key, &change);
                self.send_change(&K::make_change(key, change.cloned()));
            }
        } else {
            let change = Change::Removed(&value);
            table.send_change(&key, &change);
            self.send_change(&K::make_change(key, change.cloned()));
        }
    }

    pub fn remove<K, V>(&mut self, key: &K) -> Option<V>
    where
        T: StorageData<Change = U>,
        U: Clone,
        K: Clone + Eq + Hash + Key<Data = T, Value = V>,
        V: Clone,
    {
        let table = K::table_mut(&mut self.data);
        let value = table.data.remove(key);
        if let Some(value) = value.as_ref() {
            let change = Change::Removed(&value).cloned();
            table.send_change(key, &change);
            self.send_change(&K::make_change(key.clone(), change.cloned()));
        }
        value
    }

    fn send_change(&mut self, change: &U)
    where
        U: Clone,
    {
        self.subscribers
            .retain(|sender| sender.send(change.clone()).is_ok());
    }
}

impl<K, V> Table<K, V> {
    fn send_change(&mut self, key: &K, value: &Change<&V>)
    where
        K: Clone,
        V: Clone,
    {
        self.subscribers
            .retain(|sender| sender.send((key.clone(), value.cloned())).is_ok());
    }
}
