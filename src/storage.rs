use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::mpsc::{channel, Receiver, Sender};

//use crate::model::{GraphId, NodeId};
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NodeId(&'static str);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GraphId(&'static str);

#[derive(Clone, Debug, Default)]
pub struct Storage<T>(T);

#[derive(Clone, Debug, Default)]
pub struct StorageData {
    node_graph_ids: Table<NodeId, NodeGraphId>,
    node_coords: Table<NodeId, NodeCoords>,
    // indexes:
    //graph_node_ids: Table<GraphId, GraphNodeIds>,
}

#[derive(Clone, Debug, Default)]
pub struct StorageIndex {
    graph_node_ids: Table<GraphId, GraphNodeIds>,
}

#[derive(Clone, Debug)]
pub struct Table<K, V> {
    map: HashMap<K, V>,
    senders: Vec<Sender<(K, Change<V>)>>,
}

impl<K, V> Default for Table<K, V> {
    fn default() -> Self {
        Self {
            map: HashMap::default(),
            senders: Vec::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Change<T> {
    Added(T),
    Modified(T, T),
    Removed(T),
}

pub trait Value: Sized {
    type Key;
    type Data;
    fn table_ref(data: &Self::Data) -> &Table<Self::Key, Self>;
    fn table_mut(data: &mut Self::Data) -> &mut Table<Self::Key, Self>;
}

pub trait WritableValue {}

impl<T> Storage<T> {
    pub fn get<K, V>(&self, key: &K) -> Option<&V>
    where
        K: 'static + Eq + Hash,
        V: Value<Key = K, Data = T>,
    {
        V::table_ref(&self.0).map.get(key)
    }

    pub fn put<K, V>(&mut self, key: K, value: V)
    where
        K: Clone + Eq + Hash,
        V: Clone + PartialEq + Value<Key = K, Data = T> + WritableValue,
    {
        let table = V::table_mut(&mut self.0);
        let old_value = table.map.insert(key.clone(), value.clone());
        if let Some(old_value) = old_value {
            if value != old_value {
                table.send((key, Change::Modified(old_value, value)));
            }
        } else {
            table.send((key, Change::Added(value)));
        }
    }

    pub fn remove<K, V>(&mut self, key: K) -> Option<V>
    where
        K: Clone + Eq + Hash,
        V: Clone + Value<Key = K, Data = T> + WritableValue,
    {
        let table = V::table_mut(&mut self.0);
        let value = table.map.remove(&key);
        if let Some(value) = value.as_ref() {
            table.send((key, Change::Removed(value.clone())));
        }
        value
    }

    pub fn subscribe<K, V>(&mut self) -> Receiver<(K, Change<V>)>
    where
        V: Value<Key = K, Data = T>,
    {
        let table = V::table_mut(&mut self.0);
        let (sender, receiver) = channel();
        table.senders.push(sender);
        receiver
    }
}

impl<K, V> Table<K, V> {
    fn send(&mut self, data: (K, Change<V>))
    where
        K: Clone,
        V: Clone,
    {
        self.senders
            .retain(|sender| sender.send(data.clone()).is_ok());
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NodeGraphId(GraphId);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphNodeIds(HashSet<NodeId>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NodeCoords {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl WritableValue for NodeGraphId {}
impl WritableValue for NodeCoords {}

impl Value for NodeGraphId {
    type Key = NodeId;
    type Data = StorageData;

    fn table_ref(data: &Self::Data) -> &Table<Self::Key, Self> {
        &data.node_graph_ids
    }

    fn table_mut(data: &mut Self::Data) -> &mut Table<Self::Key, Self> {
        &mut data.node_graph_ids
    }
}

impl Value for NodeCoords {
    type Key = NodeId;
    type Data = StorageData;

    fn table_ref(data: &Self::Data) -> &Table<Self::Key, Self> {
        &data.node_coords
    }

    fn table_mut(data: &mut Self::Data) -> &mut Table<Self::Key, Self> {
        &mut data.node_coords
    }
}

impl Value for GraphNodeIds {
    type Key = GraphId;
    type Data = StorageIndex;

    fn table_ref(data: &Self::Data) -> &Table<Self::Key, Self> {
        &data.graph_node_ids
    }

    fn table_mut(data: &mut Self::Data) -> &mut Table<Self::Key, Self> {
        &mut data.graph_node_ids
    }
}

#[test]
fn test_storage() {
    let mut storage = Storage::default();
    let node_graph_id_recv = storage.subscribe::<_, NodeGraphId>();
    let node_coords_recv = storage.subscribe::<_, NodeCoords>();

    let node_graph_id_recv2 = storage.subscribe::<_, NodeGraphId>();
    let mut index = Storage::default();
    let graph_node_id_recv = index.subscribe::<_, GraphNodeIds>();

    storage.put(NodeId("123"), NodeGraphId(GraphId("234")));
    storage.put(NodeId("123"), NodeGraphId(GraphId("345")));
    storage.remove::<_, NodeGraphId>(NodeId("123"));
    storage.put(
        NodeId("123"),
        NodeCoords {
            x: 1.2,
            y: 1.2,
            width: 1.2,
            height: 1.2,
        },
    );
    let graph_id: Option<&NodeGraphId> = storage.get(&NodeId("123"));
    let coords: Option<&NodeCoords> = storage.get(&NodeId("123"));

    while let Ok(change) = node_graph_id_recv.try_recv() {
        println!("{:?}", change);
    }
    while let Ok(change) = node_coords_recv.try_recv() {
        println!("{:?}", change);
    }

    // fn StorageIndex::update_index() {
    // while let Ok(change) = node_graph_id_recv2.try_recv() {
    //     // index.put()
    // }
    // }

    panic!();
}

/*
    model.put(NodeX { node_id }, 10);
    model.put(NodeY { node_id }, 10);
    model.put(NodeText { node_id }, "asd");
    let v = model.get(NodeSomething { node_id });

    model.recv() -> Change::NodeX(NodeX { node_id }, f64)

    subscriptions: HashMap<AnyKey, set of handlers>
    subscriptions: HashMap<AnyKey, set of handlers>


    model.put(node_id, NodeX(10));
    model.put(node_id, NodeY(10));
    model.put(node_id, NodeText("asd"));
    let NodeY(y) = model.get(node_id)
*/

/*
    GraphNodes { graph_id } -> HashSet<NodeId> (later)

    NodeGraph { node_id } -> GraphId
    NodeCoords { node_id } -> Coords
    NodeSize { node_id } -> Size
    NodeText { node_id } -> String
*/
