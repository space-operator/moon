use std::collections::HashSet;
use std::hash::Hash;
use std::sync::mpsc::{channel, Receiver, Sender};

use derive_more::From;

use crate::{
    define_id, define_key,
    storage::{
        inverse_many_to_one, inverse_many_to_one_if_modified, inverse_many_to_one_property,
        BookmarkData, BookmarkId, BookmarkNodeData, BookmarkNodeId, Change, DataBookmarkNodeId,
        DataEdgeId, DataNodeId, EdgeData, EdgeId, GraphId, InputId, Key, ModelData, NodeData,
        NodeId, OutputId, ReadOnlyStorage, Storage, StorageData, Table,
    },
};

define_id!(NodeIdsGraphId: GraphId);
define_id!(OutputEdgeIdsNodeId: NodeId);
define_id!(InputEdgeIdsNodeId: NodeId);
define_id!(EdgeIdsInputId: InputId);
define_id!(EdgeIdsOutputId: OutputId);
define_id!(NodeIdsBookmarkId: BookmarkId);
define_id!(BookmarkIdsNodeId: NodeId);

#[derive(Clone, Debug, Default)]
pub struct ModelIndexData {
    graph_id_node_ids: Table<NodeIdsGraphId, HashSet<NodeId>>,
    node_id_output_edge_ids: Table<OutputEdgeIdsNodeId, HashSet<EdgeId>>,
    node_id_input_edge_ids: Table<InputEdgeIdsNodeId, HashSet<EdgeId>>,
    input_id_edge_ids: Table<EdgeIdsInputId, HashSet<EdgeId>>,
    output_id_edge_ids: Table<EdgeIdsOutputId, HashSet<EdgeId>>,
    bookmark_id_nodes_ids: Table<NodeIdsBookmarkId, HashSet<NodeId>>,
    node_id_bookmark_ids: Table<BookmarkIdsNodeId, HashSet<BookmarkId>>,
}

#[derive(Debug)]
pub struct ModelIndex {
    storage: Storage<ModelIndexData>,
    node_id_data_changes: Receiver<(DataNodeId, Change<NodeData>)>,
    edge_id_data_changes: Receiver<(DataEdgeId, Change<EdgeData>)>,
    bookmark_node_id_data: Receiver<(DataBookmarkNodeId, Change<BookmarkNodeData>)>,
}

#[derive(Clone, Debug, From)]
pub enum ModelIndexDataChange {
    GraphIdNodeIds(NodeIdsGraphId, Change<HashSet<NodeId>>),
    NodeIdOutputEdgeIds(OutputEdgeIdsNodeId, Change<HashSet<EdgeId>>),
    NodeIdInputEdgeIds(InputEdgeIdsNodeId, Change<HashSet<EdgeId>>),
    InputIdEdgeIds(EdgeIdsInputId, Change<HashSet<EdgeId>>),
    OutputIdEdgeIds(EdgeIdsOutputId, Change<HashSet<EdgeId>>),
    BookmarkIdNodeId(NodeIdsBookmarkId, Change<HashSet<NodeId>>),
    NodeIdBookmarkId(BookmarkIdsNodeId, Change<HashSet<BookmarkId>>),
}

impl StorageData for ModelIndexData {
    type Change = ModelIndexDataChange;

    fn send_as_changes_to(&self, sender: &Sender<Self::Change>) {
        let ModelIndexData {
            graph_id_node_ids,
            node_id_output_edge_ids,
            node_id_input_edge_ids,
            input_id_edge_ids,
            output_id_edge_ids,
            bookmark_id_nodes_ids,
            node_id_bookmark_ids,
        } = self;
        graph_id_node_ids.send_as_changes_to(sender);
        node_id_output_edge_ids.send_as_changes_to(sender);
        node_id_input_edge_ids.send_as_changes_to(sender);
        input_id_edge_ids.send_as_changes_to(sender);
        output_id_edge_ids.send_as_changes_to(sender);
        bookmark_id_nodes_ids.send_as_changes_to(sender);
        node_id_bookmark_ids.send_as_changes_to(sender);
    }
}

define_key!(
    ModelIndexData,
    graph_id_node_ids,
    NodeIdsGraphId,
    HashSet<NodeId>
);
define_key!(
    ModelIndexData,
    node_id_output_edge_ids,
    OutputEdgeIdsNodeId,
    HashSet<EdgeId>
);
define_key!(
    ModelIndexData,
    node_id_input_edge_ids,
    InputEdgeIdsNodeId,
    HashSet<EdgeId>
);
define_key!(
    ModelIndexData,
    input_id_edge_ids,
    EdgeIdsInputId,
    HashSet<EdgeId>
);
define_key!(
    ModelIndexData,
    output_id_edge_ids,
    EdgeIdsOutputId,
    HashSet<EdgeId>
);
define_key!(
    ModelIndexData,
    bookmark_id_nodes_ids,
    NodeIdsBookmarkId,
    HashSet<NodeId>
);
define_key!(
    ModelIndexData,
    node_id_bookmark_ids,
    BookmarkIdsNodeId,
    HashSet<BookmarkId>
);

impl ModelIndex {
    pub fn new(model: &mut Storage<ModelData>) -> Self {
        Self {
            storage: Storage::default(),
            node_id_data_changes: model.table_mut().update_and_subscribe(),
            edge_id_data_changes: model.table_mut().update_and_subscribe(),
            bookmark_node_id_data: model.table_mut().update_and_subscribe(),
        }
    }

    pub fn storage(&self) -> ReadOnlyStorage<&Storage<ModelIndexData>> {
        ReadOnlyStorage::new(&self.storage)
    }

    pub fn update(&mut self) {
        let ModelIndex {
            storage: _,
            node_id_data_changes,
            edge_id_data_changes,
            bookmark_node_id_data,
        } = self;
        while let Ok((key, change)) = node_id_data_changes.try_recv() {
            inverse_many_to_one_property::<_, _, _, NodeIdsGraphId, _, _, _>(
                &mut self.storage,
                key.0,
                change,
                |data| data.graph_id,
            );
        }

        while let Ok((key, change)) = edge_id_data_changes.try_recv() {
            let (
                source_node_id_change,
                target_node_id_change,
                source_output_id_change,
                target_input_id_change,
            ) = change
                .map(|data| {
                    (
                        data.source_node_id,
                        data.target_node_id,
                        data.source_output_id,
                        data.target_input_id,
                    )
                })
                .split();
            inverse_many_to_one_if_modified::<_, _, _, OutputEdgeIdsNodeId, _>(
                &mut self.storage,
                key.0,
                source_node_id_change,
            );
            inverse_many_to_one_if_modified::<_, _, _, InputEdgeIdsNodeId, _>(
                &mut self.storage,
                key.0,
                target_node_id_change,
            );
            inverse_many_to_one_if_modified::<_, _, _, EdgeIdsOutputId, _>(
                &mut self.storage,
                key.0,
                source_output_id_change,
            );
            inverse_many_to_one_if_modified::<_, _, _, EdgeIdsInputId, _>(
                &mut self.storage,
                key.0,
                target_input_id_change,
            );
        }

        while let Ok((_, change)) = bookmark_node_id_data.try_recv() {
            match change {
                Change::Added(data) => {
                    inverse_many_to_one::<_, _, _, BookmarkIdsNodeId, _>(
                        &mut self.storage,
                        data.bookmark_id,
                        Change::Added(data.node_id),
                    );
                    inverse_many_to_one::<_, _, _, NodeIdsBookmarkId, _>(
                        &mut self.storage,
                        data.node_id,
                        Change::Added(data.bookmark_id),
                    );
                }
                Change::Removed(data) => {
                    inverse_many_to_one::<_, _, _, BookmarkIdsNodeId, _>(
                        &mut self.storage,
                        data.bookmark_id,
                        Change::Removed(data.node_id),
                    );
                    inverse_many_to_one::<_, _, _, NodeIdsBookmarkId, _>(
                        &mut self.storage,
                        data.node_id,
                        Change::Removed(data.bookmark_id),
                    );
                }
                Change::Modified(old_data, new_data) => {
                    match (
                        old_data.bookmark_id != new_data.bookmark_id,
                        old_data.node_id != new_data.node_id,
                    ) {
                        (false, false) => {}
                        (false, true) => {
                            inverse_many_to_one::<_, _, _, NodeIdsBookmarkId, _>(
                                &mut self.storage,
                                old_data.node_id,
                                Change::Removed(old_data.bookmark_id),
                            );
                            inverse_many_to_one::<_, _, _, NodeIdsBookmarkId, _>(
                                &mut self.storage,
                                new_data.node_id,
                                Change::Added(new_data.bookmark_id),
                            );
                            inverse_many_to_one::<_, _, _, BookmarkIdsNodeId, _>(
                                &mut self.storage,
                                old_data.bookmark_id,
                                Change::Modified(old_data.node_id, new_data.node_id),
                            );
                        }
                        (true, false) => {
                            inverse_many_to_one::<_, _, _, BookmarkIdsNodeId, _>(
                                &mut self.storage,
                                old_data.bookmark_id,
                                Change::Removed(old_data.node_id),
                            );
                            inverse_many_to_one::<_, _, _, BookmarkIdsNodeId, _>(
                                &mut self.storage,
                                new_data.bookmark_id,
                                Change::Added(new_data.node_id),
                            );
                            inverse_many_to_one::<_, _, _, NodeIdsBookmarkId, _>(
                                &mut self.storage,
                                old_data.node_id,
                                Change::Modified(old_data.bookmark_id, new_data.bookmark_id),
                            );
                        }
                        (true, true) => {
                            inverse_many_to_one::<_, _, _, BookmarkIdsNodeId, _>(
                                &mut self.storage,
                                old_data.bookmark_id,
                                Change::Removed(old_data.node_id),
                            );
                            inverse_many_to_one::<_, _, _, NodeIdsBookmarkId, _>(
                                &mut self.storage,
                                old_data.node_id,
                                Change::Removed(old_data.bookmark_id),
                            );
                            inverse_many_to_one::<_, _, _, BookmarkIdsNodeId, _>(
                                &mut self.storage,
                                new_data.bookmark_id,
                                Change::Added(new_data.node_id),
                            );
                            inverse_many_to_one::<_, _, _, NodeIdsBookmarkId, _>(
                                &mut self.storage,
                                new_data.node_id,
                                Change::Added(new_data.bookmark_id),
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_node_traversing() {
    use crate::storage::{DataInputId, DataOutputId, InputData, NodeTemplate, OutputData};

    const I1: InputId = InputId("i1");
    const I2: InputId = InputId("i2");
    const O1: OutputId = OutputId("o1");
    const O2: OutputId = OutputId("o2");

    const G1: GraphId = GraphId("g1");
    const T1: NodeTemplate = NodeTemplate("t1");

    const N1: NodeId = NodeId("n1");
    const N2: NodeId = NodeId("n2");
    const N3: NodeId = NodeId("n3");
    const N1O1N2I1: EdgeId = EdgeId("n1o1n2i1");
    const N1O2N2I1: EdgeId = EdgeId("n1o2n2i1");
    const N1O1N3I1: EdgeId = EdgeId("n1o1n3i1");

    let mut storage: Storage<ModelData> = Storage::default();

    storage.put(DataInputId(I1), InputData { name: "i1n" });
    storage.put(DataInputId(I2), InputData { name: "i2n" });
    storage.put(DataOutputId(O1), OutputData { name: "o1n" });
    storage.put(DataOutputId(O2), OutputData { name: "o2n" });

    storage.put(
        DataNodeId(N1),
        NodeData {
            template: T1,
            graph_id: G1,
        },
    );
    storage.put(
        DataNodeId(N2),
        NodeData {
            template: T1,
            graph_id: G1,
        },
    );
    storage.put(
        DataNodeId(N3),
        NodeData {
            template: T1,
            graph_id: G1,
        },
    );

    storage.put(
        DataEdgeId(N1O1N2I1),
        EdgeData {
            source_node_id: N1,
            target_node_id: N2,
            source_output_id: O1,
            target_input_id: I1,
        },
    );

    storage.put(
        DataEdgeId(N1O2N2I1),
        EdgeData {
            source_node_id: N1,
            target_node_id: N2,
            source_output_id: O2,
            target_input_id: I1,
        },
    );

    storage.put(
        DataEdgeId(N1O1N3I1),
        EdgeData {
            source_node_id: N1,
            target_node_id: N3,
            source_output_id: O1,
            target_input_id: I1,
        },
    );

    let mut index = ModelIndex::new(&mut storage);
    index.update();

    for &output_node_id in index.storage().get(&NodeIdsGraphId(G1)).unwrap() {
        for &edge_id in index
            .storage()
            .get(&OutputEdgeIdsNodeId(output_node_id))
            .unwrap_or(&HashSet::new())
        {
            let edge_data = storage.get(&DataEdgeId(edge_id)).unwrap();
            let input_node_id = edge_data.target_node_id;
            let output_id = edge_data.source_output_id;
            let input_id = edge_data.target_input_id;
            let output_name = storage.get(&DataOutputId(output_id)).unwrap();
            let input_name = storage.get(&DataInputId(input_id)).unwrap();
            println!(
                "{}-{}-{}-{}-{}",
                output_node_id.0, output_name.name, edge_id.0, input_name.name, input_node_id.0
            );
        }
    }

    for &input_node_id in index.storage().get(&NodeIdsGraphId(G1)).unwrap() {
        for &edge_id in index
            .storage()
            .get(&InputEdgeIdsNodeId(input_node_id))
            .unwrap_or(&HashSet::new())
        {
            let edge_data = storage.get(&DataEdgeId(edge_id)).unwrap();
            let output_node_id = edge_data.source_node_id;
            let output_id = edge_data.source_output_id;
            let input_id = edge_data.target_input_id;
            let output_name = storage.get(&DataOutputId(output_id)).unwrap();
            let input_name = storage.get(&DataInputId(input_id)).unwrap();
            println!(
                "{}-{}-{}-{}-{}",
                output_node_id.0, output_name.name, edge_id.0, input_name.name, input_node_id.0
            );
        }
    }
}

#[test]
fn test_bookmarks() {
    use crate::storage::{
        DataBookmarkId, DataInputId, DataOutputId, InputData, NodeTemplate, OutputData,
    };

    const G1: GraphId = GraphId("g1");
    const T1: NodeTemplate = NodeTemplate("t1");

    const N1: NodeId = NodeId("n1");
    const N2: NodeId = NodeId("n2");
    const N3: NodeId = NodeId("n3");

    const B1: BookmarkId = BookmarkId("b1");
    const B2: BookmarkId = BookmarkId("b2");
    const B3: BookmarkId = BookmarkId("b3");

    const Y1: BookmarkNodeId = BookmarkNodeId("y1");
    const Y2: BookmarkNodeId = BookmarkNodeId("y2");
    const Y3: BookmarkNodeId = BookmarkNodeId("y3");
    const Y4: BookmarkNodeId = BookmarkNodeId("y4");
    const Y5: BookmarkNodeId = BookmarkNodeId("y5");
    const Y6: BookmarkNodeId = BookmarkNodeId("y6");

    let mut storage: Storage<ModelData> = Storage::default();
    let mut index = ModelIndex::new(&mut storage);
    index.update();

    storage.put(
        DataNodeId(N1),
        NodeData {
            template: T1,
            graph_id: G1,
        },
    );
    storage.put(
        DataNodeId(N2),
        NodeData {
            template: T1,
            graph_id: G1,
        },
    );
    storage.put(
        DataNodeId(N3),
        NodeData {
            template: T1,
            graph_id: G1,
        },
    );
    index.update();

    storage.put(DataBookmarkId(B1), BookmarkData { name: "name" });
    storage.put(DataBookmarkId(B2), BookmarkData { name: "name" });
    storage.put(DataBookmarkId(B3), BookmarkData { name: "name" });
    index.update();

    let update_and_dump = |index: &mut ModelIndex| {
        index.update();
        println!(
            "{:?}",
            index
                .storage()
                .table_ref::<'_, _, _, NodeIdsBookmarkId, _>()
                .data()
        );
        println!(
            "{:?}",
            index
                .storage()
                .table_ref::<'_, _, _, BookmarkIdsNodeId, _>()
                .data()
        );
        println!();
    };

    storage.put(
        DataBookmarkNodeId(Y1),
        BookmarkNodeData {
            bookmark_id: B1,
            node_id: N1,
        },
    );
    update_and_dump(&mut index);

    storage.put(
        DataBookmarkNodeId(Y2),
        BookmarkNodeData {
            bookmark_id: B2,
            node_id: N2,
        },
    );
    update_and_dump(&mut index);

    storage.put(
        DataBookmarkNodeId(Y3),
        BookmarkNodeData {
            bookmark_id: B1,
            node_id: N2,
        },
    );
    update_and_dump(&mut index);

    storage.put(
        DataBookmarkNodeId(Y4),
        BookmarkNodeData {
            bookmark_id: B2,
            node_id: N1,
        },
    );
    update_and_dump(&mut index);

    storage.remove(&DataBookmarkNodeId(Y1));
    update_and_dump(&mut index);

    storage.remove(&DataBookmarkNodeId(Y2));
    update_and_dump(&mut index);

    storage.put(
        DataBookmarkNodeId(Y3),
        BookmarkNodeData {
            bookmark_id: B2,
            node_id: N3,
        },
    );
    update_and_dump(&mut index);

    storage.put(
        DataBookmarkNodeId(Y4),
        BookmarkNodeData {
            bookmark_id: B1,
            node_id: N1,
        },
    );
    update_and_dump(&mut index);

    panic!();
}

/*
#[test]
fn test_model_index() {
    let mut storage: Storage<ModelData> = Storage::default();

    storage.put(GraphIdNodeId(NodeId("node1")), GraphId("graph1"));
    storage.put(GraphIdNodeId(NodeId("node2")), GraphId("graph2"));
    storage.put(GraphIdNodeId(NodeId("node3")), GraphId("graph1"));
    storage.put(GraphIdNodeId(NodeId("node4")), GraphId("graph2"));
    storage.put(GraphIdNodeId(NodeId("node5")), GraphId("graph2"));

    storage.put(GraphIdNodeId(NodeId("node2")), GraphId("graph1"));
    storage.put(GraphIdNodeId(NodeId("node4")), GraphId("graph3"));
    storage.remove(&GraphIdNodeId(NodeId("node1")));

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
        fn send_as_changes_to(&self, _sender: &Sender<Self::Change>) {
            let ViewIndexData {} = self;
        }
    }

    #[derive(Clone, Debug)]
    pub enum StateDataChange {}

    impl StorageData for StateData {
        type Change = StateDataChange;
        fn send_as_changes_to(&self, _sender: &Sender<Self::Change>) {
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
        model.put(GraphIdNodeId(NodeId("node1")), GraphId("graph1"));

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
*/
