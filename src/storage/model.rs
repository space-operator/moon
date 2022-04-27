use std::sync::mpsc::{Receiver, Sender};

use derive_more::From;

use crate::storage::{Change, Key, Storage, StorageData, Table};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NodeId(pub &'static str);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GraphId(pub &'static str);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GraphIdsNodeId(pub NodeId);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NodeCoordsNodeId(pub NodeId);

#[derive(Clone, Debug, Default)]
pub struct ModelData {
    node_graph_ids: Table<GraphIdsNodeId, GraphId>,
    node_coords: Table<NodeCoordsNodeId, NodeCoords>,
}

#[derive(Clone, Debug, From)]
pub enum ModelDataChange {
    NodeGraphIds((GraphIdsNodeId, Change<GraphId>)),
    NodeCoords((NodeCoordsNodeId, Change<NodeCoords>)),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NodeCoords {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl StorageData for ModelData {
    type Change = ModelDataChange;

    fn send_as_changes(&self, sender: &Sender<Self::Change>) {
        let ModelData {
            node_graph_ids,
            node_coords,
        } = self;
        node_graph_ids.send_as_changes(sender);
        node_coords.send_as_changes(sender);
    }
}

impl Key for GraphIdsNodeId {
    type Data = ModelData;
    type Value = GraphId;

    fn table_ref(data: &Self::Data) -> &Table<Self, Self::Value> {
        &data.node_graph_ids
    }

    fn table_mut(data: &mut Self::Data) -> &mut Table<Self, Self::Value> {
        &mut data.node_graph_ids
    }

    fn make_change(key: Self, change: Change<Self::Value>) -> ModelDataChange {
        ModelDataChange::NodeGraphIds((key, change))
    }
}

impl Key for NodeCoordsNodeId {
    type Data = ModelData;
    type Value = NodeCoords;

    fn table_ref(data: &Self::Data) -> &Table<Self, Self::Value> {
        &data.node_coords
    }

    fn table_mut(data: &mut Self::Data) -> &mut Table<Self, Self::Value> {
        &mut data.node_coords
    }

    fn make_change(key: Self, change: Change<Self::Value>) -> ModelDataChange {
        ModelDataChange::NodeCoords((key, change))
    }
}

#[test]
fn test_model() {
    let mut storage = Storage::default();
    let node_graph_id_recv: Receiver<(GraphIdsNodeId, _)> = storage.subscribe();
    let node_coords_recv: Receiver<(NodeCoordsNodeId, _)> = storage.subscribe();

    storage.put(GraphIdsNodeId(NodeId("123")), GraphId("234"));
    storage.put(GraphIdsNodeId(NodeId("123")), GraphId("345"));
    storage.remove(&GraphIdsNodeId(NodeId("123")));
    storage.put(GraphIdsNodeId(NodeId("234")), GraphId("456"));
    storage.put(
        NodeCoordsNodeId(NodeId("123")),
        NodeCoords {
            x: 1.2,
            y: 1.2,
            width: 1.2,
            height: 1.2,
        },
    );
    let graph_id: Option<&GraphId> = storage.get(&GraphIdsNodeId(NodeId("123")));
    let coords: Option<&NodeCoords> = storage.get(&NodeCoordsNodeId(NodeId("123")));

    let changes: Receiver<_> = storage.update_and_subscribe_all();
    while let Ok(change) = changes.try_recv() {
        println!("{:?}", change);
    }
    //while let Ok(change) = node_graph_id_recv.try_recv() {
    //    println!("{:?}", change);
    //}
    //while let Ok(change) = node_coords_recv.try_recv() {
    //    println!("{:?}", change);
    //}

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
