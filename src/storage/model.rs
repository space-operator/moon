use std::sync::mpsc::Sender;

use derive_more::From;

use crate::{
    define_id, define_key,
    storage::{Change, Key, StorageData, Table},
};

define_id!(NodeId: &'static str);
define_id!(GraphId: &'static str);
define_id!(InputId: &'static str);
define_id!(OutputId: &'static str);
define_id!(EdgeId: &'static str);
define_id!(NodeTemplate: &'static str);
define_id!(BookmarkId: &'static str);
define_id!(BookmarkNodeId: &'static str);

define_id!(DataNodeId: NodeId);
define_id!(ViewNodeId: NodeId);
define_id!(TextNodeId: NodeId);
define_id!(DataEdgeId: EdgeId);
define_id!(DataInputId: InputId);
define_id!(DataOutputId: OutputId);
define_id!(DataBookmarkId: BookmarkId);
define_id!(DataBookmarkNodeId: BookmarkNodeId);

#[derive(Clone, Debug, Default)]
pub struct ModelData {
    node_id_data: Table<DataNodeId, NodeData>,
    node_id_view: Table<ViewNodeId, NodeView>,
    node_id_text: Table<TextNodeId, String>,
    edge_id_data: Table<DataEdgeId, EdgeData>,
    input_id_data: Table<DataInputId, InputData>,
    output_id_data: Table<DataOutputId, OutputData>,
    bookmark_id_data: Table<DataBookmarkId, BookmarkData>,
    bookmark_node_id_data: Table<DataBookmarkNodeId, BookmarkNodeData>,
}

#[derive(Clone, Debug, From)]
pub enum ModelDataChange {
    NodeIdData((DataNodeId, Change<NodeData>)),
    NodeIdView((ViewNodeId, Change<NodeView>)),
    NodeIdText((TextNodeId, Change<String>)),
    EdgeIdData((DataEdgeId, Change<EdgeData>)),
    InputIdData((DataInputId, Change<InputData>)),
    OutputIdData((DataOutputId, Change<OutputData>)),
    BookmarkIdData((DataBookmarkId, Change<BookmarkData>)),
    BookmarkNodeIdData((DataBookmarkNodeId, Change<BookmarkNodeData>)),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NodeData {
    pub template: NodeTemplate,
    pub graph_id: GraphId,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NodeView {
    pub coords: NodeCoords,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EdgeData {
    pub source_node_id: NodeId,
    pub target_node_id: NodeId,
    pub source_output_id: OutputId,
    pub target_input_id: InputId,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct InputData {
    pub name: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OutputData {
    pub name: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BookmarkData {
    pub name: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BookmarkNodeData {
    pub bookmark_id: BookmarkId,
    pub node_id: NodeId,
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

    fn send_as_changes_to(&self, sender: &Sender<Self::Change>) {
        let ModelData {
            node_id_data,
            node_id_view,
            node_id_text,
            edge_id_data,
            input_id_data,
            output_id_data,
            bookmark_id_data,
            bookmark_node_id_data,
        } = self;

        node_id_data.send_as_changes_to(sender);
        node_id_view.send_as_changes_to(sender);
        node_id_text.send_as_changes_to(sender);
        edge_id_data.send_as_changes_to(sender);
        input_id_data.send_as_changes_to(sender);
        output_id_data.send_as_changes_to(sender);
        bookmark_id_data.send_as_changes_to(sender);
        bookmark_node_id_data.send_as_changes_to(sender);
    }
}

define_key!(ModelData, node_id_data, DataNodeId, NodeData);
define_key!(ModelData, node_id_view, ViewNodeId, NodeView);
define_key!(ModelData, node_id_text, TextNodeId, String);
define_key!(ModelData, edge_id_data, DataEdgeId, EdgeData);
define_key!(ModelData, input_id_data, DataInputId, InputData);
define_key!(ModelData, output_id_data, DataOutputId, OutputData);
define_key!(ModelData, bookmark_id_data, DataBookmarkId, BookmarkData);
define_key!(
    ModelData,
    bookmark_node_id_data,
    DataBookmarkNodeId,
    BookmarkNodeData
);

/*
#[test]
fn test_model() {
    use crate::storage::Storage;
    use std::sync::mpsc::{channel, Receiver};

    let mut storage = Storage::default();
    let node_id_graph_id_recv: Receiver<(GraphIdNodeId, _)> = storage.table_mut().subscribe();
    let node_id_coords_recv: Receiver<(NodeCoordsNodeId, _)> = storage.table_mut().subscribe();

    storage.put(GraphIdNodeId(NodeId("123")), GraphId("234"));
    storage.put(GraphIdNodeId(NodeId("123")), GraphId("345"));
    storage.remove(&GraphIdNodeId(NodeId("123")));
    storage.put(GraphIdNodeId(NodeId("234")), GraphId("456"));
    storage.put(
        NodeCoordsNodeId(NodeId("123")),
        NodeCoords {
            x: 1.2,
            y: 1.2,
            width: 1.2,
            height: 1.2,
        },
    );
    let graph_id: Option<&GraphId> = storage.get(&GraphIdNodeId(NodeId("123")));
    let coords: Option<&NodeCoords> = storage.get(&NodeCoordsNodeId(NodeId("123")));

    let (sender, receiver) = channel();
    storage.send_as_changes_to(&sender);
    storage.subscribe_to(sender);

    while let Ok(change) = receiver.try_recv() {
        println!("{:?}", change);
    }
    while let Ok(change) = node_id_graph_id_recv.try_recv() {
        println!("{:?}", change);
    }
    while let Ok(change) = node_id_coords_recv.try_recv() {
        println!("{:?}", change);
    }

    panic!();
}*/

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
