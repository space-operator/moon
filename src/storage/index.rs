use core::hash::Hash;
use std::collections::HashSet;

use crate::storage::{Change, Key, RawStorage, StorageData};

pub fn inverse_many_to_one<D, U, K1, K2, V1>(
    storage: &mut RawStorage<D, U>,
    key: K1,
    change: Change<V1>,
) where
    D: StorageData<Change = U>,
    U: Clone + From<(K2, Change<HashSet<K1>>)>,
    K1: Clone + Eq + Hash,
    K2: 'static + Clone + Eq + From<V1> + Hash + Key<Data = D, Value = HashSet<K1>>,
    V1: Clone,
{
    match change {
        Change::Added(value) => {
            let mut keys = storage
                .get(&K2::from(value.clone()))
                .cloned()
                .unwrap_or_default();
            keys.insert(key);
            storage.put(K2::from(value), keys);
        }
        Change::Modified(old_graph_id, new_graph_id) => {
            let mut keys = storage
                .get(&K2::from(old_graph_id.clone()))
                .cloned()
                .unwrap();
            keys.remove(&key);
            if keys.is_empty() {
                storage.remove(&K2::from(old_graph_id));
            } else {
                storage.put(K2::from(old_graph_id), keys);
            }

            let mut keys = storage
                .get(&K2::from(new_graph_id.clone()))
                .cloned()
                .unwrap_or_default();
            keys.insert(key);
            storage.put(K2::from(new_graph_id), keys);
        }
        Change::Removed(value) => {
            let mut keys = storage.get(&K2::from(value.clone())).cloned().unwrap();
            keys.remove(&key);
            if keys.is_empty() {
                storage.remove(&K2::from(value));
            } else {
                storage.put(K2::from(value), keys);
            }
        }
    }
}

pub fn inverse_many_to_one_if_modified<D, U, K1, K2, V1>(
    storage: &mut RawStorage<D, U>,
    key: K1,
    change: Change<V1>,
) where
    D: StorageData<Change = U>,
    U: Clone + From<(K2, Change<HashSet<K1>>)>,
    K1: Clone + Eq + Hash,
    K2: 'static + Clone + Eq + From<V1> + Hash + Key<Data = D, Value = HashSet<K1>>,
    V1: Clone + PartialEq,
{
    if let Some(change) = change.check_modified() {
        inverse_many_to_one(storage, key, change);
    }
}

pub fn inverse_many_to_one_property<D, U, K1, K2, U1, V1, F>(
    storage: &mut RawStorage<D, U>,
    key: K1,
    change: Change<U1>,
    func: F,
) where
    D: StorageData<Change = U>,
    U: Clone + From<(K2, Change<HashSet<K1>>)>,
    K1: Clone + Eq + Hash,
    K2: 'static + Clone + Eq + From<V1> + Hash + Key<Data = D, Value = HashSet<K1>>,
    V1: Clone + PartialEq,
    F: FnMut(U1) -> V1,
{
    if let Some(change) = change.map(func).check_modified() {
        inverse_many_to_one(storage, key, change);
    }
}
