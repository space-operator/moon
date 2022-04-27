use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::sync::Arc;

use futures::executor::block_on;
use serde::Deserialize;
use serde::Serialize;
use sunshine_core::msg::*;
use sunshine_core::store::*;

use dashmap::DashMap;
use serde_json::{json, Value as JsonValue};
use std::sync::Mutex;
use sunshine_indra::store::generate_uuid_v1;
use sunshine_indra::store::DbConfig;
use sunshine_indra::store::DB;
use sunshine_solana::RunState;

use sunshine_solana::commands::solana;
use sunshine_solana::commands::solana::SolanaNet as BackendSolanaNet;
use sunshine_solana::COMMAND_NAME_MARKER;
use sunshine_solana::{
    commands::simple::Command as SimpleCommand, commands::CommandKind, CommandConfig,
    ContextConfig, COMMAND_MARKER, CTX_EDGE_MARKER, CTX_MARKER, INPUT_ARG_NAME_MARKER,
    OUTPUT_ARG_NAME_MARKER, START_NODE_MARKER,
};
use uuid::Uuid;

use std::str::FromStr;

use crate::command::commands_map;
use crate::command::TypeBound;
use crate::command::INPUT_SIZE;
use crate::flow_context::FlowContext;

//use crate::model_ext::WidgetType;

use crate::event::Coords;
use crate::utils::Rect;

pub const COORDS_MARKER: &str = "COORDS_MARKER";
pub const DIMENSIONS_MARKER: &str = "DIMENSIONS_MARKER";
pub const BLOCK_MARKER: &str = "BLOCK_MARKER";
pub const TEXT_INPUT_MARKER: &str = "TEXT_INPUT_MARKER";
pub const DATA_MARKER: &str = "DATA_MARKER";
pub const BLOCK_TO_CMD_EDGE_MARKER: &str = "BLOCK_TO_CMD_EDGE_MARKER";
pub const FLOW_GRAPH_MARKER: &str = "FLOW_GRAPH_MARKER";
pub const TEXT_MARKER: &str = "TEXT_MARKER";
pub const ADDITIONAL_DATA_MARKER: &str = "ADDITIONAL_DATA_MARKER";
pub const REQ_ID: &str = "REQ_ID";
pub const BOOKMARKS: &str = "BOOKMARKS";
pub const BOOKMARK_NAME: &str = "BOOKMARK_NAME";

pub const INPUT_OFFSET: i64 = 50;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct GraphId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct NodeId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct EdgeId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct NodeEdgeId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct InputId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct OutputId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct BookmarkId(pub Uuid);

#[rid::model]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GraphEntry {
    id: String,
    pub name: String,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum PortId {
    Input(InputId),
    Output(OutputId),
}

#[derive(Clone)]
pub struct Db(Arc<DB>);

#[derive(Debug)]
pub struct Model {
    db: Db,
    pub graph_list: Vec<GraphEntry>,
    graph_id: Arc<Mutex<GraphId>>,
    context_node_id: NodeId,
    nodes: HashMap<NodeId, NodeModel>,
    node_edges: HashMap<NodeEdgeId, NodeEdgeModel>,
    flow_edges: HashMap<EdgeId, FlowEdgeModel>,
    inputs: HashMap<InputId, InputModel>,
    outputs: HashMap<OutputId, OutputModel>,
    pub bookmarks: HashMap<BookmarkId, BookmarkModel>,
    flow_context: FlowContext,
    pub run_status: Arc<DashMap<NodeId, (RunState, Option<String>)>>,
    pub req_id: Arc<Mutex<u64>>,
    pub solana_net: SolanaNet,
}

impl fmt::Debug for Db {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Db").finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")] // https://serde.rs/container-attrs.html
pub enum NodeModel {
    Widget(WidgetNodeData),
    // Data(DataNodeData),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WidgetNodeData {
    pub coords: Coords,
    pub dimensions: NodeDimensions,
    pub command_name: Option<String>,
    pub kind: WidgetKind,
    pub text: String,
    pub additional_data: String,
}

impl WidgetNodeData {
    fn new_command(command_name: &str, kind: WidgetKind, width: i64, height: i64) -> Self {
        Self {
            coords: Coords { x: 0.0, y: 0.0 },
            dimensions: NodeDimensions { width, height },
            command_name: Some(command_name.to_owned()),
            kind,
            text: String::new(),
            additional_data: String::new(),
        }
    }

    // fn new_const_command(value: Value, kind: WidgetKind, width: i64, height: i64) -> Self {
    //     Self {
    //         coords: Coords { x: 0, y: 0 },
    //         dimensions: NodeDimensions { width, height },
    //         kind,
    //         text: match value {
    //             Value::Integer(_) => todo!(),
    //             Value::Keypair(_) => todo!(),
    //             Value::String(string) => string,
    //             Value::NodeId(_) => todo!(),
    //             Value::DeletedNode(_) => todo!(),
    //             Value::Pubkey(_) => todo!(),
    //             Value::Success(_) => todo!(),
    //             Value::Balance(_) => todo!(),
    //             Value::U8(_) => todo!(),
    //             Value::U16(_) => todo!(),
    //             Value::U64(_) => todo!(),
    //             Value::F64(_) => todo!(),
    //             Value::Bool(_) => todo!(),
    //             Value::StringOpt(_) => todo!(),
    //             Value::Empty => todo!(),
    //             Value::NodeIdOpt(_) => todo!(),
    //             Value::NftCreators(_) => todo!(),
    //             Value::MetadataAccountData(_) => todo!(),
    //         },
    //     }
    // }

    fn new_block(coords: Coords) -> Self {
        Self {
            coords,
            dimensions: NodeDimensions {
                height: 75,
                width: 300,
            },
            command_name: None,
            kind: WidgetKind::Basic(BasicWidgetKind::Block),
            text: String::new(),
            additional_data: String::new(),
        }
    }

    fn new_text_input() -> Self {
        Self {
            coords: Coords { x: 0.0, y: 0.0 },
            dimensions: NodeDimensions {
                height: 70,
                width: 300,
            },
            command_name: None,
            kind: WidgetKind::Basic(BasicWidgetKind::TextInput),
            text: String::new(),
            additional_data: String::new(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct DataNodeData {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct NodeEdgeModel {
    pub from: NodeId,
    pub to: NodeId,
    pub data: EdgeModelData,
}

/// From, to coords needed to render the edges on Flutter side
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EdgeModelData {
    pub edge_type: EdgeType,
    pub from_coords: Option<Coords>,
    pub to_coords: Option<Coords>,
}
#[derive(Debug, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct NodeDimensions {
    pub height: i64,
    pub width: i64,
}

#[derive(Debug, Clone, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum EdgeType {
    Child,
    Data,
    Flow,
}
#[derive(Clone, Debug)]
pub struct NodeAndDataResult {
    // pub data_node_id: NodeId,
    pub widget_node_id: NodeId,
    // pub edge_id: NodeEdgeId,
}

/// Model for input output edges
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FlowEdgeModel {
    pub input_id: InputId,
    pub output_id: OutputId,
    pub edge_type: EdgeType,
    pub db_edge_id: EdgeId,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InputModel {
    pub parent_node_id: NodeId,
    pub command_id: NodeId,
    pub local_coords: Coords,
    pub label: String,
    pub index: i64,
    pub required: bool,
    pub tooltip: String,
    pub type_bounds: String,
    pub has_default: bool,
    pub default_value: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OutputModel {
    pub parent_node_id: NodeId,
    pub command_id: NodeId,
    pub local_coords: Coords,
    pub label: String,
    pub index: i64,
    pub passthrough: bool,
    pub tooltip: String,
    pub type_bound: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BookmarkModel {
    pub name: String,
    pub nodes_ids: HashSet<NodeId>,
}

#[rid::model]
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum SolanaNet {
    Devnet,
    Testnet,
    Mainnet,
}

impl From<SolanaNet> for BackendSolanaNet {
    fn from(solana_net: SolanaNet) -> BackendSolanaNet {
        match solana_net {
            SolanaNet::Devnet => BackendSolanaNet::Devnet,
            SolanaNet::Testnet => BackendSolanaNet::Testnet,
            SolanaNet::Mainnet => BackendSolanaNet::Mainnet,
        }
    }
}

impl From<BackendSolanaNet> for SolanaNet {
    fn from(solana_net: BackendSolanaNet) -> SolanaNet {
        match solana_net {
            BackendSolanaNet::Devnet => SolanaNet::Devnet,
            BackendSolanaNet::Testnet => SolanaNet::Testnet,
            BackendSolanaNet::Mainnet => SolanaNet::Mainnet,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WidgetKind {
    Basic(BasicWidgetKind),
    Command(CommandConfig),
    Context(ContextConfig),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BasicWidgetKind {
    Block,
    TextInput,
    Dummy,
}

impl Model {
    pub fn new(db_path: String, log_path: String) -> Self {
        // database configuration
        let cfg = DbConfig { db_path };

        let db = DB::new(&cfg).unwrap();
        let db = Arc::new(db);

        // Get list of graphs in database

        let graph_list = block_on(db.execute(Action::Query(QueryKind::ListGraphs)))
            .unwrap()
            .into_node_list()
            .unwrap()
            .into_iter()
            .filter(|(_, properties)| properties.contains_key(FLOW_GRAPH_MARKER))
            .map(|(node_id, properties)| GraphEntry {
                id: node_id.to_string(),
                name: properties.get("name").unwrap().as_str().unwrap().to_owned(),
            })
            .collect::<Vec<_>>();

        // load graph id or create a new one

        let graph_id = match graph_list.len() {
            0 => {
                let properties = json!({
                    "name":&Self::random_name(),
                    FLOW_GRAPH_MARKER: true,
                });

                let properties = match properties {
                    JsonValue::Object(props) => props,
                    _ => unreachable!(),
                };

                let graph_id = block_on(db.execute(Action::CreateGraph(properties)))
                    .unwrap()
                    .as_id()
                    .unwrap();

                create_wallet_and_context(Db(db.clone()), GraphId(graph_id));

                graph_id
            }
            _ => Uuid::from_str(&graph_list.last().unwrap().id).unwrap(),
        };
        // let graph_id = match graph_list.len() {
        //     0 =>
        //     _ =>
        // };

        let graph_list = block_on(db.execute(Action::Query(QueryKind::ListGraphs)))
            .unwrap()
            .into_node_list()
            .unwrap()
            .into_iter()
            .filter(|(_, properties)| properties.contains_key(FLOW_GRAPH_MARKER))
            .map(|(node_id, properties)| GraphEntry {
                id: node_id.to_string(),
                name: properties.get("name").unwrap().as_str().unwrap().to_owned(),
            })
            .collect::<Vec<_>>();

        let context_node_id = NodeId(generate_uuid_v1());

        let run_status = Arc::new(DashMap::new());
        let req_id = Arc::new(Mutex::new(u64::default()));

        let graph_id = Arc::new(Mutex::new(GraphId(graph_id)));

        let graph_entry = graph_list
            .clone()
            .into_iter()
            .find(|entry| entry.id == graph_id.lock().unwrap().0.to_string())
            .unwrap();

        let mut model = Self {
            db: Db(db.clone()),
            graph_id: graph_id.clone(),
            context_node_id, // this will be replaced when we call read_graph
            nodes: HashMap::new(),
            flow_edges: HashMap::new(),
            node_edges: HashMap::new(),
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            graph_list,
            bookmarks: HashMap::new(),
            flow_context: FlowContext::new(
                db.clone(),
                run_status.clone(),
                req_id.clone(),
                graph_id.clone(),
                graph_entry,//TODO wrong, doesn't update
                log_path,
            ),
            run_status,
            req_id,
            solana_net: SolanaNet::Devnet,
        };

        model.read_graph(model.graph_id());

        assert_ne!(context_node_id, model.context_node_id);

        model
    }

    /// GETTERS
    ///
    ///

    /// DB
    ///
    pub fn db(&self) -> &Arc<DB> {
        &self.db.0
    }

    pub fn get_node(&self, node_id: &NodeId) -> Option<&NodeModel> {
        self.nodes.get(node_id)
    }

    pub fn nodes(&self) -> &HashMap<NodeId, NodeModel> {
        &self.nodes
    }

    pub fn inputs(&self) -> &HashMap<InputId, InputModel> {
        &self.inputs
    }

    pub fn outputs(&self) -> &HashMap<OutputId, OutputModel> {
        &self.outputs
    }

    pub fn flow_edges(&self) -> &HashMap<EdgeId, FlowEdgeModel> {
        &self.flow_edges
    }

    pub fn node_edges(&self) -> &HashMap<NodeEdgeId, NodeEdgeModel> {
        &self.node_edges
    }

    pub fn graph_id(&self) -> GraphId {
        let graph_id = self.graph_id.lock().unwrap();

        *graph_id
    }

    pub fn get_graph_entry(&self, graph_id: GraphId) -> GraphEntry {
        self.graph_list
            .iter()
            .find(|entry| entry.id == graph_id.0.to_string())
            .unwrap()
            .clone()
    }

    // pub fn save_req_id(&mut self, req_id: u64) {
    //     let mut props = Properties::new();

    //     props.insert(REQ_ID.into(), serde_json::to_value(req_id).unwrap());

    //     block_on(self.db.0.execute(Action::Mutate(
    //         self.graph_id().0,
    //         MutateKind::CreateNode(props),
    //     )))
    //     .unwrap();
    // }

    // TODO how to pass req_id to refresh UI?
    pub fn deploy(&self) {
        /*
        let graph = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadGraph(self.graph_id().0))),
        )
        .unwrap()
        .into_graph()
        .unwrap();

        println!("{:#?}", graph);
        */

        self.run_status.clear();
        block_on(self.flow_context.deploy(self.graph_id()));
    }

    pub fn undeploy(&self) {
        self.run_status.clear();
        block_on(self.flow_context.undeploy(self.graph_id()));

        // TODO: refresh ui
    }

    pub fn change_solana_net(&mut self, solana_net: SolanaNet) {
        let mut ctx_node = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadNode(self.context_node_id.0))),
        )
        .unwrap()
        .into_node()
        .unwrap();

        let mut ctx_cfg: solana::Config =
            serde_json::from_value(ctx_node.properties.remove(CTX_MARKER).unwrap()).unwrap();

        ctx_cfg.solana_net = solana_net.into();

        ctx_node.properties.insert(
            CTX_MARKER.to_owned(),
            serde_json::to_value(&ctx_cfg).unwrap(),
        );

        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::UpdateNode((ctx_node.node_id, ctx_node.properties)),
        )))
        .unwrap();

        self.solana_net = solana_net;
        println!("Solana Net changed to {:?}", self.solana_net);
    }

    pub fn export(&self, path: String, filename: String) {
        // get current graph entry
        //
        let graph = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadGraph(self.graph_id().0))),
        )
        .unwrap()
        .into_graph()
        .unwrap();

        let graph_id = self.graph_id().0.to_string();

        // let graph_name = self
        //     .graph_list
        //     .iter()
        //     .find(|entry| entry.id == graph_id)
        //     .map(|entry| entry.name.clone())
        //     .unwrap();

        // let timestamp = chrono::offset::Utc::now().timestamp_millis();
        // dbg!(path.clone());
        // dbg!(filename.clone());
        std::fs::write(
            format!("{}.json", path),
            serde_json::to_vec(&graph).unwrap(),
        )
        .unwrap();
    }

    pub fn import(&mut self, path: &str) {
        let bytes = std::fs::read(path).unwrap();
        let graph: Graph = serde_json::from_slice(&bytes).unwrap();

        let properties = json!({
            "name":&Self::random_name(),
            FLOW_GRAPH_MARKER: true,
        });

        let properties = match properties {
            JsonValue::Object(props) => props,
            _ => unreachable!(),
        };

        let graph_id = block_on(self.db.0.execute(Action::CreateGraph(properties)))
            .unwrap()
            .as_id()
            .unwrap();

        let mut node_id_map = HashMap::new();

        for node in graph
            .nodes
            .iter()
            .filter(|node| !node.properties.contains_key("_state_id_prop"))
        {
            let new_node_id = generate_uuid_v1();

            node_id_map.insert(node.node_id, new_node_id);

            if !node.properties.contains_key("BOOKMARKS") {
                block_on(self.db.0.execute(Action::Mutate(
                    graph_id,
                    MutateKind::CreateNodeWithId((new_node_id, node.properties.clone())),
                )))
                .unwrap();
            }
        }

        // update node ids in bookmarks
        for node in graph
            .nodes
            .iter()
            .filter(|node| node.properties.contains_key("BOOKMARKS"))
        {
            let mut properties = node.properties.clone();

            let bookmarks = properties
                .get_mut("BOOKMARKS")
                .unwrap()
                .as_array_mut()
                .unwrap();

            bookmarks.iter_mut().for_each(|bookmark| {
                let bookmark_id = bookmark.as_str().unwrap();
                *bookmark =
                    serde_json::to_value(node_id_map[&Uuid::from_str(bookmark_id).unwrap()])
                        .unwrap();
            });

            block_on(self.db.0.execute(Action::Mutate(
                graph_id,
                MutateKind::CreateNodeWithId((node_id_map[&node.node_id], properties)),
            )))
            .unwrap();
        }

        for edge in graph
            .nodes
            .into_iter()
            .filter(|node| !node.properties.contains_key("_state_id_prop"))
            .map(|node| node.outbound_edges.into_iter())
            .flatten()
        {
            let properties = graph.edges.get(&edge.id).unwrap().clone();
            //println!("{}", edge.to);

            block_on(self.db.0.execute(Action::Mutate(
                graph_id,
                MutateKind::CreateEdge(CreateEdge {
                    from: node_id_map[&edge.from],
                    to: node_id_map[&edge.to],
                    properties,
                }),
            )))
            .unwrap();
        }

        self.read_graph(GraphId(graph_id));
    }

    pub fn iter_widget_nodes(&self) -> impl Iterator<Item = (&NodeId, &WidgetNodeData)> {
        self.nodes.iter().filter_map(|(node_id, node)| match node {
            NodeModel::Widget(data) => Some((node_id, data)),
            //NodeModel::Data(_) => None,
        })
    }

    pub fn iter_node_edges(&self) -> impl Iterator<Item = (&NodeEdgeId, &NodeEdgeModel)> {
        self.node_edges.iter()
    }

    fn random_name() -> String {
        use bip39::{Language, Mnemonic, MnemonicType};

        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        let phrase = mnemonic.into_phrase();

        phrase.split(' ').next().unwrap().to_owned()
    }

    pub fn generate_seed() -> String {
        use bip39::{Language, Mnemonic, MnemonicType};

        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        let phrase = mnemonic.into_phrase();

        phrase
    }

    // pub fn validate_seed(phrase: String) -> String {
    //     use bip39::{Language, Mnemonic, MnemonicType};

    //     let validate = Mnemonic::validate(&phrase, Language::English);
    // }

    pub fn new_graph(&mut self) {
        self.run_status.clear();

        let properties = json!({
            "name": &Self::random_name(),
            FLOW_GRAPH_MARKER: true,
        });

        let properties = match properties {
            JsonValue::Object(props) => props,
            _ => unreachable!(),
        };

        let graph_id = block_on(self.db.0.execute(Action::CreateGraph(properties)))
            .unwrap()
            .as_id()
            .unwrap();

        create_wallet_and_context(self.db.clone(), GraphId(graph_id));

        self.read_graph(GraphId(graph_id));
    }

    pub fn read_graph(&mut self, graph_id: GraphId) {
        self.undeploy();
        self.run_status.clear();

        self.graph_list = block_on(self.db.0.execute(Action::Query(QueryKind::ListGraphs)))
            .unwrap()
            .into_node_list()
            .unwrap()
            .into_iter()
            .filter(|(_, properties)| properties.contains_key(FLOW_GRAPH_MARKER))
            .map(|(node_id, properties)| GraphEntry {
                id: node_id.to_string(),
                name: properties.get("name").unwrap().as_str().unwrap().to_owned(),
            })
            .collect::<Vec<_>>();

        // get graph, nodes, and edges
        let graph = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadGraph(graph_id.0))),
        )
        .unwrap()
        .into_graph()
        .unwrap();

        {
            let mut graph_id_ref = self.graph_id.lock().unwrap();

            *graph_id_ref = graph_id
        }


  

        self.bookmarks = HashMap::new();
        for node in graph.nodes.iter() {
            if let Some(bookmark_name) = node.properties.get(BOOKMARK_NAME) {
                let bookmark_name = serde_json::from_value(bookmark_name.clone()).unwrap();

                let bookmarked_node_ids = node.properties.get(BOOKMARKS).unwrap();
                let bookmarked_node_ids =
                    serde_json::from_value(bookmarked_node_ids.clone()).unwrap();

                self.bookmarks.insert(
                    BookmarkId(node.node_id),
                    BookmarkModel {
                        name: bookmark_name,
                        nodes_ids: bookmarked_node_ids,
                    },
                );
            }
        }
        // dbg!(self.bookmarks.clone());

        self.nodes = HashMap::new();

        let get_widget_kind = |properties: &Properties| {
            if let Some(command_config) = properties.get(COMMAND_MARKER) {
                let command_config = serde_json::from_value(command_config.clone()).unwrap();

                return Some(WidgetKind::Command(command_config));
            }

            if let Some(context_config) = properties.get(CTX_MARKER) {
                let context_config = serde_json::from_value(context_config.clone()).unwrap();

                return Some(WidgetKind::Context(context_config));
            }

            if properties.get(BLOCK_MARKER).is_some() {
                return Some(WidgetKind::Basic(BasicWidgetKind::Block));
            }
            if properties.get(TEXT_INPUT_MARKER).is_some() {
                return Some(WidgetKind::Basic(BasicWidgetKind::TextInput));
            }

            None
        };

        for node in graph.nodes.iter() {
            let kind = match get_widget_kind(&node.properties) {
                Some(WidgetKind::Context(ctx)) => {
                    self.context_node_id = NodeId(node.node_id);

                    self.solana_net = ctx.solana_net.into();

                    continue;
                }
                Some(widget_kind) => widget_kind,
                None => continue,
            };

            // dbg!(node.properties.clone());
            let coords = node.properties.get(COORDS_MARKER).unwrap();
            let coords = serde_json::from_value(coords.clone()).unwrap();
            let dimensions = node.properties.get(DIMENSIONS_MARKER).unwrap();
            let dimensions = serde_json::from_value(dimensions.clone()).unwrap();

            // let text = match kind {
            //     WidgetKind::Basic(_) => {}
            //     WidgetKind::Command(cfg) => match cfg {
            //         CommandConfig::Simple(cmd) => match cmd {
            //             SimpleCommand::Const(value) => {
            //                 let text = node.properties.get(TEXT_MARKER).unwrap(); //unwrap_or(&empty_text);
            //                 // let text = text.into();
            //                 let text = serde_json::from_value(text.clone()).unwrap();
            //                 text
            //             }
            //             SimpleCommand::Print => {}
            //             SimpleCommand::HttpRequest(_) => {}
            //             SimpleCommand::JsonExtract(_) => {}
            //         },
            //         CommandConfig::Solana(_) => {}
            //     },
            //     WidgetKind::Context(_) => {}
            // };
            //WidgetKind::Command(CommandConfig::Simple(SimpleCommandKind::Const)) {}
            let empty_text = JsonValue::String("".to_string());
            let text = node.properties.get(TEXT_MARKER).unwrap_or(&empty_text);
            let text = serde_json::from_value(text.clone()).unwrap();

            let additional_data = node
                .properties
                .get(ADDITIONAL_DATA_MARKER)
                .unwrap_or(&empty_text);
            let additional_data = serde_json::from_value(additional_data.clone()).unwrap();

            let command_name = node.properties.get(COMMAND_NAME_MARKER).unwrap();
            let command_name = match command_name {
                JsonValue::Null => None,
                JsonValue::String(name) => Some(name.clone()),
                JsonValue::Bool(_) => panic!(),
                JsonValue::Number(_) => panic!(),
                JsonValue::Array(_) => panic!(),
                JsonValue::Object(_) => panic!(),
            };

            self.nodes.insert(
                NodeId(node.node_id),
                NodeModel::Widget(WidgetNodeData {
                    kind,
                    command_name,
                    coords,
                    dimensions,
                    text,
                    additional_data,
                }),
            );
        }

        self.node_edges = HashMap::new();

        for node in graph.nodes.iter() {
            for edge in node.inbound_edges.iter() {
                let props = block_on(
                    self.db
                        .0
                        .execute(Action::Query(QueryKind::ReadEdgeProperties(*edge))),
                )
                .unwrap()
                .into_properties()
                .unwrap();

                if props.get(BLOCK_TO_CMD_EDGE_MARKER).is_none() {
                    // TODO update block to child
                    continue;
                }

                self.node_edges.insert(
                    NodeEdgeId(edge.id),
                    NodeEdgeModel {
                        from: NodeId(edge.from),
                        to: NodeId(edge.to),
                        data: EdgeModelData {
                            edge_type: EdgeType::Child,
                            from_coords: None,
                            to_coords: None,
                        },
                    },
                );
            }
        }

        self.inputs = HashMap::new();
        self.outputs = HashMap::new();

        // INPUT OUTPUT EDGES
        for (node_id, node) in self.nodes.iter() {
            let node = match &node {
                NodeModel::Widget(w) => w,
                _ => continue,
            };

            let width = node.dimensions.width;

            let cmd = match &node.kind {
                WidgetKind::Command(cmd) => cmd,
                _ => continue,
            };

            let command_name = node.command_name.as_ref().unwrap(); // because it is command

            //get parent coords
            let coords = Coords { x: 0.0, y: 0.0 };
            // dbg!(&node);
            let (_, edge) = self
                .node_edges
                .iter()
                .find(|(_, edge)| &edge.to == node_id)
                .unwrap();
            let block_id = edge.from;

            let (inputs, outputs) =
                Self::generate_ports(*node_id, command_name, cmd, (block_id, coords), width);

            for input in inputs {
                self.inputs.insert(InputId(generate_uuid_v1()), input);
            }

            for output in outputs {
                self.outputs.insert(OutputId(generate_uuid_v1()), output);
            }
        }

        self.flow_edges = HashMap::new();

        for (&node_id, node) in self.nodes.iter() {
            let node = match &node {
                NodeModel::Widget(w) => w,
                _ => continue,
            };

            // let from_cmd = match &node.kind {
            //     WidgetKind::Command(cmd) => cmd,
            //     _ => continue,
            // };

            for edge in graph
                .nodes
                .iter()
                .find(|node| node.node_id == node_id.0)
                .unwrap()
                .outbound_edges
                .iter()
            {
                let props = block_on(
                    self.db
                        .0
                        .execute(Action::Query(QueryKind::ReadEdgeProperties(*edge))),
                )
                .unwrap()
                .into_properties()
                .unwrap();

                if props.get(INPUT_ARG_NAME_MARKER).is_none() {
                    continue;
                }

                let (input_id, _) = self
                    .inputs
                    .iter()
                    .find(|(_input_id, input)| {
                        input.command_id.0 == edge.to
                            && input.label
                                == props.get(INPUT_ARG_NAME_MARKER).unwrap().as_str().unwrap()
                    })
                    .unwrap();

                let (output_id, _) = self
                    .outputs
                    .iter()
                    .find(|(_output_id, output)| {
                        // dbg!(output.label.clone());
                        // dbg!(props.get(OUTPUT_ARG_NAME_MARKER).unwrap().as_str().unwrap());

                        output.command_id.0 == edge.from
                            && output.label
                                == props.get(OUTPUT_ARG_NAME_MARKER).unwrap().as_str().unwrap()
                    })
                    .inspect(|(id, m)| {
                        // dbg!(m.label.clone());
                        // dbg!(props.get(OUTPUT_ARG_NAME_MARKER).unwrap().as_str().unwrap());

                        // println!("{:#?}", m.label)
                    })
                    .unwrap();

                self.flow_edges.insert(
                    EdgeId(edge.id),
                    FlowEdgeModel {
                        input_id: *input_id,
                        output_id: *output_id,
                        edge_type: EdgeType::Flow,
                        db_edge_id: EdgeId(edge.id),
                    },
                );
            }
        }
    }

    pub fn save_bookmark(&mut self, bookmark_id: BookmarkId, bookmark_model: BookmarkModel) {
        let mut props = Properties::new();
        props.insert(BOOKMARK_NAME.into(), JsonValue::String(bookmark_model.name));

        props.insert(
            BOOKMARKS.into(),
            serde_json::to_value(bookmark_model.nodes_ids).unwrap(),
        );
        println!("{:?}", props.clone());

        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::CreateNodeWithId((bookmark_id.0, props)),
        )))
        .unwrap();
    }

    pub fn update_bookmark_node_ids_in_db(
        &mut self,
        bookmark_id: BookmarkId,
        bookmark_model: BookmarkModel,
    ) {
        let mut props = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadNode(bookmark_id.0))),
        )
        .unwrap()
        .into_node()
        .unwrap()
        .properties;

        props.insert(
            BOOKMARKS.into(),
            serde_json::to_value(bookmark_model.nodes_ids).unwrap(),
        );

        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::UpdateNode((bookmark_id.0, props)),
        )))
        .unwrap();
    }

    pub fn delete_bookmark(&mut self, bookmark_id: BookmarkId) {
        self.bookmarks.remove(&bookmark_id);

        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::DeleteNode(bookmark_id.0),
        )))
        .unwrap();
    }

    // has both the node id of the block and of the command
    pub fn generate_ports(
        node_id: NodeId,
        command_name: &str,
        cfg: &CommandConfig,
        parent: (NodeId, Coords),
        width: i64,
    ) -> (Vec<InputModel>, Vec<OutputModel>) {
        const Y_INPUT_OFFSET: i64 = 30; // offset for block title
        const X_INPUT_OFFSET: i64 = 0;

        // get parent coordinates and add them

        let (parent_id, _coords) = parent;

        let commands_map = commands_map();
        let command = commands_map.get(command_name).unwrap();
        let inputs: Vec<(String, bool, String, String, bool, String)> = command
            .inputs()
            .iter()
            .map(|input| {
                let mut type_bounds = input.acceptable_types().into_iter().collect::<Vec<&str>>();
                type_bounds.sort_by_key(|&name| name.to_lowercase());

                (
                    input.name.to_owned(),
                    input.required.to_owned(),
                    input.tooltip.to_owned(),
                    type_bounds.join(", ").as_str().to_owned(),
                    input.has_default.to_owned(),
                    input.default_value.to_owned(),
                )
            })
            .collect();
        let outputs: Vec<(String, String, bool, String)> = command
            .outputs()
            .iter()
            .map(|output| {
                (
                    output.name.to_owned(),
                    output.r#type.to_owned(),
                    output.passthrough.to_owned(),
                    output.tooltip.to_owned(),
                )
            })
            .collect();

        // let required:bool = command.inputs().iter().map(|input| input.required).collect();
        // command.inputs().iter().map(|input| input.type_bounds)
        //label: &str, required: bool, tooltip: &str, port_type: &[TypeBound]
        let mut y = Y_INPUT_OFFSET - INPUT_OFFSET;
        let mut index = 0;
        let input = |input_data: (String, bool, String, String, bool, String)| {
            y += INPUT_OFFSET;
            index += 1;
            InputModel {
                index,
                parent_node_id: parent_id,
                local_coords: Coords {
                    x: X_INPUT_OFFSET as f64, // - INPUT_OFFSET,
                    y: y as f64,
                },
                label: input_data.0,
                command_id: node_id,
                required: input_data.1,
                tooltip: input_data.2,
                type_bounds: input_data.3,
                has_default: input_data.4,
                default_value: input_data.5,
            }
        };

        let mut y = Y_INPUT_OFFSET - INPUT_OFFSET;
        let mut index = 0;

        let output = |output_data: (String, String, bool, String)| {
            y += INPUT_OFFSET;
            index += 1;
            OutputModel {
                index,

                parent_node_id: parent_id,
                local_coords: Coords {
                    x: (width - INPUT_OFFSET) as f64,
                    y: y as f64,
                },
                label: output_data.0,
                command_id: node_id,
                type_bound: output_data.1,
                passthrough: output_data.2,
                tooltip: output_data.3,
            }
        };

        (
            inputs.into_iter().map(input).collect(),
            outputs.into_iter().map(output).collect(),
        )
    }

    pub fn set_node_text(&mut self, node_id: &NodeId, text: String) {
        // FIXME: should panic if invalid node_id used
        // if let Some(node) =
        self.nodes.get_mut(node_id).unwrap().data_mut().text = text;
        // let node = match node {
        //     NodeModel::Widget(node) => node,
        // };

        // update db
        // }
    }

    pub fn set_node_additional_data(&mut self, node_id: &NodeId, additional_data: String) {
        // FIXME: should panic if invalid node_id used
        self.nodes
            .get_mut(node_id)
            .unwrap()
            .data_mut()
            .additional_data = additional_data;

        // update db
    }

    pub fn save_text_to_db(&mut self, node_id: &NodeId, text: &str) {
        let mut props = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadNode(node_id.0))),
        )
        .unwrap()
        .into_node()
        .unwrap()
        .properties;

        // dbg!(props.clone());
        //update 'text' key
        props
            .insert(TEXT_MARKER.into(), JsonValue::String(text.to_string()))
            .unwrap();

        // dbg!(props.clone());

        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::UpdateNode((node_id.0, props)),
        )))
        .unwrap();
    }

    // pub fn update_command(&mut self, node_id: NodeId, input_id: InputId, text: &str) {
    //     let command = 1;

    //     match command {
    //         Command::CreateMetadataAccounts {
    //             match input_name {
    //                 "fee_payer" => {
    //                     command.fee_payer = Some(text.to_value().unwrap()),
    //                 },

    //             }

    //         }
    //     }
    // }

    pub fn update_const_in_db(&mut self, node_id: NodeId, text: &str) {
        // dbg!(text.clone());
        let cfg: SimpleCommand = match serde_json::from_str(text) {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("error while saving const: {:#?}", e);
                return;
            }
        };

        let cfg = CommandConfig::Simple(cfg);

        let mut props = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadNode(node_id.0))),
        )
        .unwrap()
        .into_node()
        .unwrap()
        .properties;

        props
            .insert(COMMAND_MARKER.into(), serde_json::to_value(&cfg).unwrap())
            .unwrap();

        //update 'text' key
        props
            .insert(TEXT_MARKER.into(), JsonValue::String(text.to_string()))
            .unwrap();

        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::UpdateNode((node_id.0, props)),
        )))
        .unwrap();
    }

    pub fn update_const_additional_in_db(&mut self, node_id: NodeId, additional_data: &str) {
        let mut props = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadNode(node_id.0))),
        )
        .unwrap()
        .into_node()
        .unwrap()
        .properties;

        //update  key
        props
            .insert(
                ADDITIONAL_DATA_MARKER.into(),
                JsonValue::String(additional_data.to_string()),
            )
            .unwrap();

        // dbg!(props.clone());

        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::UpdateNode((node_id.0, props)),
        )))
        .unwrap();
    }

    /*
         let input = InputModel {
                       node_id,
                       local_coords: Coords { x: 10, y: 40 },
                       label: "Input 2".to_owned(),
                   };
                   let id = InputId(generate_uuid_v1());
                   let prev = self.inputs.insert(id, input);
                   assert!(prev.is_none());

                   let output = OutputModel {
                       node_id,
                       local_coords: Coords { x: 40, y: 10 },
                       label: "Output 1".to_owned(),
                   };
    */

    /// CREATE NODE
    ///
    pub fn add_node(&mut self, node: NodeModel, parent: Option<(NodeId, Coords)>) -> NodeId {
        let data = match &node {
            NodeModel::Widget(data) => data,
        };
        let width = data.dimensions.width;

        let node_id = NodeId(generate_uuid_v1());
        match &node {
            NodeModel::Widget(widget_node_data) => {
                let mut props = match widget_node_data.kind.clone() {
                    WidgetKind::Command(config) => {
                        let command_name = widget_node_data.command_name.as_ref().unwrap();
                        let (parent_node_id, coords) = parent.unwrap();
                        let (inputs, outputs) = Self::generate_ports(
                            node_id,
                            &command_name,
                            &config,
                            (parent_node_id, coords),
                            width,
                        );
                        //
                        for input in inputs {
                            let id = InputId(generate_uuid_v1());
                            self.inputs.insert(id, input);
                        }
                        for output in outputs {
                            let id = OutputId(generate_uuid_v1());
                            self.outputs.insert(id, output);
                        }

                        // save to db
                        let mut props = Properties::new();

                        props.insert(COMMAND_MARKER.into(), serde_json::to_value(config).unwrap());
                        props.insert(
                            COMMAND_NAME_MARKER.into(),
                            JsonValue::String(command_name.clone()),
                        );
                        props.insert(START_NODE_MARKER.into(), JsonValue::Bool(true));
                        props.insert(TEXT_MARKER.into(), JsonValue::String(String::new()));
                        props.insert(
                            ADDITIONAL_DATA_MARKER.into(),
                            JsonValue::String(String::new()),
                        );

                        props
                    }
                    WidgetKind::Context(_) => todo!(), //TODO don't do this, new context created on start
                    WidgetKind::Basic(kind) => match kind {
                        BasicWidgetKind::Block => {
                            // save to db
                            let mut props = Properties::new();

                            props.insert(BLOCK_MARKER.into(), JsonValue::Bool(true));
                            props.insert(COMMAND_NAME_MARKER.into(), JsonValue::Null);
                            props
                        }
                        BasicWidgetKind::TextInput => {
                            // save to db
                            let mut props = Properties::new();

                            props.insert(TEXT_INPUT_MARKER.into(), JsonValue::Bool(true));
                            props.insert(TEXT_MARKER.into(), JsonValue::String(String::new()));
                            props.insert(COMMAND_NAME_MARKER.into(), JsonValue::Null);
                            props
                        }
                        BasicWidgetKind::Dummy => Properties::new(),
                    },
                };

                props.insert(
                    DIMENSIONS_MARKER.into(),
                    serde_json::to_value(widget_node_data.dimensions.clone()).unwrap(),
                );

                props.insert(
                    COORDS_MARKER.into(),
                    serde_json::to_value(widget_node_data.coords.clone()).unwrap(),
                );

                block_on(self.db.0.execute(Action::Mutate(
                    self.graph_id().0,
                    MutateKind::CreateNodeWithId((node_id.0, props)),
                )))
                .unwrap();

                match widget_node_data.kind.clone() {
                    WidgetKind::Command(c) => match c.kind() {
                        CommandKind::Solana(_) => {
                            let mut properties = Properties::new();

                            properties.insert(CTX_EDGE_MARKER.into(), JsonValue::Bool(true));

                            block_on(self.db.0.execute(Action::Mutate(
                                self.graph_id().0,
                                MutateKind::CreateEdge(CreateEdge {
                                    from: self.context_node_id.0,
                                    to: node_id.0,
                                    properties,
                                }),
                            )))
                            .unwrap();
                        }
                        _ => (),
                    },
                    _ => (),
                }
            } // NodeModel::Data(_) => {}
        };

        //
        // dbg!(self.inputs.clone());
        let prev = self.nodes.insert(node_id, node);
        assert!(prev.is_none());

        node_id
    }

    /// TODO fix delete graph on indra side
    pub fn delete_graph(&mut self, graph_id: GraphId) {
        //
        let mut node = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadNode(graph_id.0))),
        )
        .unwrap()
        .into_node()
        .unwrap();

        node.properties.remove(FLOW_GRAPH_MARKER.into()).unwrap();

        node.properties
            .insert("DELETED_GRAPH_MARKER".into(), JsonValue::Bool(true));

        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::UpdateNode((graph_id.0, node.properties)),
        )))
        .unwrap();

        // Update GRAPH LIST
        // self.graph_list.retain(|item| {
        //     dbg!(item);
        //     dbg!(graph_id);
        //     item.id != graph_id.0.to_string()
        // });
        // dbg!(self.graph_list.clone());
    }

    pub fn rename_graph(&mut self, graph_id: GraphId, new_name: String) {
        // remove from db
        let mut node = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadNode(graph_id.0))),
        )
        .unwrap()
        .into_node()
        .unwrap();

        node.properties
            .insert("name".into(), JsonValue::String(new_name.clone()));

        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::UpdateNode((graph_id.0, node.properties)),
        )))
        .unwrap();

        // update in from model

        {
            let graph_entry = self
                .graph_list
                .iter_mut()
                .find(|entry| entry.id == graph_id.0.to_string())
                .unwrap();

            *graph_entry = GraphEntry {
                id: graph_id.0.to_string(),
                name: new_name,
            };
        }
    }

    /// Remove node from model and db
    pub fn remove_node(&mut self, node_id: NodeId) {
        // Block > node_edge > Widget >
        // if command, remove Flow edges
        // Graph > node_edge > Block
        // bookmarks

        // bookmarks
        let mut bookmarks_to_update = HashMap::new();
        let mut bookmarks_to_remove = Vec::new();

        // remove from bookmarks in model
        let is_in_bookmark = self
            .bookmarks
            .borrow_mut()
            .into_iter()
            .filter(|(_, bookmark_model)| bookmark_model.nodes_ids.contains(&node_id));

        // update in model
        for (bookmark_id, bookmark_model) in is_in_bookmark {
            match bookmark_model.nodes_ids.len() {
                0 => panic!(),
                // self is the only node, remove whole bookmark
                1 => {
                    bookmarks_to_remove.push(bookmark_id.clone());
                }
                // other nodes are bookmarked, only remove self
                _ => {
                    bookmark_model.nodes_ids.remove(&node_id);
                    bookmarks_to_update.insert(bookmark_id.clone(), bookmark_model.clone());
                }
            }
        }

        // update in database
        bookmarks_to_update
            .into_iter()
            .for_each(|(bookmark_id, bookmark_model)| {
                self.update_bookmark_node_ids_in_db(bookmark_id, bookmark_model);
            });

        // remove from model and database
        bookmarks_to_remove.into_iter().for_each(|bookmark_id| {
            self.delete_bookmark(bookmark_id);
        });

        // nodes and edges
        let mut node_edges_to_remove = Vec::new();
        let mut flow_edges_to_remove = HashSet::new();
        let mut inputs_to_remove = Vec::new();
        let mut outputs_to_remove = Vec::new();
        let mut children_to_remove = Vec::new();
        //
        for (edge_id, edge) in self.node_edges.iter() {
            if edge.from != node_id {
                continue;
            }

            let child_id = edge.to;
            let child = self.nodes.get(&edge.to).unwrap();

            children_to_remove.push(edge.to);

            match child {
                NodeModel::Widget(widget) => match widget.kind {
                    WidgetKind::Basic(BasicWidgetKind::Block) => unreachable!(),
                    WidgetKind::Basic(BasicWidgetKind::TextInput) => (),
                    WidgetKind::Basic(BasicWidgetKind::Dummy) => unreachable!(),
                    WidgetKind::Command(_) => {
                        for (input_id, input) in self.inputs.iter() {
                            if input.command_id != child_id {
                                continue;
                            }

                            for (flow_edge_id, flow_edge) in self.flow_edges.iter() {
                                if flow_edge.input_id == *input_id {
                                    flow_edges_to_remove.insert(*flow_edge_id);
                                }
                            }

                            inputs_to_remove.push(*input_id);
                        }

                        for (output_id, output) in self.outputs.iter() {
                            if output.command_id != child_id {
                                continue;
                            }

                            for (flow_edge_id, flow_edge) in self.flow_edges.iter() {
                                if flow_edge.output_id == *output_id {
                                    flow_edges_to_remove.insert(*flow_edge_id);
                                }
                            }

                            outputs_to_remove.push(*output_id);
                        }
                    }
                    WidgetKind::Context(_) => unreachable!(),
                },
            }

            node_edges_to_remove.push(*edge_id);
        }

        // println!("Deleting node:{:#?}", node_id);
        // println!("Deleting node_edges:{:#?}", node_edges_to_remove);
        // println!("Deleting flow_edges:{:#?}", flow_edges_to_remove);
        // println!("Deleting inputs:{:#?}", inputs_to_remove);
        // println!("Deleting outputs:{:#?}", outputs_to_remove);
        // println!("Deleting children nodes:{:#?}", children_to_remove);

        for flow_edge_id in flow_edges_to_remove {
            self.flow_edges.remove(&flow_edge_id).unwrap();
        }

        for node_edge_id in node_edges_to_remove {
            self.node_edges.remove(&node_edge_id).unwrap();
        }

        for input_id in inputs_to_remove {
            self.inputs.remove(&input_id).unwrap();
        }

        for output_id in outputs_to_remove {
            self.outputs.remove(&output_id).unwrap();
        }

        for child_id in children_to_remove {
            block_on(self.db.0.execute(Action::Mutate(
                self.graph_id().0,
                MutateKind::DeleteNode(child_id.0),
            )))
            .unwrap();
            self.nodes.remove(&child_id).unwrap();
        }

        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::DeleteNode(node_id.0),
        )))
        .unwrap();

        self.nodes.remove(&node_id).unwrap();
    }

    // ADD INPUT OUTPUT EDGE
    // TODO when creating edge, toggle start marker
    pub fn add_or_remove_flow_edge(
        &mut self,
        input_id: InputId,
        output_id: OutputId,
    ) -> Option<EdgeId> {
        for (&edge_id, edge) in &self.flow_edges {
            if edge.input_id == input_id && edge.output_id == output_id {
                block_on(self.db.0.execute(Action::Mutate(
                    self.graph_id().0,
                    MutateKind::DeleteEdge(Edge {
                        id: edge.db_edge_id.0,
                        from: self.outputs.get(&edge.output_id).unwrap().command_id.0,
                        to: self.inputs.get(&edge.input_id).unwrap().command_id.0,
                    }),
                )))
                .unwrap();

                self.flow_edges.remove(&edge_id);
                return None;
            }
        }

        let input_model = self.inputs.get(&input_id).unwrap();
        let output_model = self.outputs.get(&output_id).unwrap();

        let mut properties = serde_json::Map::new();

        properties.insert(
            INPUT_ARG_NAME_MARKER.into(),
            serde_json::to_value(&input_model.label).unwrap(),
        );
        properties.insert(
            OUTPUT_ARG_NAME_MARKER.into(),
            serde_json::to_value(&output_model.label).unwrap(),
        );
        // dbg!(properties.clone());

        let edge_id = block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::CreateEdge(CreateEdge {
                from: output_model.command_id.0,
                to: input_model.command_id.0,
                properties,
            }),
        )))
        .unwrap()
        .as_id()
        .unwrap();

        let mut props = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadNode(input_model.command_id.0))),
        )
        .unwrap()
        .into_node()
        .unwrap()
        .properties;

        props.remove(START_NODE_MARKER);

        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::UpdateNode((input_model.command_id.0, props)),
        )))
        .unwrap();

        let edge_id = EdgeId(edge_id);

        // Save to model
        let prev = self.flow_edges.insert(
            edge_id,
            FlowEdgeModel {
                input_id,
                output_id,
                edge_type: EdgeType::Flow,
                db_edge_id: EdgeId(edge_id.0),
            },
        );
        assert!(prev.is_none());

        Some(edge_id)
    }

    /// CREATE NODE EDGE
    /// inserts to db and model
    ///
    pub fn add_node_edge(&mut self, edge: NodeEdgeModel) -> NodeEdgeId {
        let mut properties = Properties::new();

        properties.insert(BLOCK_TO_CMD_EDGE_MARKER.into(), JsonValue::Bool(true));
        // dbg!(edge.clone());
        let edge_id = block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::CreateEdge(CreateEdge {
                from: edge.from.0,
                to: edge.to.0,
                properties,
            }),
        )))
        .unwrap()
        .as_id()
        .unwrap();

        let edge_id = NodeEdgeId(edge_id);

        let prev = self.node_edges.insert(edge_id, edge);
        assert!(prev.is_none());

        edge_id
    }

    /// UPDATE NODE
    ///
    pub fn set_node_coords(&mut self, node_id: &NodeId, coords: Coords) {
        let node = self.nodes.get_mut(node_id).unwrap();

        match node {
            NodeModel::Widget(ref mut data) => {
                data.coords = coords.clone();
            } // NodeModel::Data(_) => panic!(),
        };

        let mut properties = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadNode(node_id.0))),
        )
        .unwrap()
        .into_node()
        .unwrap()
        .properties;

        properties.insert(COORDS_MARKER.into(), serde_json::to_value(&coords).unwrap());

        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::UpdateNode((node_id.0, properties)),
        )))
        .unwrap();
    }

    /// UPDATE NODE DIMENSIONS
    /// some test /command more text
    ///
    pub fn set_node_dimensions(&mut self, node_id: &NodeId, dimensions: NodeDimensions) {
        let node = self.nodes.get_mut(node_id).unwrap();

        match node {
            NodeModel::Widget(ref mut data) => {
                data.dimensions = dimensions.clone();
            } // NodeModel::Data(_) => panic!(),
        };

        let mut properties = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadNode(node_id.0))),
        )
        .unwrap()
        .into_node()
        .unwrap()
        .properties;

        properties.insert(
            DIMENSIONS_MARKER.into(),
            serde_json::to_value(&dimensions).unwrap(),
        );

        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::UpdateNode((node_id.0, properties)),
        )))
        .unwrap();

        //get block's outputs, and change their width
        let outputs = self
            .outputs
            .borrow_mut()
            .into_iter()
            .filter(|(_id, output_model)| output_model.parent_node_id == *node_id);

        for (_, output_model) in outputs {
            *output_model = OutputModel {
                parent_node_id: output_model.parent_node_id,
                command_id: output_model.command_id,
                local_coords: Coords {
                    x: (dimensions.width - INPUT_OFFSET) as f64,
                    y: output_model.local_coords.y,
                },
                label: output_model.label.clone(),
                index: output_model.index,
                passthrough: output_model.passthrough,
                tooltip: output_model.tooltip.to_owned(),
                type_bound: output_model.type_bound.to_owned(),
            }
        }
    }

    /// CREATE WIDGET NODE and DATA NODE
    ///
    pub fn create_node(
        &mut self,
        widget_node_data: WidgetNodeData,
        parent: Option<(NodeId, Coords)>,
    ) -> NodeId {
        let node_id = match parent {
            Some((parent_node_id, coords)) => {
                let node = NodeModel::Widget(widget_node_data.clone());
                let widget_node_id = self.add_node(node, Some((parent_node_id, coords)));

                widget_node_id
            }
            None => {
                let node = NodeModel::Widget(widget_node_data.clone());
                let widget_node_id = self.add_node(node, None);

                widget_node_id
            }
        };

        node_id
    }

    //
    pub fn add_child_edge(
        &mut self,
        parent_widget_node_id: NodeId,
        child_widget_node_id: NodeId,
    ) -> NodeEdgeId {
        let widget_edge_id = self.add_node_edge(NodeEdgeModel {
            from: parent_widget_node_id,
            to: child_widget_node_id,
            data: EdgeModelData {
                edge_type: EdgeType::Child,
                from_coords: None,
                to_coords: None,
            },
        });
        widget_edge_id
    }

    pub fn next_input_at<'a>(
        &'a self,
        coords: &'a Coords,
    ) -> Option<(&'a InputId, &'a InputModel)> {
        self.inputs_at(coords).next()
    }

    pub fn next_output_at<'a>(
        &'a self,
        coords: &'a Coords,
    ) -> Option<(&'a OutputId, &'a OutputModel)> {
        self.outputs_at(coords).next()
    }

    pub fn next_input_or_output_at<'a>(&'a self, coords: &'a Coords) -> Option<PortId> {
        self.next_input_at(coords)
            .map(|(input_id, _)| PortId::Input(*input_id))
            .or_else(|| {
                self.next_output_at(coords)
                    .map(|(output_id, _)| PortId::Output(*output_id))
            })
    }

    pub fn next_movable_widget_node_at<'a>(
        &'a self,
        coords: &'a Coords,
    ) -> Option<(&'a NodeId, &'a WidgetNodeData)> {
        self.movable_widget_nodes_at(coords).next()
    }

    pub fn movable_widget_nodes_at<'a>(
        &'a self,
        coords: &'a Coords,
    ) -> impl Iterator<Item = (&'a NodeId, &'a WidgetNodeData)> {
        self.iter_widget_nodes().filter(|(_node_id, node)| {
            //dbg!(node);
            match node.kind {
                WidgetKind::Basic(BasicWidgetKind::Block) => (),
                _ => return false,
            }

            let rect = Rect {
                x1: node.coords.x as i64,
                x2: node.coords.x as i64 + node.dimensions.width,
                y1: node.coords.y as i64,
                y2: node.coords.y as i64 + node.dimensions.height,
            };

            rect.contains(coords.x as i64, coords.y as i64)
        })
    }

    // TODO not used, remove or keep?
    // pub fn command_at<'a>(&'a self, coords: &'a Coords) -> NodeId {
    //     *self
    //         .iter_widget_nodes()
    //         .filter(|(_node_id, node)| {
    //             match node.kind {
    //                 WidgetKind::Command(_) => (),
    //                 _ => return false,
    //             }

    //             let rect = Rect {
    //                 x1: node.coords.x,
    //                 x2: node.coords.x + node.dimensions.width,
    //                 y1: node.coords.y,
    //                 y2: node.coords.y + node.dimensions.height,
    //             };

    //             rect.contains(coords.x, coords.y)
    //         })
    //         .next()
    //         .unwrap()
    //         .0
    // }

    pub fn inputs_at<'a>(
        &'a self,
        coords: &'a Coords,
    ) -> impl Iterator<Item = (&'a InputId, &'a InputModel)> {
        self.inputs.iter().filter(|(_, input)| {
            let node = self.nodes.get(&input.parent_node_id).unwrap();
            let data = match node {
                NodeModel::Widget(data) => data,
                // NodeModel::Data(_) => panic!(),
            };
            let rect = Rect {
                x1: (data.coords.x + input.local_coords.x) as i64,
                x2: (data.coords.x + input.local_coords.x) as i64 + INPUT_SIZE,
                y1: (data.coords.y + input.local_coords.y) as i64,
                y2: (data.coords.y + input.local_coords.y) as i64 + INPUT_SIZE,
            };
            // dbg!(rect.clone());
            rect.contains(coords.x as i64, coords.y as i64)
        })
    }

    pub fn outputs_at<'a>(
        &'a self,
        coords: &'a Coords,
    ) -> impl Iterator<Item = (&'a OutputId, &'a OutputModel)> {
        self.outputs.iter().filter(|(_, output)| {
            let node = self.nodes.get(&output.parent_node_id).unwrap();
            let data = match node {
                NodeModel::Widget(data) => data,
                // NodeModel::Data(_) => panic!(),
            };
            let rect = Rect {
                x1: (data.coords.x + output.local_coords.x) as i64,
                x2: (data.coords.x + output.local_coords.x) as i64 + INPUT_SIZE,
                y1: (data.coords.y + output.local_coords.y) as i64,
                y2: (data.coords.y + output.local_coords.y) as i64 + INPUT_SIZE,
            };
            // dbg!(rect.clone());
            rect.contains(coords.x as i64, coords.y as i64)
        })
    }

    /// TEMPLATE
    /// block
    ///     text_input
    /// Given coordinates, create a starting block node nested with a text_input field node
    ///
    pub fn create_starting_node_block(&mut self, coords: Coords) -> NodeId {
        // block properties
        let block_widget = WidgetNodeData::new_block(coords);

        // child properties

        let child_widget = WidgetNodeData::new_text_input();

        // create widget
        let block_node_id = self.create_node(block_widget, None);

        // create child
        let child_node_id = self.create_node(child_widget, None);

        // connect with child edge
        self.add_child_edge(block_node_id, child_node_id);

        block_node_id
    }

    /// TEMPLATE
    ///
    /// block
    ///     command
    ///
    /// add command widget, remove text_input
    ///
    pub fn into_command_block(
        &mut self,
        block_id: NodeId,
        coords: Coords,
        command_name: &str,
        kind: WidgetKind,
        dimensions: NodeDimensions,
    ) {
        // let dimensions = kind.default_widget_dimensions();
        let command_widget_node_data =
            WidgetNodeData::new_command(command_name, kind, dimensions.width, dimensions.height);

        // update block dimensions
        self.set_node_dimensions(&block_id, command_widget_node_data.clone().dimensions);

        // get edge between block and text input
        // get child of block widget
        let (_, node_edge_model) = self
            .iter_node_edges()
            .find(|(_n, node_edge_model)| {
                node_edge_model.from == block_id
                    && node_edge_model.data.edge_type == EdgeType::Child
            })
            .unwrap();

        // get text input node id
        let text_input_id = node_edge_model.to;

        // create command node
        let command_node_id = self.create_node(command_widget_node_data, Some((block_id, coords)));

        // connect block widget to command widget
        self.add_node_edge(NodeEdgeModel {
            from: block_id,
            to: command_node_id,
            data: EdgeModelData {
                edge_type: EdgeType::Child,
                from_coords: None,
                to_coords: None,
            },
        });

        // Remove text input edge and node
        self.remove_node_and_edges(text_input_id); //FIXME edge properties return null when queried to be removed
    }

    pub fn remove_node_and_edges(&mut self, node_id: NodeId) {
        // from db
        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::DeleteNode(node_id.0),
        )))
        .unwrap();

        //from ui
        let removed_node = self.nodes.remove_entry(&node_id).unwrap();

        //check outbound edges
        // let outbound_edges: Vec<(&NodeEdgeId, &NodeEdgeModel)> =
        let edges: Vec<(NodeEdgeId, NodeEdgeModel)> = self
            .node_edges
            .clone()
            .into_iter()
            .filter(|(_edge, edge_model)| edge_model.to == removed_node.0)
            .collect();

        for edge in edges {
            self.node_edges.remove_entry(&edge.0);
        }
    }
}

// Create wallet graph and context node
pub fn create_wallet_and_context(db: Db, graph_id: GraphId) -> ContextConfig {
    // create wallet graph
    let wallet_graph_id = block_on(db.0.execute(Action::CreateGraph(Default::default())))
        .unwrap()
        .as_id()
        .unwrap();

    let solana_context_config = solana::Config {
        solana_net: BackendSolanaNet::Devnet,
        wallet_graph: wallet_graph_id,
    };

    // create context node
    let mut props = serde_json::Map::new();

    props.insert(
        CTX_MARKER.into(),
        serde_json::to_value(&solana_context_config).unwrap(),
    );

    let solana_ctx_node_id =
        block_on(db.0.execute(Action::Mutate(graph_id.0, MutateKind::CreateNode(props))))
            .unwrap()
            .as_id()
            .unwrap();

    solana_context_config
}

impl NodeModel {
    pub fn data(&self) -> &WidgetNodeData {
        match self {
            Self::Widget(data) => data,
        }
    }

    pub fn data_mut(&mut self) -> &mut WidgetNodeData {
        match self {
            Self::Widget(data) => data,
        }
    }
}
