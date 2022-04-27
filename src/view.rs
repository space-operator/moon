use std::collections::HashMap;

use crate::command::*;
use crate::model::{GraphEntry, SolanaNet};

// #[derive(rid::Config)]
#[rid::model]
#[derive(Clone, Debug)]
#[rid::structs(
    NodeView,
    Camera,
    Selection,
    Command,
    WidgetTextCommand,
    EdgeView,
    GraphEntry,
    BookmarkView,
    DebugData
)]
#[rid::enums(SolanaNet)]
pub struct View {
    pub graph_entry: GraphEntry,
    pub nodes: HashMap<String, NodeView>,
    pub flow_edges: HashMap<String, EdgeView>,
    pub selected_node_ids: Vec<String>,
    pub selected_command_ids: Vec<String>,
    pub selection: Selection, // TODO Implement
    pub command: Command,     // not used
    pub text_commands: Vec<WidgetTextCommand>,
    pub graph_list: Vec<GraphEntry>,
    pub highlighted: Vec<String>,
    pub transform: Camera,
    pub transform_screenshot: Camera,
    pub bookmarks: HashMap<String, BookmarkView>,
    pub solana_net: SolanaNet,
    pub ui_state_debug: DebugData,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[rid::model]
pub struct DebugData {
    pub ui_state: String,
    pub mapping_kind: String,
    pub selected_node_ids: String,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[rid::model]
pub struct Ratio {
    pub numer: i64,
    pub denom: u64,
}

#[derive(Clone, Debug, Default)]
#[rid::model]
#[rid::structs(NodeChange)]
pub struct LastViewChanges {
    pub changed_nodes_ids: HashMap<String, NodeChange>, /*NodeChangeKind*/
    pub changed_flow_edges_ids: Vec<String>,
    pub is_selected_node_ids_changed: bool,
    pub is_selection_changed: bool,
    pub is_command_changed: bool,
    pub is_text_commands_changed: bool,
    pub is_graph_list_changed: bool,
    pub is_highlighted_changed: bool,
    pub is_transform_changed: bool,
    pub is_transform_screenshot_changed: bool,
    pub is_graph_changed: bool,
    pub is_bookmark_changed: bool,
}

impl From<i64> for Ratio {
    fn from(numer: i64) -> Self {
        Self { numer, denom: 1 }
    }
}

impl From<f64> for Ratio {
    fn from(value: f64) -> Self {
        Self {
            numer: (value * 4294967296.0) as i64,
            denom: 4294967296,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
#[rid::model]
#[rid::structs(Ratio)]
pub struct Camera {
    pub x: Ratio,
    pub y: Ratio,
    pub scale: Ratio,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            x: Ratio::from(0),
            y: Ratio::from(0),
            scale: Ratio::from(1),
        }
    }
}

#[rid::model]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[rid::enums(NodeChangeKind)]
pub struct NodeChange {
    pub kind: NodeChangeKind,
}

#[rid::model]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum NodeChangeKind {
    Added,
    Removed,
    Modified,
    AddedOrModified, // temporary
}

pub trait CommandView: crate::command::Command {
    const VIEW_TYPE: NodeViewType;
}

pub trait DynCommandView: DynCommand + std::fmt::Debug {
    fn view_type(&self) -> NodeViewType;
}

impl<T: CommandView + std::fmt::Debug> DynCommandView for T {
    fn view_type(&self) -> NodeViewType {
        T::VIEW_TYPE
    }
}

impl CommandView for PrintCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::Print;
}

impl CommandView for ConstCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::Const;
}

impl CommandView for JsonExtractCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::JsonExtract;
}
impl CommandView for JsonInsertCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::JsonInsert;
}
impl CommandView for HttpRequestCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::HttpRequest;
}
impl CommandView for IpfsUploadCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::IpfsUpload;
}

impl CommandView for IpfsNftUploadCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::IpfsNftUpload;
}
impl CommandView for BranchCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::Branch;
}

impl CommandView for WaitCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::Wait;
}
impl CommandView for CreateMintAccountCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::CreateMintAccount;
}
// impl CommandView for AddPubkeyCommand {
//     const VIEW_TYPE: NodeViewType = NodeViewType::AddPubkey;
// }
impl CommandView for CreateTokenAccountCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::CreateTokenAccount;
}
impl CommandView for GenerateKeypairCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::GenerateKeypair;
}
impl CommandView for MintTokenCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::MintToken;
}
impl CommandView for TransferTokenCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::TransferToken;
}

impl CommandView for TransferSolanaCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::TransferSolana;
}
impl CommandView for RequestAirdropCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::RequestAirdrop;
}
impl CommandView for GetBalanceCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::GetBalance;
}
impl CommandView for CreateMetadataAccountsCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::CreateMetadataAccounts;
}
impl CommandView for CreateMasterEditionCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::CreateMasterEdition;
}
impl CommandView for UpdateMetadataAccountsCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::UpdateMetadataAccounts;
}

impl CommandView for VerifyCollectionCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::VerifyCollection;
}
impl CommandView for ApproveCollectionAuthorityCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::ApproveCollectionAuthority;
}
impl CommandView for SignMetadataCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::SignMetadata;
}
impl CommandView for UtilizeCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::Utilize;
}
impl CommandView for ApproveUseAuthorityCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::ApproveUseAuthority;
}
impl CommandView for GetLeftUsesCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::GetLeftUses;
}
// impl CommandView for ArweaveUploadCommand {
//     const VIEW_TYPE: NodeViewType = NodeViewType::ArweaveUpload;
// }
// impl CommandView for ArweaveNftUploadCommand {
//     const VIEW_TYPE: NodeViewType = NodeViewType::ArweaveNftUpload;
// }
impl CommandView for ArweaveNftUploadCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::ArweaveNftUpload;
}

impl CommandView for ArweaveFileUploadCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::ArweaveFileUpload;
}

// TODO: list all commands
pub const VIEW_COMMANDS: &'static [&'static dyn DynCommandView] = &[
    &PrintCommand,
    &ConstCommand,
    &JsonExtractCommand,
    &JsonInsertCommand,
    &HttpRequestCommand,
    &IpfsUploadCommand,
    &IpfsNftUploadCommand,
    &WaitCommand,
    &BranchCommand,
    // Solana
    &CreateMintAccountCommand,
    // &AddPubkeyCommand,
    &CreateTokenAccountCommand,
    &GenerateKeypairCommand,
    &MintTokenCommand,
    &TransferTokenCommand,
    &TransferSolanaCommand,
    &RequestAirdropCommand,
    &GetBalanceCommand,
    // NFTs
    &CreateMetadataAccountsCommand,
    &CreateMasterEditionCommand,
    &UpdateMetadataAccountsCommand,
    &VerifyCollectionCommand,
    &ApproveCollectionAuthorityCommand,
    &SignMetadataCommand,
    &UtilizeCommand,
    &ApproveUseAuthorityCommand,
    &GetLeftUsesCommand,
    // &ArweaveUploadCommand,
    // &ArweaveNftUploadCommand,
    &ArweaveNftUploadCommand,
    &ArweaveFileUploadCommand,
];

// TODO: Build once on initialization
pub fn commands_view_map() -> HashMap<&'static str, &'static dyn DynCommandView> {
    VIEW_COMMANDS
        .iter()
        .map(|&command| (command.command_name(), command))
        .collect()
}

#[test]
fn commands_equal_view_commands() {
    assert_eq!(COMMANDS.len(), VIEW_COMMANDS.len());
    assert!(COMMANDS
        .iter()
        .zip(VIEW_COMMANDS.iter())
        .all(|(command, view_command)| { command.command_name() == view_command.command_name() }));
}

pub fn generate_default_text_commands() -> Vec<WidgetTextCommand> {
    COMMANDS
        .iter()
        .map(|command| WidgetTextCommand {
            command_name: command.command_name().to_owned(),
            widget_name: command.widget_name().to_owned(),
            inputs: command
                .inputs()
                .iter()
                .map(|input| TextCommandInput {
                    name: input.name.to_owned(),
                    acceptable_kinds: input
                        .acceptable_types()
                        .iter()
                        .map(|&value| value.to_owned())
                        .collect(),
                    required: input.required.to_owned(),
                    tooltip: input.tooltip.to_owned(),
                    has_default: input.has_default.to_owned(),
                    default_value: input.default_value.to_owned(),
                })
                .collect(),
            outputs: command
                .outputs()
                .iter()
                .map(|output| TextCommandOutput {
                    name: output.name.to_owned(),
                    kind: output.r#type.to_owned(),
                    passthrough: output.passthrough.to_owned(),
                    tooltip: output.tooltip.to_owned(),
                })
                .collect(),
            description: command.description().to_owned(),
            availability: command
                .availability()
                .iter()
                .map(|solana_net| match solana_net {
                    SolanaNet::Devnet => "devnet".to_string(),
                    SolanaNet::Testnet => "testnet".to_string(),
                    SolanaNet::Mainnet => "mainnet".to_string(),
                })
                .collect(),
        })
        .collect()
}

impl Default for View {
    fn default() -> Self {
        Self {
            graph_entry: GraphEntry::default(),
            nodes: HashMap::default(),
            flow_edges: HashMap::default(),
            selected_node_ids: Vec::default(),
            selected_command_ids: Vec::default(),
            selection: Selection::default(),
            command: Default::default(),
            text_commands: generate_default_text_commands(),
            graph_list: Vec::default(),
            highlighted: Vec::default(),
            transform: Camera::default(),
            transform_screenshot: Camera::default(),
            bookmarks: HashMap::default(),
            solana_net: SolanaNet::Devnet,
            ui_state_debug: DebugData::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
#[rid::model]
pub struct Selection {
    pub is_active: bool,
    pub x1: i64,
    pub y1: i64,
    pub x2: i64,
    pub y2: i64,
}

// not used
#[derive(Debug, Default, Clone, Eq, PartialEq)]
#[rid::model]
pub struct Command {
    pub is_active: bool,
    pub command: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[rid::model]
#[rid::structs(TextCommandInput, TextCommandOutput)]
pub struct WidgetTextCommand {
    pub command_name: String,
    pub widget_name: String,
    pub description: String,
    pub inputs: Vec<TextCommandInput>,
    pub outputs: Vec<TextCommandOutput>,
    pub availability: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[rid::model]
pub struct TextCommandInput {
    pub name: String,
    pub acceptable_kinds: Vec<String>,
    pub required: bool,
    pub tooltip: String,
    pub has_default: bool,
    pub default_value: String,
}

#[derive(rid::Config, Debug, Clone, Eq, PartialEq)]
#[rid::model]
pub struct TextCommandOutput {
    pub name: String,
    pub kind: String,
    pub passthrough: bool,
    pub tooltip: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[rid::model]
#[rid::structs(EdgeView)]
#[rid::enums(NodeViewType, RunStateView)]
pub struct NodeView {
    pub index: i64, // only for input output nodes
    pub parent_id: String,
    pub origin_x: i64, // position of node before movements or after moved
    pub origin_y: i64,
    pub x: i64,
    pub y: i64,
    pub height: i64,
    pub width: i64,
    pub text: String,
    pub outbound_edges: HashMap<String, EdgeView>, // not include flow edges
    pub widget_type: NodeViewType,                 // FIXME, pub NodeType
    pub flow_inbound_edges: Vec<String>,
    pub flow_outbound_edges: Vec<String>,
    pub run_state: RunStateView,
    pub elapsed_time: u64,
    pub error: String,
    pub print_output: String,
    pub additional_data: String,
    pub required: bool,
    pub tooltip: String,
    pub type_bounds: String,
    pub passthrough: bool,
    pub default_value: String,
    pub has_default: bool,
}

#[rid::model]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RunStateView {
    WaitingInputs,
    Running,
    Failed,
    Success,
    NotRunning,
    Canceled,
}

#[rid::model]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NodeViewType {
    Data,
    WidgetBlock,
    WidgetTextInput,
    DummyEdgeHandle,
    WidgetInput,
    WidgetOutput,
    //
    Print,
    Const,
    JsonExtract,
    JsonInsert,
    HttpRequest,
    IpfsUpload,
    IpfsNftUpload,
    Wait,
    Branch,
    //
    CreateMintAccount,
    // AddPubkey,
    CreateTokenAccount,
    GenerateKeypair,
    MintToken,
    TransferToken,
    TransferSolana,
    RequestAirdrop,
    GetBalance,
    //
    CreateMetadataAccounts,
    CreateMasterEdition,
    UpdateMetadataAccounts,
    VerifyCollection,
    ApproveCollectionAuthority,
    SignMetadata,
    Utilize,
    ApproveUseAuthority,
    GetLeftUses,
    // ArweaveUpload,
    // ArweaveNftUpload,
    ArweaveNftUpload,
    ArweaveFileUpload,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[rid::model]
#[rid::enums(ViewEdgeType)]
pub struct EdgeView {
    pub from: String,
    pub to: String,
    pub edge_type: ViewEdgeType,
    pub from_coords_x: i64,
    pub from_coords_y: i64,
    pub to_coords_x: i64,
    pub to_coords_y: i64,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[rid::model]
pub enum ViewEdgeType {
    Child,
    Data,
    Flow,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[rid::model]
pub struct BookmarkView {
    pub name: String,
    pub nodes: Vec<String>,
}
