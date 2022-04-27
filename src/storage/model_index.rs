use std::collections::HashSet;
use std::hash::Hash;
use std::sync::mpsc::{Receiver, Sender};

use derive_more::From;

use crate::storage::{
    Change, GraphId, GraphIdsNodeId, Key, ModelData, ModelDataChange, NodeId, Storage, StorageData,
    Table,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NodeIdsGraphId(pub GraphId);

#[derive(Clone, Debug, Default)]
pub struct ModelIndexData {
    graph_node_ids: Table<NodeIdsGraphId, HashSet<NodeId>>,
}

#[derive(Debug)]
pub struct ModelIndex {
    storage: Storage<ModelIndexData>,
    node_graph_ids_changes: Receiver<(GraphIdsNodeId, Change<GraphId>)>,
}

#[derive(Clone, Debug, From)]
pub enum ModelIndexDataChange {
    GraphNodeIds(NodeIdsGraphId, Change<HashSet<NodeId>>),
}

impl StorageData for ModelIndexData {
    type Change = ModelIndexDataChange;

    fn send_as_changes(&self, sender: &Sender<Self::Change>) {
        let ModelIndexData { graph_node_ids } = self;
        graph_node_ids.send_as_changes(sender);
    }
}

impl Key for NodeIdsGraphId {
    type Data = ModelIndexData;
    type Value = HashSet<NodeId>;

    fn table_ref(data: &Self::Data) -> &Table<Self, Self::Value> {
        &data.graph_node_ids
    }

    fn table_mut(data: &mut Self::Data) -> &mut Table<Self, Self::Value> {
        &mut data.graph_node_ids
    }

    fn make_change(key: Self, change: Change<Self::Value>) -> ModelIndexDataChange {
        ModelIndexDataChange::GraphNodeIds(key, change)
    }
}

impl ModelIndex {
    pub fn new(model: &mut Storage<ModelData>) -> Self {
        Self {
            storage: Storage::default(), // FIXME: build index from model
            node_graph_ids_changes: model.update_and_subscribe(),
        }
    }

    /// TODO
    ///
    /// Index is not automatically updated.
    /// In order to receive up-to-date result use index.update() before.
    pub fn get<K, V>(&self, key: &K) -> Option<&V>
    where
        K: 'static + Eq + Hash + Key<Data = ModelIndexData, Value = V>,
    {
        self.storage.get(key)
    }

    pub fn subscribe<K, V>(&mut self) -> Receiver<(K, Change<V>)>
    where
        K: Key<Data = ModelIndexData, Value = V>,
    {
        self.storage.subscribe()
    }

    pub fn update_and_subscribe_all(&mut self) -> Receiver<ModelIndexDataChange> {
        self.storage.update_and_subscribe_all()
    }

    pub fn update(&mut self) {
        while let Ok((node_id, change)) = self.node_graph_ids_changes.try_recv() {
            match change {
                Change::Added(graph_id) => {
                    let mut node_ids = self
                        .storage
                        .get(&NodeIdsGraphId(graph_id))
                        .cloned()
                        .unwrap_or_default();
                    node_ids.insert(node_id.0);
                    self.storage.put(NodeIdsGraphId(graph_id), node_ids);
                }
                Change::Modified(old_graph_id, new_graph_id) => {
                    let mut node_ids = self
                        .storage
                        .get(&NodeIdsGraphId(old_graph_id))
                        .cloned()
                        .unwrap();
                    node_ids.remove(&node_id.0);
                    if node_ids.is_empty() {
                        self.storage.remove(&NodeIdsGraphId(old_graph_id));
                    } else {
                        self.storage.put(NodeIdsGraphId(old_graph_id), node_ids);
                    }

                    let mut node_ids = self
                        .storage
                        .get(&NodeIdsGraphId(new_graph_id))
                        .cloned()
                        .unwrap_or_default();
                    node_ids.insert(node_id.0);
                    self.storage.put(NodeIdsGraphId(new_graph_id), node_ids);
                }
                Change::Removed(graph_id) => {
                    let mut node_ids = self
                        .storage
                        .get(&NodeIdsGraphId(graph_id))
                        .cloned()
                        .unwrap();
                    node_ids.remove(&node_id.0);
                    if node_ids.is_empty() {
                        self.storage.remove(&NodeIdsGraphId(graph_id));
                    } else {
                        self.storage.put(NodeIdsGraphId(graph_id), node_ids);
                    }
                }
            }
        }
    }
}

#[test]
fn test_model_index() {
    let mut storage: Storage<ModelData> = Storage::default();

    storage.put(GraphIdsNodeId(NodeId("node1")), GraphId("graph1"));
    storage.put(GraphIdsNodeId(NodeId("node2")), GraphId("graph2"));
    storage.put(GraphIdsNodeId(NodeId("node3")), GraphId("graph1"));
    storage.put(GraphIdsNodeId(NodeId("node4")), GraphId("graph2"));
    storage.put(GraphIdsNodeId(NodeId("node5")), GraphId("graph2"));

    storage.put(GraphIdsNodeId(NodeId("node2")), GraphId("graph1"));
    storage.put(GraphIdsNodeId(NodeId("node4")), GraphId("graph3"));
    storage.remove(&GraphIdsNodeId(NodeId("node1")));

    let mut index = ModelIndex::new(&mut storage);
    index.update();

    println!("{:#?}", storage);
    println!("{:#?}", index);
}

#[test]
fn text_example() {
    use crate::storage::*;

    #[derive(Clone, Debug, Default)]
    pub struct ViewIndexData {}

    #[derive(Clone, Debug, Default)]
    pub struct StateData {}

    #[derive(Debug)]
    pub struct ViewIndex {
        storage: Storage<ViewIndexData>,
    }

    #[derive(Clone, Debug)]
    pub enum ViewIndexDataChange {}

    impl StorageData for ViewIndexData {
        type Change = ViewIndexDataChange;
        fn send_as_changes(&self, _sender: &Sender<Self::Change>) {
            let ViewIndexData {} = self;
        }
    }

    #[derive(Clone, Debug)]
    pub enum StateDataChange {}

    impl StorageData for StateData {
        type Change = StateDataChange;
        fn send_as_changes(&self, _sender: &Sender<Self::Change>) {
            let StateData {} = self;
        }
    }

    impl ViewIndex {
        pub fn new(
            model: &mut Storage<ModelData>,
            state: &mut Storage<StateData>,
            model_index: &mut ModelIndex,
        ) -> Self {
            Self {
                storage: Storage::default(),
                // foo: model.update_and_subscribe(),
                // bar: model_index.update_and_subscribe(),
                // bazz: state.update_and_subscribe();
            }
        }
        pub fn update(&mut self) {}
    }

    fn init() {
        // store all this somewhere in env
        let mut model: Storage<ModelData> = Storage::default();
        let mut state: Storage<StateData> = Storage::default();
        let mut model_index = ModelIndex::new(&mut model);
        let view = ViewIndex::new(&mut model, &mut state, &mut model_index);
        //let mut ui_foo_table_recv: Receiver<(??, _)> = view.subscribe();
        //let mut ui_bar_table_recv: Receiver<(??, _)> = view.subscribe();
    }

    fn event_handler(ev: ()) {
        // take this from context or env
        let mut model: Storage<ModelData> = Storage::default();
        let mut state: Storage<StateData> = Storage::default();
        let mut model_index = ModelIndex::new(&mut model);
        let mut view = ViewIndex::new(&mut model, &mut state, &mut model_index);
        //let mut ui_foo_table_recv: Receiver<(??, _)> = view.subscribe();
        //let mut ui_bar_table_recv: Receiver<(??, _)> = view.subscribe();
        // take this from context or env

        // handle events in specific event handlers
        model.put(GraphIdsNodeId(NodeId("node1")), GraphId("graph1"));

        // do after each model modification
        model_index.update();

        // and then after model_index update dependencies (indexes)
        view.update();

        // iterator over ui_foo_table_recv and ui_bar_table_recv and others
        // and convert or serialize changes to json
        // and send it (deltas, changes) to flutter
        // update flutter view copy using changes
        // and handle only changed things
    }
}
