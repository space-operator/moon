// #![feature(map_first_last)]
//  clear && ./sh/clean && cargo build && ./sh/bindgen && flutter run -d linux
#![feature(result_option_inspect)]

// TODO: fixed edges not updated for the last node movement frame

/*
    https://stackoverflow.com/a/62415754

    1: detect zoom gesture on touch screen and probably touchpads
    2: use separate events for mouse scroll (results in zoom)
       and touch or touchpad two-finger scroll (results in screen move)
*/

/*
    flutter-front-end
        db to from  view
    rust-front-end
        db to from  model
    rust-back-end
        db

    input (click with lmb where shift + !ctrl) -> behavior_widget

    widget (click)

    widget
        destroy
        add click handler
    click on input_id
*/
// mod workaround;
mod api;
mod command;
mod event;
mod flow_context;
mod input;
mod model;
mod state;
//pub mod storage;
mod utils;
mod view;

use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use api::Response;
use futures::executor::block_on;
use model::BasicWidgetKind;
use model::GraphId;
use model::SolanaNet;
use model::WidgetKind;

use model::WidgetNodeData;
use serde::Deserialize;

use serde_json::json;
use sunshine_core::msg::Action;
use sunshine_core::msg::QueryKind;
use sunshine_core::store::Datastore;
use sunshine_solana::RunState;

use uuid::Uuid;

use event::{Coords, Event};
use rid::RidStore;
use view::EdgeView;
use view::NodeView;
use view::NodeViewType;
use view::RunStateView;
use view::ViewEdgeType;
use view::{
    commands_view_map, BookmarkView, Command, DebugData, LastViewChanges, Ratio, Selection, View,
};

use sunshine_indra::store::generate_uuid_v1;

use crate::command::*;
use crate::model::{BookmarkId, BookmarkModel, PortId};
use crate::view::NodeViewType::{DummyEdgeHandle, WidgetBlock, WidgetTextInput};
use crate::view::{NodeChange, NodeChangeKind};

use api::*;
use input::{FlutterPointerEvent, MappingKind};
use model::{NodeId, NodeModel};
use state::*;

#[rid::store]
#[rid::structs(View, LastViewChanges)]
#[derive(rid::Config)]
pub struct Store {
    #[rid(skip)]
    state: Option<State>,
    view: View,
    last_view_changes: LastViewChanges,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InputEvent {
    node_id: String,
    text: String,
}

const MIN_SCROLL_ZOOM: f64 = 0.1; // zoom out
const MAX_SCROLL_ZOOM: f64 = 10.0; // zoom in

// Also valid for fit screen
const MIN_BOOKMARK_ZOOM: f64 = 0.1;
const MAX_BOOKMARK_ZOOM: f64 = 2.5;
const BOOKMARK_MARGIN_MULT: f64 = 0.25; // if 0 then no margin

const ZOOM_IN_MULT: f64 = std::f64::consts::SQRT_2;
const ZOOM_OUT_MULT: f64 = 1.0 / ZOOM_IN_MULT;

impl RidStore<Msg> for Store {
    fn create() -> Self {
        Self {
            state: None,
            view: View::default(),
            last_view_changes: LastViewChanges::default(),
        }
    }

    fn update(&mut self, req_id: u64, msg: Msg) {
        match msg {
            Msg::Initialize(ev) => {
                #[derive(Clone, Debug, Deserialize)]
                #[serde(rename_all = "camelCase")]
                struct InitializeEvent {
                    db_path: String,
                    log_path: String,
                    canvas_width: u64,
                    canvas_height: u64,
                }

                let event: InitializeEvent = serde_json::from_str(&ev).unwrap();
                let db_path = event.db_path;
                assert!(self.state.is_none());
                self.state = Some(State::new(
                    db_path.clone(),
                    event.log_path,
                    event.canvas_width,
                    event.canvas_height,
                ));

                self.update_graph_list();
                self.refresh_ui();

                rid::post(Confirm::Initialized(req_id, format!("{:?}", db_path,)));
            }
            Msg::ResizeCanvas(ev) => {
                #[derive(Clone, Debug, Deserialize)]
                #[serde(rename_all = "camelCase")]
                struct ResizeCanvasEvent {
                    width: u64,
                    height: u64,
                }

                let event: ResizeCanvasEvent = serde_json::from_str(&ev).unwrap();
                let state = self.state.as_mut().unwrap();

                state.canvas.width = event.width;
                state.canvas.height = event.height;
                rid::post(Confirm::ReceivedEvent(req_id, "".to_owned()));
            }
            Msg::MouseEvent(ev) => {
                let state = self.state.as_mut().unwrap();
                // dbg!(&ev);
                let events: Vec<_> = state.on_flutter_mouse_event(&ev).collect();
                self.apply(events, req_id);
                //rid::post(Confirm::ReceivedEvent(req_id, format!("{:?}", ev)));
                rid::post(Confirm::ReceivedEvent(req_id, "".to_owned()));
            }
            Msg::KeyboardEvent(ev) => {
                let state = self.state.as_mut().unwrap();
                // dbg!(&ev);
                let events: Vec<_> = state.on_flutter_keyboard_event(&ev).collect();
                self.apply(events, req_id);
                //rid::post(Confirm::ReceivedEvent(req_id, format!("{:?}", ev)));
                rid::post(Confirm::ReceivedEvent(req_id, "".to_owned()));
            }
            Msg::LoadGraph(ev) => {
                let state = self.state.as_mut().unwrap();
                let model = state.model_mut();
                model.bookmarks.clear(); // FIXME: do this in new_graph and read_graph
                if ev == "new" {
                    model.new_graph();
                    
                } else {
                    let graph_id = GraphId(Uuid::from_str(&ev).unwrap());
                    model.read_graph(graph_id);
                }

                state.reset();
                self.refresh_ui();

                rid::post(Confirm::LoadGraph(req_id, "".to_owned()));
            }
            Msg::DeleteGraph(graph_id) => {
                let model = self.state.as_mut().unwrap().model_mut();
                model.delete_graph(GraphId(Uuid::parse_str(&graph_id).unwrap()));

                self.refresh_ui();

                rid::post(Confirm::DeleteGraph(req_id, "".to_owned()));
            }
            Msg::RenameGraph(graph_id, new_name) => {
                let model = self.state.as_mut().unwrap().model_mut();
                model.rename_graph(GraphId(Uuid::parse_str(&graph_id).unwrap()), new_name);

                self.refresh_ui();

                rid::post(Confirm::RefreshUI(req_id, "".to_owned()));
            }
            Msg::ChangeSolanaNet(ev) => {
                let model = self.state.as_mut().unwrap().model_mut();
                let solana_net = match ev.as_str() {
                    "Testnet" => SolanaNet::Testnet,
                    "Devnet" => SolanaNet::Devnet,
                    "Mainnet" => SolanaNet::Mainnet,
                    _ => panic!(),
                };
                model.change_solana_net(solana_net);

                self.refresh_ui();

                rid::post(Confirm::ReceivedEvent(req_id, "".to_owned()));
            }
            Msg::Import(path) => {
                let model = self.state.as_mut().unwrap().model_mut();

                model.import(&path);

                let state = self.state.as_mut().unwrap();
                state.reset();
                self.refresh_ui();
                rid::post(Confirm::LoadGraph(req_id, "".to_owned()));
            }
            Msg::Export(path, filename) => {
                let model = self.state.as_ref().unwrap().model();

                model.export(path, filename);
                rid::post(Confirm::ReceivedEvent(req_id, "".to_owned()));
            }
            Msg::Debug(_ev) => {
                let model = self.state.as_ref().unwrap().model();
                let graph_id = model.graph_id();

                let graph = block_on(
                    model
                        .db()
                        .execute(Action::Query(QueryKind::ReadGraph(graph_id.0))),
                )
                .unwrap()
                .into_graph()
                .unwrap();

                let graph = serde_json::to_string_pretty(&graph).unwrap();

                // for console
                println!("{:#?}", graph);
                let model = dbg!(self.state.as_ref().unwrap().model());

                rid::post(Confirm::ReceivedEvent(req_id, graph.to_owned()));
            }
            Msg::StartInput(_) => {
                // when focus on textinput
                let mut state = self.state.as_mut().unwrap();
                state.ui_state = UiState::UiInput;

                rid::post(Confirm::ReceivedEvent(req_id, "".to_owned()));
            }
            Msg::StopInput(_) => {
                // when we loose focus
                let mut state = self.state.as_mut().unwrap();
                match &state.ui_state {
                    UiState::UiInput => state.ui_state = UiState::Default,
                    _ => {}
                }

                rid::post(Confirm::ReceivedEvent(req_id, "".to_owned()));
            }
            /*Msg::ApplyInput(ev) => {
                // on textinput sumbit/enter
                let mut state = self.state.as_mut().unwrap();
                match state.ui_state {
                    UiState::UiInput => {
                        let event: InputEvent = serde_json::from_str(&ev).unwrap();
                        println!("{:?}", event);
                        state.ui_state = UiState::Default;
                        let node_id = NodeId(Uuid::parse_str(&event.node_id).unwrap());
                        self.handle_text_input(node_id, &event.text, true, req_id);
                    }
                    _ => {}
                }


            }*/
            // Msg::SetText(ev) => {
            //     // set text but try to apply command

            //     // TODO: check if we are in UiState::UiInput
            //     // TODO: check can be added if StartInput and CancelInput properly called

            //     let event: InputEvent = serde_json::from_str(&ev).unwrap();
            //     println!("{:?}", event);

            //     // NodeId is Option only while we have no way to detect it
            //     // let node_id = match event.node_id.as_str() {
            //     //     "dummy" => state.active_node.unwrap(),
            //     //     node_id => NodeId(Uuid::parse_str(&node_id).unwrap()),
            //     // };

            //     let node_id = NodeId(Uuid::parse_str(&event.node_id).unwrap());

            //     self.handle_text_input(node_id, &event.text, false, req_id);
            //     rid::post(Confirm::ReceivedEvent(req_id, ev.to_owned()));
            // }
            Msg::SetText(ev) => {
                let event: InputEvent = serde_json::from_str(&ev).unwrap();
                println!("{:?}", event);
                let node_id = NodeId(Uuid::parse_str(&event.node_id).unwrap());

                // save to model
                self.state
                    .as_mut()
                    .unwrap()
                    .model_mut()
                    .set_node_text(&node_id, event.text.to_owned());

                // save to db
                self.state
                    .as_mut()
                    .unwrap()
                    .model_mut()
                    .save_text_to_db(&node_id, &event.text);

                //

                // https://github.com/solana-labs/solana/blob/master/remote-wallet/Cargo.toml
                // https://github.com/solana-labs/solana/commit/b635073829e41960c67556412edbb5315ff265e4
                // https://github.com/solana-labs/solana/commit/d6f22433d07584625c17aa83819e2396591242cd

                use serde::Serialize;

                let mut parser = parse::BlockParser::new();
                let lines = event.text.split("\n");
                for line in lines {
                    parser = parser.with(&line);
                }

                let blocks = parser.into_blocks();

                #[derive(Serialize)]
                struct Block {
                    spans: Vec<Span>,
                    children: Vec<Block>,
                }

                #[derive(Serialize)]
                pub enum Span {
                    Link(String),
                    Url(Vec<Span>, String),
                    Text(String),
                    Char(char),
                    Bold(Vec<Span>),
                    Italics(Vec<Span>),
                    Strikethrough(Vec<Span>),
                }

                impl From<parse::Block> for Block {
                    fn from(block: parse::Block) -> Self {
                        Self {
                            spans: parse::parse(&block.span_text)
                                .into_iter()
                                .map(Into::into)
                                .collect(),
                            children: block.children.into_iter().map(Into::into).collect(),
                        }
                    }
                }

                impl From<parse::Span<'_>> for Span {
                    fn from(span: parse::Span) -> Self {
                        match span {
                            parse::Span::Link(link) => Span::Link(link.0.to_owned()),
                            parse::Span::Url(spans, name) => Span::Url(
                                spans.into_iter().map(Into::into).collect(),
                                name.to_owned(),
                            ),
                            parse::Span::Text(text) => Span::Text(text.to_owned()),
                            parse::Span::Char(ch) => Span::Char(ch),
                            parse::Span::Bold(spans) => {
                                Span::Bold(spans.into_iter().map(Into::into).collect())
                            }
                            parse::Span::Italics(spans) => {
                                Span::Italics(spans.into_iter().map(Into::into).collect())
                            }
                            parse::Span::Strikethrough(spans) => {
                                Span::Strikethrough(spans.into_iter().map(Into::into).collect())
                            }
                        }
                    }
                }

                let blocks: Vec<Block> = blocks.into_iter().map(Into::into).collect();
                let json = serde_json::to_string_pretty(&blocks).unwrap();
                println!("{}", &json);
                //

                rid::post(Confirm::ReceivedEvent(req_id, "".to_owned()));
            }
            Msg::ApplyCommand(node_id, command_name) => {
                // node_id, command_name
                self.state
                    .as_mut()
                    .unwrap()
                    .apply_command(&node_id, &command_name)
                    .unwrap();
                self.refresh_ui();

                rid::post(Confirm::ApplyCommand(req_id, command_name.to_owned()));
            }
            Msg::ApplyAutocomplete(_ev) => {}
            Msg::SendJson(ev) => {
                let event: InputEvent = serde_json::from_str(&ev).unwrap();
                // println!("{:?}", event);
                // if !event.text.is_empty() {
                let model = self.state.as_mut().unwrap().model_mut();

                let node_id = NodeId(Uuid::parse_str(&event.node_id).unwrap());

                model.set_node_text(&node_id, event.text.clone());
                model.update_const_in_db(node_id, &event.text);
                // }

                rid::post(Confirm::ReceivedEvent(req_id, ev.to_owned()));
            }
            Msg::GenerateSeedPhrase(_) => {
                let phrase = crate::model::Model::generate_seed();

                rid::post(Confirm::SendSeedPhrase(req_id, phrase.to_owned()));
            }
            Msg::Deploy(_ev) => {
                let mut req_id_lock = self.state.as_ref().unwrap().model().req_id.lock().unwrap();
                *req_id_lock = req_id;

                self.state.as_ref().unwrap().model().deploy();

                // rid::post(Confirm::Deployed(req_id, ev.to_owned()));
            }
            Msg::UnDeploy(ev) => {
                self.state.as_ref().unwrap().model().undeploy();
                self.refresh_ui();

                rid::post(Confirm::UnDeployed(req_id, ev.to_owned()));
            }
            Msg::Request(ev) => {
                let request: Request = serde_json::from_str(&ev).unwrap();
                let response = match request {
                    Request::Initialize(data) => self.on_initialize(data),
                    Request::Mouse(data) => self.on_mouse(data),
                    Request::TextInput(data) => self.on_text_input(data),
                };
                let response = serde_json::to_string(&response).unwrap();
                rid::post(Confirm::Response(req_id, response));
            }

            // Refresh flow status while running
            Msg::Refresh(_ev) => {
                //println!("{}", ev);
                self.refresh_ui();
                rid::post(Confirm::RefreshStatus(req_id));
            }
            Msg::ResetZoom(_) => {
                self.state.as_mut().unwrap().transform = Transform::default();
                self.refresh_ui_transform();
                rid::post(Confirm::RefreshUI(req_id, "".to_owned()));
            }
            Msg::ZoomIn(_) => {
                let state = self.state.as_mut().unwrap();
                let new_zoom = state.transform.scale * ZOOM_IN_MULT;
                if new_zoom >= MIN_SCROLL_ZOOM && new_zoom <= MAX_SCROLL_ZOOM {
                    let screen_x = 0.5 * state.canvas.width as f64;
                    let screen_y = 0.5 * state.canvas.height as f64;

                    // println!(
                    //     "screen zoom {} {} {} {}",
                    //     screen_x, screen_y, state.transform.scale, new_zoom
                    // );

                    state.transform.x -= screen_x / state.transform.scale;
                    state.transform.y -= screen_y / state.transform.scale;

                    state.transform.scale = new_zoom;

                    state.transform.x += screen_x / state.transform.scale;
                    state.transform.y += screen_y / state.transform.scale;

                    self.refresh_ui_transform();
                    rid::post(Confirm::RefreshUI(req_id, "".to_owned()));
                } else {
                    rid::post(Confirm::ReceivedEvent(req_id, "".to_owned()));
                }
            }
            Msg::ZoomOut(_) => {
                let state = self.state.as_mut().unwrap();
                let new_zoom = state.transform.scale * ZOOM_OUT_MULT;
                if new_zoom >= MIN_SCROLL_ZOOM && new_zoom <= MAX_SCROLL_ZOOM {
                    let screen_x = 0.5 * state.canvas.width as f64;
                    let screen_y = 0.5 * state.canvas.height as f64;

                    // println!(
                    //     "screen zoom {} {} {} {}",
                    //     screen_x, screen_y, state.transform.scale, new_zoom
                    // );

                    state.transform.x -= screen_x / state.transform.scale;
                    state.transform.y -= screen_y / state.transform.scale;

                    state.transform.scale = new_zoom;

                    state.transform.x += screen_x / state.transform.scale;
                    state.transform.y += screen_y / state.transform.scale;

                    self.refresh_ui_transform();
                    rid::post(Confirm::RefreshUI(req_id, "".to_owned()));
                } else {
                    rid::post(Confirm::ReceivedEvent(req_id, "".to_owned()));
                }
            }
            Msg::FitNodesToScreen(_) => {
                let state = self.state.as_mut().unwrap();

                let model = state.model_mut();
                let mut nodes = model.nodes().values().filter(|node| {
                    matches!(node.data().kind, WidgetKind::Basic(BasicWidgetKind::Block))
                });
                let first_node = nodes.next();

                if let Some(node) = first_node {
                    let mut x1 = node.data().coords.x;
                    let mut y1 = node.data().coords.y;
                    let mut x2 = node.data().coords.x + node.data().dimensions.width as f64;
                    let mut y2 = node.data().coords.y + node.data().dimensions.height as f64;

                    // println!("Node {:10} {:10} : {:10} {:10}", &x1, &y1, &x2, &y2);

                    for node in nodes {
                        x1 = x1.min(node.data().coords.x);
                        y1 = y1.min(node.data().coords.y);
                        x2 = x2.max(node.data().coords.x + node.data().dimensions.width as f64);
                        y2 = y2.max(node.data().coords.y + node.data().dimensions.height as f64);

                        // println!("Node {:10} {:10} : {:10} {:10}", &x1, &y1, &x2, &y2);
                    }

                    let x = 0.5 * (x1 + x2);
                    let y = 0.5 * (y1 + y2);
                    let w = x2 - x1;
                    let h = y2 - y1;

                    let x1 = x - 0.5 * w * (1.0 + BOOKMARK_MARGIN_MULT);
                    let y1 = y - 0.5 * h * (1.0 + BOOKMARK_MARGIN_MULT);
                    let x2 = x + 0.5 * w * (1.0 + BOOKMARK_MARGIN_MULT);
                    let y2 = y + 0.5 * h * (1.0 + BOOKMARK_MARGIN_MULT);

                    let xscale = state.canvas.width as f64 / (x2 - x1);
                    let yscale = state.canvas.height as f64 / (y2 - y1);
                    let scale = xscale
                        .min(yscale)
                        .clamp(MIN_BOOKMARK_ZOOM, MAX_BOOKMARK_ZOOM);

                    let tx = x - 0.5 * state.canvas.width as f64 / scale;
                    let ty = y - 0.5 * state.canvas.height as f64 / scale;
                    // dbg!(tx, ty);
                    // println!("FitToScreen {:10} {:10} : {:10} {:10}", x, y, w, h);
                    // println!("FitToScreen {:10} {:10} : {:10} {:10}", &x1, &y1, &x2, &y2);
                    // println!(
                    //     "FitToScreen {:10} {:10} {:10} : {:10} {:10}",
                    //     xscale, yscale, scale, tx, ty
                    // );

                    self.state.as_mut().unwrap().transform = Transform {
                        x: -tx,
                        y: -ty,
                        scale,
                    };
                    self.refresh_ui_transform();
                    rid::post(Confirm::RefreshUI(req_id, "".to_owned()));
                } else {
                    rid::post(Confirm::ReceivedEvent(req_id, "".to_owned()));
                }
            }
            Msg::CreateBookmark(name) => {
                let state = self.state.as_mut().unwrap();
                let bookmark_id = BookmarkId(generate_uuid_v1());
                let nodes = state.selected_node_ids.clone();

                // save to model
                let bookmark_model = BookmarkModel {
                    name,
                    nodes_ids: nodes,
                };
                state
                    .model_mut()
                    .bookmarks
                    .insert(bookmark_id, bookmark_model.clone());

                //save to db
                state.model_mut().save_bookmark(bookmark_id, bookmark_model);
                // state.model_mut()
                self.refresh_ui();

                rid::post(Confirm::RefreshUI(req_id, bookmark_id.0.to_string()));
            }
            Msg::GotoBookmark(bookmark_id) => {
                // bookmark_id can be obtained from view.bookmarks
                let state = self.state.as_mut().unwrap();

                let model = state.model_mut();
                let bookmark_id = BookmarkId(bookmark_id.parse().unwrap());
                let bookmark = model.bookmarks.get(&bookmark_id).unwrap();
                // dbg!(&bookmark);

                let mut nodes = bookmark
                    .nodes_ids
                    .iter()
                    .filter_map(|node_id| model.nodes().get(node_id));
                let first_node = nodes.next();
                // dbg!(&first_node);

                if let Some(node) = first_node {
                    let mut x1 = node.data().coords.x;
                    let mut y1 = node.data().coords.y;
                    let mut x2 = node.data().coords.x + node.data().dimensions.width as f64;
                    let mut y2 = node.data().coords.y + node.data().dimensions.height as f64;

                    // println!("Node {:10} {:10} : {:10} {:10}", &x1, &y1, &x2, &y2);

                    for node in nodes {
                        x1 = x1.min(node.data().coords.x);
                        y1 = y1.min(node.data().coords.y);
                        x2 = x2.max(node.data().coords.x + node.data().dimensions.width as f64);
                        y2 = y2.max(node.data().coords.y + node.data().dimensions.height as f64);

                        println!("Node {:10} {:10} : {:10} {:10}", &x1, &y1, &x2, &y2);
                    }

                    let x = 0.5 * (x1 + x2);
                    let y = 0.5 * (y1 + y2);
                    let w = x2 - x1;
                    let h = y2 - y1;

                    let x1 = x - 0.5 * w * (1.0 + BOOKMARK_MARGIN_MULT);
                    let y1 = y - 0.5 * h * (1.0 + BOOKMARK_MARGIN_MULT);
                    let x2 = x + 0.5 * w * (1.0 + BOOKMARK_MARGIN_MULT);
                    let y2 = y + 0.5 * h * (1.0 + BOOKMARK_MARGIN_MULT);

                    let xscale = state.canvas.width as f64 / (x2 - x1);
                    let yscale = state.canvas.height as f64 / (y2 - y1);
                    let scale = xscale
                        .min(yscale)
                        .clamp(MIN_BOOKMARK_ZOOM, MAX_BOOKMARK_ZOOM); //controls min/

                    let tx = x - 0.5 * state.canvas.width as f64 / scale;
                    let ty = y - 0.5 * state.canvas.height as f64 / scale;
                    // dbg!(tx, ty);
                    // println!("Bookmark {:10} {:10} : {:10} {:10}", x, y, w, h);
                    // println!("Bookmark {:10} {:10} : {:10} {:10}", &x1, &y1, &x2, &y2);
                    // println!(
                    //     "Bookmark {:10} {:10} {:10} : {:10} {:10}",
                    //     xscale, yscale, scale, tx, ty
                    // );

                    self.state.as_mut().unwrap().transform = Transform {
                        x: -tx,
                        y: -ty,
                        scale,
                    };
                    self.refresh_ui_transform();
                    rid::post(Confirm::RefreshUI(req_id, "".to_owned()));
                } else {
                    // TODO: Handle if no nodes in bookmark
                    rid::post(Confirm::ReceivedEvent(req_id, "".to_owned()));
                }
            }

            Msg::BookmarkScreenshot(bookmark_id) => {
                // bookmark_id can be obtained from view.bookmarks
                let state = self.state.as_mut().unwrap();

                let model = state.model_mut();
                let bookmark_id = BookmarkId(bookmark_id.parse().unwrap());
                // let bookmark_id = model.bookmarks.keys().next().unwrap().clone(); // FIXME: Remove this, use bookmark_id from above
                let bookmark = model.bookmarks.get(&bookmark_id).unwrap();
                // dbg!(&bookmark);

                let mut nodes = bookmark
                    .nodes_ids
                    .iter()
                    .filter_map(|node_id| model.nodes().get(node_id));
                let first_node = nodes.next();
                // dbg!(&first_node);

                if let Some(node) = first_node {
                    let mut x1 = node.data().coords.x;
                    let mut y1 = node.data().coords.y;
                    let mut x2 = node.data().coords.x + node.data().dimensions.width as f64;
                    let mut y2 = node.data().coords.y + node.data().dimensions.height as f64;

                    // println!("Node {:10} {:10} : {:10} {:10}", &x1, &y1, &x2, &y2);

                    for node in nodes {
                        x1 = x1.min(node.data().coords.x);
                        y1 = y1.min(node.data().coords.y);
                        x2 = x2.max(node.data().coords.x + node.data().dimensions.width as f64);
                        y2 = y2.max(node.data().coords.y + node.data().dimensions.height as f64);

                        println!("Node {:10} {:10} : {:10} {:10}", &x1, &y1, &x2, &y2);
                    }

                    let x = 0.5 * (x1 + x2);
                    let y = 0.5 * (y1 + y2);
                    let w = x2 - x1;
                    let h = y2 - y1;

                    let x1 = x - 0.5 * w * (1.0 + BOOKMARK_MARGIN_MULT);
                    let y1 = y - 0.5 * h * (1.0 + BOOKMARK_MARGIN_MULT);
                    let x2 = x + 0.5 * w * (1.0 + BOOKMARK_MARGIN_MULT);
                    let y2 = y + 0.5 * h * (1.0 + BOOKMARK_MARGIN_MULT);

                    let xscale = state.canvas.width as f64 / (x2 - x1);
                    let yscale = state.canvas.height as f64 / (y2 - y1);
                    let scale = xscale
                        .min(yscale)
                        .clamp(MIN_BOOKMARK_ZOOM, MAX_BOOKMARK_ZOOM); //controls min/

                    let tx = x - 0.5 * state.canvas.width as f64 / scale;
                    let ty = y - 0.5 * state.canvas.height as f64 / scale;
                    // dbg!(tx, ty);
                    // println!("Bookmark {:10} {:10} : {:10} {:10}", x, y, w, h);
                    // println!("Bookmark {:10} {:10} : {:10} {:10}", &x1, &y1, &x2, &y2);
                    // println!(
                    //     "Bookmark {:10} {:10} {:10} : {:10} {:10}",
                    //     xscale, yscale, scale, tx, ty
                    // );

                    // let transform = Transform {
                    //     x: -tx,
                    //     y: -ty,
                    //     scale,
                    // };
                    self.state.as_mut().unwrap().transform_screenshot = Transform {
                        x: -tx,
                        y: -ty,
                        scale,
                    };
                    self.refresh_ui_transform_screenshot();
                    rid::post(Confirm::RefreshUI(req_id, "".to_owned()));
                } else {
                    // TODO: Handle if no nodes in bookmark
                    rid::post(Confirm::ReceivedEvent(req_id, "".to_owned()));
                }
            }
            Msg::DeleteBookmark(bookmark_id) => {
                let state = self.state.as_mut().unwrap();
                let bookmark_id = BookmarkId(bookmark_id.parse().unwrap());

                let _bookmark_data = state.model_mut().delete_bookmark(bookmark_id);

                // FIXME: Check bookmark_data.is_none()
                self.refresh_ui();

                rid::post(Confirm::RefreshUI(req_id, "".to_owned()));
            }

            Msg::RemoveNode(node_id) => {
                //println!("remove nodes {:?}", node_id);
                let state = self.state.as_mut().unwrap();

                state
                    .model_mut()
                    .remove_node(NodeId(Uuid::parse_str(&node_id).unwrap()));
                self.refresh_ui();
                rid::post(Confirm::RemoveNode(req_id, node_id));
            }

            // FIXME: somehow invalid node_id received
            // and probably panicked in set_node_text, set_node_additional_data
            Msg::UpdateDimensions(node_id, value_type, width, height) => {
                // Command Id
                let child_id = NodeId(Uuid::parse_str(&node_id).unwrap());

                // Get Block Id
                let state = self.state.as_mut().unwrap();
                let selected_block_id = state.active_node.unwrap();

                // clear text content
                state.model_mut().set_node_text(&child_id, String::new());
                state
                    .model_mut()
                    .update_const_in_db(child_id, &String::new());

                // get sub node type: String, Json, Bool, Seed Phrase, etc.
                let additional_data = &value_type;

                //get config based on

                // save to model
                state
                    .model_mut()
                    .set_node_additional_data(&child_id, additional_data.clone());

                // save to db
                state
                    .model_mut()
                    .update_const_additional_in_db(child_id, additional_data);

                // dimensions
                let dimensions = model::NodeDimensions { height, width };

                // update block dimensions
                state
                    .model_mut()
                    .set_node_dimensions(&selected_block_id, dimensions.clone());

                //update child dimensions
                state
                    .model_mut()
                    .set_node_dimensions(&child_id, dimensions.clone());

                // format dimensions for Dart
                let dimensions = json!(dimensions);

                // refresh ui
                self.refresh_ui();

                rid::post(Confirm::UpdatedDimensions(req_id, dimensions.to_string()));
            }
            Msg::SetAdditionalData(node_id, value_type) => {
                let child_id = NodeId(Uuid::parse_str(&node_id).unwrap());

                let state = self.state.as_mut().unwrap();

                // get sub node type: String, Json, Bool, Seed Phrase, etc.
                let additional_data = &value_type;

                //get config based on

                // save to model
                state
                    .model_mut()
                    .set_node_additional_data(&child_id, additional_data.clone());

                // save to db
                state
                    .model_mut()
                    .update_const_additional_in_db(child_id, additional_data);
            }
            Msg::SetMappingKind(kind) => {
                let state = self.state.as_mut().unwrap();

                match kind.as_str() {
                    "mouse" => state.mapping_kind = MappingKind::Mouse,
                    "touch" => state.mapping_kind = MappingKind::Touch,
                    _ => panic!("invalid mapping kind"),
                }

                rid::post(Confirm::ReceivedEvent(req_id, "".to_owned()));
            }
        };
        // rid::post(Confirm::ReceivedEvent(req_id, String::new()));
    }
}

impl Store {
    pub fn handle_text_input(
        &mut self,
        node_id: NodeId,
        text: &str,
        is_finished: bool,
        req_id: u64,
    ) {
        let mut state = self.state.as_mut().unwrap();

        println!("{:?}", text);
        let data = if is_finished {
            text.split_once("/").and_then(|(before, after)| {
                after
                    .split_once(&[' ', '\n'])
                    .map(|(command, after)| (before, command, after))
                    .or_else(|| Some((before, after, "")))
            })
        } else {
            text.split_once("/").and_then(|(before, after)| {
                after
                    .split_once(&[' ', '\n'])
                    .map(|(command, after)| (before, command, after))
            })
        };

        if let Some((_before, command, _after)) = data {
            let node_id_str = node_id.0.to_string();
            match state.apply_command(&node_id_str, command) {
                Ok(()) => {
                    state.ui_state = UiState::Default;
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id, "".to_owned()));
                    // send StopInput to flutter
                }
                /*CommandAction::Replace(replacement) => {
                    state
                        .model_mut()
                        .set_node_text(&node_id, format!("{}{}{}", before, replacement, after));
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id, "".to_owned()));
                    // send updated text to flutter
                }*/
                Err(()) => {
                    // todo: check all commands
                    state.model_mut().set_node_text(&node_id, text.to_owned());
                }
            }
        } else {
            state.model_mut().set_node_text(&node_id, text.to_owned());
        }
    }

    fn on_initialize(&mut self, _data: InitializeRequest) -> Option<Response> {
        // let db_path = "/home/amir/SUNSHINE_DB".to_owned(); //FIXME
        // assert!(self.state.is_none());
        // self.state = Some(State::new(db_path.clone()));
        // self.update_graph_list();
        // self.refresh_ui();
        // Some(Response::RefreshUi)
        todo!()
    }

    fn on_mouse(&mut self, _data: FlutterPointerEvent) -> Option<Response> {
        todo!()
    }

    fn on_text_input(&mut self, _data: InputRequest) -> Option<Response> {
        todo!()
    }
}

#[rid::message(Confirm)]
#[derive(Debug, Clone)]
pub enum Msg {
    Initialize(String),
    ResizeCanvas(String),
    MouseEvent(String),
    KeyboardEvent(String),
    LoadGraph(String), //StartEdge(String), // { input: { ... } } | { scheduled: { id: 32 | 64 } }
    Debug(String),
    SendJson(String),
    StartInput(String),
    StopInput(String),
    // ApplyInput(String),
    SetText(String), // { node_id, text }
    // SetText2(String),     // { node_id, text }
    ApplyCommand(String, String),
    ApplyAutocomplete(String),
    Deploy(String),
    UnDeploy(String),
    Request(String), //
    Refresh(String),
    Import(String),
    Export(String, String),
    ResetZoom(String),
    FitNodesToScreen(String),
    CreateBookmark(String),
    GotoBookmark(String),
    DeleteBookmark(String),
    ChangeSolanaNet(String),
    BookmarkScreenshot(String),
    UpdateDimensions(String, String, i64, i64),
    GenerateSeedPhrase(String),
    RemoveNode(String),
    SetMappingKind(String), // "mouse" | "touch"
    ZoomIn(String),
    ZoomOut(String),
    SetAdditionalData(String, String),
    RenameGraph(String, String),
    DeleteGraph(String),
}

#[rid::reply]
#[derive(Clone)]
pub enum Confirm {
    ReceivedEvent(u64, String), // { updates: [ ... ], scheduled: [[1.0, 64], [2.0, 32]] }
    RefreshUI(u64, String),
    Initialized(u64, String),
    LoadGraph(u64, String),
    Deployed(u64, String),
    UnDeployed(u64, String),
    ApplyCommand(u64, String),
    Response(u64, String), //
    RequestRefresh(u64),
    RefreshStatus(u64),
    CreateNode(u64),
    RemoveNode(u64, String),
    UpdatedDimensions(u64, String),
    SendSeedPhrase(u64, String),
    Debug(u64, String),
    DeleteGraph(u64, String),
    RefreshNode(u64, String),
    RefreshDraggedEdge(u64, String),
}

impl Store {
    fn apply(&mut self, events: impl IntoIterator<Item = Event>, req_id: u64) {
        for event in events {
            let state = self.state.as_mut().unwrap();
            // println!("{:?}", event);
            match event {
                // BASIC SELECTION
                Event::Unselect => {
                    // FIXME: background doen't send events if node loses focus
                    // println!("unselect");
                    state.clear_selection();
                    self.refresh_selection();
                    rid::post(Confirm::RefreshUI(req_id, "selection".to_owned()))
                }
                Event::SelectNode(node_id) => {
                    // println!("select node {node_id:?}");
                    state.clear_selection();

                    state.add_to_selection(node_id);
                    // state.update_active_node(node_id);
                    self.refresh_selection();
                    rid::post(Confirm::RefreshUI(req_id, "selection".to_owned()))
                }
                Event::AddOrRemoveNodeToSelection(node_id) => {
                    // println!("add node to selection {node_id:?}");
                    state.add_or_remove_from_selection(node_id);

                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id, "".to_owned()))
                }

                // CRUD
                Event::CreateNode(coords) => {
                    let node_id = state.model_mut().create_starting_node_block(coords);
                    // println!("selected node {:?}", node_id);
                    state.add_to_selection(node_id);
                    self.refresh_ui();
                    rid::post(Confirm::CreateNode(req_id))
                }
                Event::EditNode(_) => {}
                Event::RemoveNodes(removable_node_ids) => {
                    // println!("remove nodes {:?}", removable_node_ids);
                    for node_id in removable_node_ids {
                        state.model_mut().remove_node(node_id);
                        let node_id = node_id.0.to_string();
                        rid::post(Confirm::RemoveNode(req_id, node_id))
                    }
                    self.refresh_ui();
                    /*let mut all_removable_nodes = HashSet::new();
                    let mut parents: HashMap<NodeId, NodeId> = HashMap::new();

                    for (edge_id, edge) in state.model().node_edges() {
                        parents.insert(edge.to, edge.from);
                    }

                    for (node_id, node) in state.model().nodes() {
                        let mut parent_id = Some(node_id);
                        while let Some(child_id) = parent_id {
                            if removable_node_ids.contains(&child_id) {
                                all_removable_nodes.insert(node_id);
                                break;
                            }

                            parent_id = parents.get(child_id);
                        }
                    }*/
                }

                // SELECTION RECTANGLE
                Event::MaybeStartSelection(start) => {
                    // TODO: todo
                    // state.ui_state = UiState::MaybeSelection(start);
                }
                Event::NotASelection => {
                    state.ui_state = UiState::Default;
                }
                Event::StartSelection(start_coords, coords) => {
                    state.ui_state = UiState::Selection(start_coords, coords);
                }
                Event::ContinueSelection(_, _) => {}
                Event::EndSelection(_, _) => {
                    state.ui_state = UiState::Default; // question
                }
                Event::CancelSelection => {
                    // println!(":  cancelled");
                    state.ui_state = UiState::Default;
                }

                // SELECTION RECTANGLE
                Event::MaybeStartTransformMove(start) => {
                    state.ui_state = UiState::MaybeTransformMove(start);
                }
                Event::NotATransformMove => {
                    state.ui_state = UiState::Default;
                }
                Event::StartTransformMove(start_coords, coords) => {
                    let transform = &mut state.transform;
                    transform.x += coords.x - start_coords.x;
                    transform.y += coords.y - start_coords.y;
                    //dbg!(start_coords.x, coords.x, transform.x);
                    state.ui_state = UiState::TransformMove(start_coords);
                    self.refresh_ui_transform();
                    rid::post(Confirm::RefreshUI(req_id, "".to_owned()))
                }
                Event::ContinueTransformMove(start_coords, coords) => {
                    let transform = &mut state.transform;
                    transform.x += coords.x - start_coords.x;
                    transform.y += coords.y - start_coords.y;
                    //dbg!(start_coords.x, coords.x, transform.x);
                    state.ui_state = UiState::TransformMove(start_coords);
                    self.refresh_ui_transform();
                    rid::post(Confirm::RefreshUI(req_id, "".to_owned()))
                }
                Event::EndTransformMove(start_coords, coords) => {
                    let transform = &mut state.transform;
                    transform.x += coords.x - start_coords.x;
                    transform.y += coords.y - start_coords.y;
                    //dbg!(start_coords.x, coords.x, transform.x);
                    state.ui_state = UiState::Default;
                    self.refresh_ui_transform();
                    rid::post(Confirm::RefreshUI(req_id, "".to_owned()))
                }
                Event::CancelTransformMove => {
                    state.ui_state = UiState::Default;
                }
                // MOVE
                Event::MaybeStartNodeMove(node_id, coords) => {
                    //println!("maybe start node move {:?}", node_id);
                    state.ui_state = UiState::MaybeNodeMove(node_id, coords);
                }
                Event::NotANodeMove => {
                    state.ui_state = UiState::Default;
                }
                Event::StartNodeMove(node_id, start_coords, coords) => {
                    //println!("start node move {:?}", node_id);
                    if !state.selected_node_ids.contains(&node_id) {
                        //println!("do reselect on maybe start node move");
                        state.clear_selection();
                        state.add_to_selection(node_id.clone());
                    }

                    state.ui_state = UiState::NodeMove(start_coords, coords);
                    // self.refresh_ui(); //FIXME commented to speed up but needed for selection
                    self.refresh_selection();
                    rid::post(Confirm::RefreshUI(req_id, "selection".to_owned()))
                    // select node
                    // state.clear_selection(); // clears :  on multi select // FIXME: workaround

                    // state.add_to_selection(node_id.clone());
                    /*
                    let node_ids: Vec<NodeId> = state.selected_node_ids().copied().collect();

                    for node_id in node_ids {
                        let node_id_str = node_id.0.to_string();
                        let node_view = self.view.nodes.get_mut(&node_id_str).unwrap();

                        // set origin coordinates
                        let node = state.model().get_node(&node_id).unwrap();

                        let node_coords = match node {
                            NodeModel::Widget(data) => data.coords,
                            NodeModel::Data(_) => panic!(),
                        };
                        node_view.origin_x = node_coords.x;
                        node_view.origin_y = node_coords.y;
                    }*/
                }
                Event::ContinueNodeMove(start_coords, coords) => {
                    state.ui_state = UiState::NodeMove(start_coords, coords);
                    let selection = state.selected_node_ids.clone();
                    // dbg!(&selection);
                    self.refresh_nodes(selection.clone());

                    // if want to pass node_ids as String
                    // let ids: Vec<String> =
                    //     selection.iter().map(|uuid| uuid.0.to_string()).collect();
                    // let ids = serde_json::to_string(&ids).unwrap();

                    rid::post(Confirm::RefreshNode(req_id, "".to_owned()));
                    // self.refresh_ui();
                    // rid::post(Confirm::RefreshUI(req_id, "".to_owned()));

                    /*
                    let node_ids: Vec<NodeId> = state.selected_node_ids().copied().collect();

                    for node_id in node_ids {
                        // move only in view, not in model
                        let node_id_str = node_id.0.to_string();
                        let node_view = self.view.nodes.get_mut(&node_id_str).unwrap();

                        node_view.x = node_view.origin_x + coords.x - start_coords.x;
                        node_view.y = node_view.origin_y + coords.y - start_coords.y;
                    }*/
                }
                Event::EndNodeMove(start_coords, coords) => {
                    let node_ids: Vec<NodeId> = state.selected_node_ids().copied().collect();

                    for node_id in node_ids {
                        let node = state.model().get_node(&node_id).unwrap();
                        let data = match node {
                            NodeModel::Widget(data) => data,
                            // NodeModel::Data(_) => panic!(),
                        };
                        let x = data.coords.x + coords.x - start_coords.x;
                        let y = data.coords.y + coords.y - start_coords.y;
                        state.set_node_coords(&node_id, Coords { x, y });
                    }

                    state.ui_state = UiState::Default;
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id, "".to_owned()));
                }
                Event::CancelNodeMove => {
                    /*let node_ids: Vec<NodeId> = state.selected_node_ids().copied().collect();

                    for node_id in node_ids {
                        // move only in view, not in model
                        let node_id_str = node_id.0.to_string();
                        let node_view = self.view.nodes.get_mut(&node_id_str).unwrap();

                        node_view.x = node_view.origin_x;
                        node_view.y = node_view.origin_y;
                    }*/

                    state.ui_state = UiState::Default;
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id, "".to_owned()));
                }
                /*Event::Typing(chars) => {
                    match state.ui_state {
                        UiState::Typing(node_id, text) => {
                            state.ui_state = UiState::Typing(node_id, text + &chars)
                        }
                        UiState::Default || ... => panic!(),
                    }
                }*/
                Event::MaybeStartEdge(port_id) => {
                    state.ui_state = UiState::MaybeEdge(port_id);
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id, "".to_owned()));
                }
                Event::NotAEdge => {
                    state.ui_state = UiState::Default;
                }
                Event::StartEdge(port_id, coords) => {
                    state.ui_state = UiState::Edge(port_id, coords);
                    self.refresh_ui_flow_edges(); // self.refresh_ui();
                    rid::post(Confirm::RefreshDraggedEdge(req_id, "start".to_owned()));
                }
                Event::ContinueEdge(port_id, coords) => {
                    state.ui_state = UiState::Edge(port_id, coords);
                    self.refresh_ui_flow_edges();
                    rid::post(Confirm::RefreshDraggedEdge(req_id, "".to_owned()));
                }
                Event::EndEdge(port_id, output_id) => {
                    // println!("Connect Edge {:?} {:?}", port_id, output_id);
                    state
                        .model_mut()
                        .add_or_remove_flow_edge(port_id, output_id);
                    state.ui_state = UiState::Default; // Question? should it be last?
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id, "end_edge".to_owned()));
                }
                Event::CancelEdge(_) => {
                    state.ui_state = UiState::Default;
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id, "end_edge".to_owned()));
                } /*Event::StartCommandInput(command) | Event::ModifyCommandInput(command) => {
                println!("command input: {}", &command);
                state.ui_state = UiState::CommandInput(command);
                self.refresh_ui();
                rid::post(Confirm::RefreshUI(req_id));
                }*/
                /*Event::ApplyCommandInput(command) => {
                }
                Event::CancelCommandInput => {
                    state.ui_state = UiState::Default;
                    println!("cancel command input");
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id));
                }*/
                Event::ScrollZoom(x, y, dzoom) => {
                    // let old_zoom = state.transform.scale;
                    let new_zoom = state.transform.scale * dzoom;
                    if new_zoom >= MIN_SCROLL_ZOOM && new_zoom <= MAX_SCROLL_ZOOM {
                        let screen_x = (x + state.transform.x) * state.transform.scale;
                        let screen_y = (y + state.transform.y) * state.transform.scale;
                        // println!("screen zoom {} {} {} {}", screen_x, screen_y, state.transform.scale, new_zoom);

                        state.transform.x -= screen_x / state.transform.scale;
                        state.transform.y -= screen_y / state.transform.scale;

                        state.transform.scale = new_zoom;

                        state.transform.x += screen_x / state.transform.scale;
                        state.transform.y += screen_y / state.transform.scale;

                        self.refresh_ui_transform();
                        rid::post(Confirm::RefreshUI(req_id, "".to_owned()));
                    }
                }
                Event::ScrollMoveScreen(dx, dy) => {
                    state.transform.x -= dx / state.transform.scale;
                    state.transform.y -= dy / state.transform.scale;
                    self.refresh_ui_transform();
                    rid::post(Confirm::RefreshUI(req_id, "".to_owned()));
                }
            }
        }
    }

    fn update_graph_list(&mut self) {
        let state = self.state.as_ref().unwrap();

        self.view.graph_list = state.model().graph_list.clone();
    }

    /*
        #[derive(Clone, Copy, Debug)]
        struct RefreshFlags {
            nodes: bool, // Vec<NodeId> // Filter
            edges: bool,
            smth_else: bool,
        }

        fn refresh_ui(&mut self, flags: RefreshFlags) {
            let state = self.state.as_ref().unwrap();

            let (nodes, changed_nodes_ids) = if flags.nodes {
                ...

                let changed_nodes_ids = ...
            } else {
                (state.view.nodes, vec![]);
            }

            // ...
            let changes = LastViewChanges {
                changed_nodes_ids,
                changed_flow_edges_ids,
                is_selected_node_ids_changed,
                is_selection_changed,
                is_command_changed,
                is_text_commands_changed,
                is_graph_list_changed,
                is_highlighted_changed,
                is_transform_changed,
                is_transform_screenshot_changed,
                is_graph_changed,
                is_bookmark_changed,
            };
            self.view = new_view;
            self.last_view_changes = changes;
        }
    */

    fn refresh_selection(&mut self) {
        let state = self.state.as_ref().unwrap();

        let old_view = &self.view;

        let mut changes = LastViewChanges::default();

        ///
        let command = if let UiState::CommandInput(command) = &state.ui_state {
            Command {
                is_active: true,
                command: command.to_owned(),
            }
        } else {
            Command {
                is_active: false,
                command: String::new(),
            }
        };

        let selected_node_ids: Vec<String> = self
            .state
            .as_ref()
            .unwrap()
            .selected_node_ids()
            .map(|uuid| uuid.0.to_string())
            .collect();

        // TODO not in changes
        // self.view.selected_command_ids =
        let selected_command_ids: Vec<String> = self
            .state
            .as_ref()
            .unwrap()
            .model()
            .node_edges()
            .iter()
            .filter(|(_, model)| {
               selected_node_ids
                    .contains(&model.from.0.to_string())
            })
            .map(|(_, model)| model.to.0.to_string())
            .collect();

        let is_selected_node_ids_changed = old_view.selected_node_ids != selected_node_ids;
        let is_selected_command_changed = old_view.selected_command_ids != selected_command_ids;
        let is_command_changed = old_view.command != command;

        changes.is_selected_node_ids_changed = is_selected_node_ids_changed;
        changes.is_command_changed = is_command_changed;

        self.view.command = command;
        self.view.selected_node_ids = selected_node_ids;
        self.view.selected_command_ids = selected_command_ids;

        self.last_view_changes = changes;
    }

    fn refresh_nodes(&mut self, node_ids_to_update: HashSet<NodeId>) {
        // println!("refresh node");

        let mut changes = LastViewChanges::default();

        let state = self.state.as_ref().unwrap();

           // move currently movable nodes
           let (dx, dy) = if let UiState::NodeMove(start_coords, coords) = state.ui_state {
            let dx = coords.x - start_coords.x;
            let dy = coords.y - start_coords.y;

            // model.nodes = 1:A 2:B 3:C 4:D 5:E 6:F
            // node_ids_to_update 2 6 3 1
            // model.nodes = ...

            for node_id in &node_ids_to_update {
                let widget_node_data = state.model().get_node(node_id).unwrap().data(); //FIXME unwraps none?

                let node_id_str = node_id.0.to_string();
                changes.changed_nodes_ids.insert(
                    node_id_str.clone(),
                    NodeChange {
                        kind: NodeChangeKind::Modified,
                    },
                );
                let node_view = self.view.nodes.get_mut(&node_id_str).unwrap();

                // node_view.origin_x = widget_node_data.coords.x as i64 + dx as i64;
                // node_view.origin_y = widget_node_data.coords.y as i64 + dy as i64;
                node_view.x = widget_node_data.coords.x as i64 + dx as i64;
                node_view.y = widget_node_data.coords.y as i64 + dy as i64;
            }

            (dx, dy)
        } else {
            (0.0, 0.0)
        };

        // insert input/output edges
        for (edge_id, edge) in state.model().flow_edges() {
            let input = state.model().inputs().get(&edge.input_id).unwrap();
            let output = state.model().outputs().get(&edge.output_id).unwrap();

            if !node_ids_to_update.contains(&input.parent_node_id)
                && !node_ids_to_update.contains(&output.parent_node_id)
            {
                continue;
            }

            let input_node = state.model().nodes().get(&input.parent_node_id).unwrap();
            let input_data = match input_node {
                NodeModel::Widget(data) => data,
                // NodeModel::Data(_) => panic!(),
            };
            let output_node = state.model().nodes().get(&output.parent_node_id).unwrap();
            let output_data = match output_node {
                NodeModel::Widget(data) => data,
                // NodeModel::Data(_) => panic!(),
            };

            //let node_view = node_views
            //    .get_mut(&input.parent_node_id.0.to_string())
            //   .unwrap();

            let (dx1, dy1) = if node_ids_to_update.contains(&output.parent_node_id) {
                (dx, dy)
            } else {
                (0.0, 0.0)
            };

            let (dx2, dy2) = if node_ids_to_update.contains(&input.parent_node_id) {
                (dx, dy)
            } else {
                (0.0, 0.0)
            };

            changes.changed_flow_edges_ids.push(edge_id.0.to_string());
            self.view.flow_edges.insert(
                edge_id.0.to_string(),
                EdgeView {
                    from: edge.output_id.0.to_string(), // FIXME: input.label.clone(),
                    to: edge.input_id.0.to_string(),    // FIXME: output.label.clone(),
                    edge_type: ViewEdgeType::Flow,
                    from_coords_x: (output_data.coords.x + output.local_coords.x + dx1) as i64 + 35,
                    from_coords_y: (output_data.coords.y + output.local_coords.y + dy1) as i64
                        + INPUT_SIZE / 2, //half width of input size
                    to_coords_x: (input_data.coords.x + input.local_coords.x + dx2) as i64 + 15,
                    to_coords_y: (input_data.coords.y + input.local_coords.y + dy2) as i64
                        + INPUT_SIZE / 2,
                },
            );
        }
        self.last_view_changes = changes;
    }

    fn refresh_ui(&mut self) {
        // println!("refresh ui");
        //let old_view = std::mem::take(&mut self.view);

        let state = self.state.as_ref().unwrap();

        // SELECTION
        let selection = if let UiState::Selection(start_coords, coords) = &state.ui_state {
            Selection {
                is_active: true,
                x1: start_coords.x as i64,
                y1: start_coords.y as i64,
                x2: coords.x as i64,
                y2: coords.y as i64,
            }
        } else {
            Selection {
                is_active: false,
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
            }
        };

        // COMMAND INPUT
        let command = if let UiState::CommandInput(command) = &state.ui_state {
            Command {
                is_active: true,
                command: command.to_owned(),
            }
        } else {
            Command {
                is_active: false,
                command: String::new(),
            }
        };

        // GET WIDGET NODES
        let widget_nodes = state
            .model()
            .iter_widget_nodes()
            .map(|(node_id, widget_node_data)| {
                (
                    node_id.0.to_string(),
                    NodeView {
                        index: i64::default(),
                        parent_id: "".to_owned(),
                        origin_x: widget_node_data.coords.x as i64,
                        origin_y: widget_node_data.coords.y as i64,
                        x: widget_node_data.coords.x as i64,
                        y: widget_node_data.coords.y as i64,
                        height: widget_node_data.dimensions.height,
                        width: widget_node_data.dimensions.width,
                        text: widget_node_data.text.to_owned(),
                        outbound_edges: HashMap::new(),
                        widget_type: match &widget_node_data.kind {
                            WidgetKind::Basic(BasicWidgetKind::Block) => WidgetBlock,
                            WidgetKind::Basic(BasicWidgetKind::TextInput) => WidgetTextInput,
                            WidgetKind::Basic(BasicWidgetKind::Dummy) => DummyEdgeHandle,
                            WidgetKind::Command(cfg) => {
                                // command_name should be always exist for command widget
                                if let Some(command_name) = &widget_node_data.command_name {
                                    let command_view_map = commands_view_map();
                                    let command_view =
                                        command_view_map.get(command_name.as_str()).unwrap();
                                    command_view.view_type()
                                } else {
                                    NodeViewType::Print // TODO: FIXME :wrong default
                                }
                            }
                            WidgetKind::Context(_) => todo!(),
                            // WidgetKind::Context(_) => todo!(),
                        },
                        flow_inbound_edges: vec![],
                        flow_outbound_edges: vec![],
                        run_state: match state.model().run_status.get(node_id) {
                            Some(v) => match v.value().0 {
                                RunState::WaitingInputs => RunStateView::WaitingInputs,
                                RunState::Running => RunStateView::Running,
                                RunState::Failed(_, _) => RunStateView::Failed,
                                RunState::Success(_) => RunStateView::Success,
                                RunState::Canceled => RunStateView::Canceled,
                            },
                            None => RunStateView::NotRunning,
                        },
                        elapsed_time: match state.model().run_status.get(node_id) {
                            Some(v) => match v.value().0 {
                                RunState::Failed(t, _) => t,
                                RunState::Success(t) => t,
                                _ => 0,
                            },
                            None => 0,
                        },
                        error: match state.model().run_status.get(node_id) {
                            Some(v) => match &v.value().0 {
                                RunState::Failed(_, e) => e.clone(),
                                _ => String::new(),
                            },
                            None => String::new(),
                        },
                        print_output: match state.model().run_status.get(node_id) {
                            Some(v) => match v.value().1 {
                                Some(ref v) => v.clone(),
                                None => "waiting".to_owned(),
                            },
                            None => String::new(),
                        },
                        additional_data: widget_node_data.additional_data.to_owned(),
                        required: bool::default(),
                        tooltip: String::new(),
                        type_bounds: String::new(),
                        passthrough: bool::default(),
                        default_value: String::new(),
                        has_default: bool::default(),
                    },
                )
            });

        let input_nodes = state.model().inputs().iter().map(|(input_id, input)| {
            let parent_node_id = input.parent_node_id;
            let input_node = state.model().nodes().get(&parent_node_id).unwrap();
            //dbg!(input.clone());
            let input_data = match input_node {
                NodeModel::Widget(data) => data,
                // NodeModel::Data(_) => panic!(),
            };
            (
                input_id.0.to_string(),
                NodeView {
                    index: input.index,
                    parent_id: input.command_id.0.to_string(),
                    origin_x: (input_data.coords.x + input.local_coords.x) as i64,
                    origin_y: (input_data.coords.y + input.local_coords.y) as i64,
                    x: (input_data.coords.x + input.local_coords.x) as i64,
                    y: (input_data.coords.y + input.local_coords.y) as i64,
                    height: INPUT_SIZE,
                    width: INPUT_SIZE,
                    text: input.label.to_owned(),
                    outbound_edges: HashMap::new(),
                    widget_type: NodeViewType::WidgetInput,
                    flow_outbound_edges: vec![],
                    flow_inbound_edges: state
                        .model()
                        .flow_edges()
                        .iter()
                        .filter(|(_, edge)| &edge.input_id == input_id)
                        .map(
                            |(edge_id, _flow_edge_model)| {
                                edge_id.0.to_string()
                                //flow_edge_model.input_id.0.to_string() // FIXME: should be the edge id, not the input id
                            }, /* edge_id.0.to_string() */
                        )
                        .collect(),
                    run_state: RunStateView::NotRunning,
                    error: String::new(),
                    print_output: String::new(),
                    elapsed_time: 0,
                    additional_data: String::new(),
                    required: input.required,
                    tooltip: input.tooltip.to_owned(),
                    type_bounds: input.type_bounds.to_owned(),
                    passthrough: bool::default(),
                    default_value: input.default_value.to_owned(),
                    has_default: input.has_default.to_owned(),
                },
            )
        });

        let output_node = state.model().outputs().iter().map(|(output_id, output)| {
            let parent_node_id = output.parent_node_id;
            let output_node = state.model().nodes().get(&parent_node_id).unwrap();
            let output_data = match output_node {
                NodeModel::Widget(data) => data,
                // NodeModel::Data(_) => panic!(),
            };
            (
                output_id.0.to_string(),
                NodeView {
                    index: output.index,
                    parent_id: output.command_id.0.to_string(),
                    origin_x: (output_data.coords.x + output.local_coords.x) as i64,
                    origin_y: (output_data.coords.y + output.local_coords.y) as i64,
                    x: (output_data.coords.x + output.local_coords.x) as i64,
                    y: (output_data.coords.y + output.local_coords.y) as i64,
                    height: INPUT_SIZE,
                    width: INPUT_SIZE,
                    text: output.label.to_owned(),
                    outbound_edges: HashMap::new(),
                    widget_type: NodeViewType::WidgetOutput,
                    flow_outbound_edges: state
                        .model()
                        .flow_edges()
                        .iter()
                        .filter(|(_, edge)| &edge.output_id == output_id)
                        .map(
                            |(edge_id, flow_edge_model)| edge_id.0.to_string(), //flow_edge_model.output_id.0.to_string(), // FIXME: should be the edge id, not the input id
                        )
                        .collect(),
                    flow_inbound_edges: vec![],
                    run_state: RunStateView::NotRunning,
                    error: String::new(),
                    print_output: String::new(),
                    elapsed_time: 0,
                    additional_data: String::new(),
                    required: bool::default(),
                    tooltip: output.tooltip.to_owned(),
                    type_bounds: output.type_bound.to_owned(),
                    passthrough: output.passthrough,
                    default_value: String::default(),
                    has_default: bool::default(),
                },
            )
        });

        let mut node_views: HashMap<String, NodeView> =
            widget_nodes.chain(input_nodes).chain(output_node).collect();
        //dbg!(node_views.clone());

        // insert node edges
        for (edge_id, edge_model) in state.model().iter_node_edges() {
            let node_id = edge_model.from.0.to_string();

            let edge = EdgeView {
                from: node_id.clone(),
                to: edge_model.to.0.to_string(),
                edge_type: match edge_model.data.edge_type {
                    model::EdgeType::Child => ViewEdgeType::Child,
                    model::EdgeType::Data => ViewEdgeType::Data,
                    model::EdgeType::Flow => ViewEdgeType::Flow,
                },
                from_coords_x: match edge_model.data.from_coords {
                    Some(coords) => coords.x as i64,
                    None => 0,
                },
                from_coords_y: match edge_model.data.from_coords {
                    Some(coords) => coords.y as i64,
                    None => 0,
                },
                to_coords_x: match edge_model.data.to_coords {
                    Some(coords) => coords.x as i64,
                    None => 0,
                },
                to_coords_y: match edge_model.data.to_coords {
                    Some(coords) => coords.y as i64,
                    None => 0,
                },
            };

            // TODO: always add for widget nodes
            if let Some(node) = node_views.get_mut(&node_id) {
                node.outbound_edges.insert(edge_id.0.to_string(), edge);
            }
        }

        let node_ids: HashSet<NodeId> = state.selected_node_ids().copied().collect();
        // move currently movable nodes
        let (dx, dy) = if let UiState::NodeMove(start_coords, coords) = state.ui_state {
            let dx = coords.x - start_coords.x;
            let dy = coords.y - start_coords.y;

            for node_id in &node_ids {
                let node_id_str = node_id.0.to_string();
                let node_view = node_views.get_mut(&node_id_str).unwrap();

                node_view.x += dx as i64;
                node_view.y += dy as i64;
            }
            (dx, dy)
        } else {
            (0.0, 0.0)
        };

        //self.view.flow_edges.clear();
        let mut flow_edges = HashMap::new();
        // insert input/output edges
        for (edge_id, edge) in state.model().flow_edges() {
            let input = state.model().inputs().get(&edge.input_id).unwrap();
            let output = state.model().outputs().get(&edge.output_id).unwrap();

            let input_node = state.model().nodes().get(&input.parent_node_id).unwrap();
            let input_data = match input_node {
                NodeModel::Widget(data) => data,
                // NodeModel::Data(_) => panic!(),
            };
            let output_node = state.model().nodes().get(&output.parent_node_id).unwrap();
            let output_data = match output_node {
                NodeModel::Widget(data) => data,
                // NodeModel::Data(_) => panic!(),
            };

            //let node_view = node_views
            //    .get_mut(&input.parent_node_id.0.to_string())
            //   .unwrap();

            let (dx1, dy1) = if node_ids.contains(&output.parent_node_id) {
                (dx, dy)
            } else {
                (0.0, 0.0)
            };

            let (dx2, dy2) = if node_ids.contains(&input.parent_node_id) {
                (dx, dy)
            } else {
                (0.0, 0.0)
            };

            // node_view.outbound_edges
            flow_edges.insert(
                edge_id.0.to_string(),
                EdgeView {
                    from: edge.output_id.0.to_string(), // FIXME: input.label.clone(),
                    to: edge.input_id.0.to_string(),    // FIXME: output.label.clone(),
                    edge_type: ViewEdgeType::Flow,
                    from_coords_x: (output_data.coords.x + output.local_coords.x + dx1) as i64 + 35,
                    from_coords_y: (output_data.coords.y + output.local_coords.y + dy1) as i64
                        + INPUT_SIZE / 2, //half width of input size
                    to_coords_x: (input_data.coords.x + input.local_coords.x + dx2) as i64 + 15,
                    to_coords_y: (input_data.coords.y + input.local_coords.y + dy2) as i64
                        + INPUT_SIZE / 2,
                },
            );
        }

        let mut highlighted = vec![];

        // SELECTED NODE IDS
        let selected_node_ids: Vec<String> = self
            .state
            .as_ref()
            .unwrap()
            .selected_node_ids()
            .map(|uuid| uuid.0.to_string())
            .collect();
        // dbg!(selected_node_ids.clone());

        //child id, not really command
        let selected_command_ids: Vec<String> = self
            .state
            .as_ref()
            .unwrap()
            .model()
            .node_edges()
            .iter()
            .filter(|(_, model)| selected_node_ids.contains(&model.from.0.to_string()))
            .map(|(_, model)| model.to.0.to_string())
            .collect();
        // dbg!(selected_command_ids.clone());

        // Add currently creatable edge
        //
        if let UiState::Edge(input_id, coords) = state.ui_state {
            const DUMMY_EDGE_ID: &'static str = "dummy_edge";
            const DUMMY_NODE_ID: &'static str = "dummy_node";

            let commands_map = commands_map();
            // UPDATE when adding commandse
            let command_by_command_name =
                |command_name: &str| commands_map.get(command_name).unwrap();

            match input_id {
                PortId::Input(input_id) => {
                    let input = state.model().inputs().get(&input_id).unwrap();
                    let node_id = input.parent_node_id;
                    let input_node = state.model().nodes().get(&node_id).unwrap();
                    // dbg!(input_node);
                    let input_data = match input_node {
                        NodeModel::Widget(data) => data,
                        // NodeModel::Data(_) => panic!(),
                    };

                    flow_edges.insert(
                        DUMMY_EDGE_ID.to_owned(),
                        EdgeView {
                            from: DUMMY_NODE_ID.to_owned(),
                            to: input_id.0.to_string(), //node_id.0.to_string(),
                            edge_type: ViewEdgeType::Flow,
                            // +15 +25 is adjustment for offset port and edge in flutter dragging
                            from_coords_x: coords.x as i64,
                            from_coords_y: coords.y as i64,
                            to_coords_x: (input_data.coords.x + input.local_coords.x) as i64 + 15,
                            to_coords_y: (input_data.coords.y + input.local_coords.y) as i64
                                + INPUT_SIZE / 2,
                        },
                    );

                    node_views
                        .get_mut(&node_id.0.to_string())
                        .unwrap()
                        .flow_inbound_edges
                        .push(DUMMY_EDGE_ID.to_owned());

                    node_views.insert(
                        DUMMY_NODE_ID.to_owned(),
                        NodeView {
                            index: 0,
                            parent_id: "".to_owned(),
                            origin_x: coords.x as i64,
                            origin_y: coords.y as i64,
                            x: coords.x as i64,
                            y: coords.y as i64,
                            width: 0,
                            height: 0,
                            text: "".to_owned(),
                            outbound_edges: HashMap::new(),
                            widget_type: NodeViewType::DummyEdgeHandle,
                            flow_inbound_edges: vec![],
                            flow_outbound_edges: vec![DUMMY_EDGE_ID.to_owned()],
                            run_state: RunStateView::NotRunning,
                            error: String::new(),
                            print_output: String::new(),
                            elapsed_time: 0,
                            additional_data: String::new(),
                            required: bool::default(),
                            tooltip: String::new(),
                            type_bounds: String::new(),
                            passthrough: bool::default(),
                            default_value: String::new(),
                            has_default: bool::default(),
                        },
                    );

                    // get command_node_id
                    let command_node_id = input.command_id;

                    let command_node = state.model().nodes().get(&command_node_id).unwrap();
                    //dbg!(command_node);
                    let input_data = match command_node {
                        NodeModel::Widget(data) => data,
                        // NodeModel::Data(_) => panic!(),
                    };

                    //dbg!(&input_data.kind);
                    let input_command = input_data
                        .command_name
                        .as_ref()
                        .map(|name| command_by_command_name(&name)); // <-- CommandBlock data
                                                                     //dbg!(&input_command);
                    let command_input = input_command.and_then(|input_command| {
                        input_command
                            .inputs()
                            .iter()
                            .find(|command_input| command_input.name == input.label)
                    });
                    //dbg!(&command_input);

                    if let Some(command_input) = command_input {
                        highlighted.extend(
                            state
                                .model()
                                .outputs()
                                .iter()
                                .filter(|(_, output)| {
                                    if output.command_id == input.command_id {
                                        return false;
                                    }
                                    let output_node =
                                        state.model().nodes().get(&output.command_id).unwrap();
                                    let output_data = match output_node {
                                        NodeModel::Widget(data) => data,
                                    };
                                    //dbg!(&output_data.kind);
                                    let output_command = output_data
                                        .command_name
                                        .as_ref()
                                        .map(|name| command_by_command_name(&name));

                                    // dbg!(&output_command);
                                    let command_output =
                                        output_command.and_then(|output_command| {
                                            output_command.outputs().iter().find(|command_output| {
                                                command_output.name == output.label
                                            })
                                        });
                                    //dbg!(&command_output);
                                    if let Some(command_output) = command_output {
                                        // dbg!(
                                        //     command_input.acceptable_types(),
                                        //     command_output.r#type
                                        // );
                                        command_input
                                            .acceptable_types()
                                            .contains(&command_output.r#type)
                                    } else {
                                        false
                                    }
                                })
                                .map(|(output_id, _)| output_id.0.to_string()),
                        );
                    }
                }
                PortId::Output(output_id) => {
                    let output = state.model().outputs().get(&output_id).unwrap();
                    let node_id = output.parent_node_id;
                    let output_node = state.model().nodes().get(&node_id).unwrap();
                    let output_data = match output_node {
                        NodeModel::Widget(data) => data,
                        // NodeModel::Data(_) => panic!(),
                    };

                    node_views
                        .get_mut(&node_id.0.to_string())
                        .unwrap()
                        .flow_outbound_edges
                        .push(DUMMY_EDGE_ID.to_owned());

                    node_views.insert(
                        DUMMY_NODE_ID.to_owned(),
                        NodeView {
                            index: i64::default(),
                            parent_id: "".to_owned(),
                            origin_x: coords.x as i64,
                            origin_y: coords.y as i64,
                            x: coords.x as i64,
                            y: coords.y as i64,
                            width: 100,
                            height: 100,
                            text: "".to_owned(),
                            outbound_edges: HashMap::new(),
                            widget_type: NodeViewType::DummyEdgeHandle,
                            flow_inbound_edges: vec![DUMMY_EDGE_ID.to_owned()],
                            flow_outbound_edges: vec![],
                            run_state: RunStateView::NotRunning,
                            error: String::new(),
                            print_output: String::new(),
                            elapsed_time: 0,
                            additional_data: String::new(),
                            required: bool::default(),
                            tooltip: String::new(),
                            type_bounds: String::new(),
                            passthrough: bool::default(),
                            default_value: String::new(),
                            has_default: bool::default(),
                        },
                    );

                    flow_edges.insert(
                        DUMMY_EDGE_ID.to_owned(),
                        EdgeView {
                            to: DUMMY_NODE_ID.to_owned(),  // FIXME: "".to_owned(),
                            from: output_id.0.to_string(), // FIXME: output.label.clone(),
                            edge_type: ViewEdgeType::Flow,
                            from_coords_x: (output_data.coords.x + output.local_coords.x) as i64
                                + 35,
                            from_coords_y: (output_data.coords.y + output.local_coords.y) as i64
                                + 25,
                            // +35 +25 is adjustment for offset port and edge in flutter dragging
                            to_coords_x: coords.x as i64,
                            to_coords_y: coords.y as i64,
                        },
                    );

                    // get command_node_id
                    let command_node_id = output.command_id;

                    let command_node = state.model().nodes().get(&command_node_id).unwrap();
                    // dbg!(command_node);
                    let output_data = match command_node {
                        NodeModel::Widget(data) => data,
                        // NodeModel::Data(_) => panic!(),
                    };
                    // dbg!(&output_data.kind);

                    let output_command = output_data
                        .command_name
                        .as_ref()
                        .map(|name| command_by_command_name(&name));
                    // dbg!(&output_command);

                    let command_output = output_command.and_then(|output_command| {
                        output_command
                            .outputs()
                            .iter()
                            .find(|command_output| command_output.name == output.label)
                    });
                    // dbg!(&command_output);

                    if let Some(command_output) = command_output {
                        highlighted.extend(
                            state
                                .model()
                                .inputs()
                                .iter()
                                .filter(|(_, input)| {
                                    if output.command_id == input.command_id {
                                        return false;
                                    }
                                    let input_node =
                                        state.model().nodes().get(&input.command_id).unwrap();
                                    let input_data = match input_node {
                                        NodeModel::Widget(data) => data,
                                    };
                                    // dbg!(&input_data.kind);
                                    let input_command = input_data
                                        .command_name
                                        .as_ref()
                                        .map(|name| command_by_command_name(&name));

                                    // dbg!(&input_command);
                                    let command_input = input_command.and_then(|input_command| {
                                        input_command
                                            .inputs()
                                            .iter()
                                            .find(|command_input| command_input.name == input.label)
                                    });
                                    // dbg!(&command_input);
                                    if let Some(command_input) = command_input {
                                        // dbg!(
                                        //     command_input.acceptable_types(),
                                        //     command_output.r#type
                                        // );
                                        command_input
                                            .acceptable_types()
                                            .contains(&command_output.r#type)
                                    } else {
                                        false
                                    }
                                })
                                .map(|(input_id, _)| input_id.0.to_string()),
                        );
                    }
                }
            }
        }

        let transform = Self::get_view_tranform(state);
        let transform_screenshot = Self::get_view_tranform_screenshot(state);

        // self.view.flow_edges = flow_edges.clone();
        // dbg!(self.view.flow_edges.clone());
        let nodes = node_views;

        let text_commands = self.view.text_commands.clone();

        // SET OLD VIEW
        let old_view = &self.view;

        // GRAPH ID & GRAPH ENTRY
        let graph_id = state.model().graph_id();

        let graph_entry = state.model().get_graph_entry(graph_id);

        // GRAPH LIST
        //  let graph_list = self.view.graph_list.clone();
        let graph_list = state.model().graph_list.clone(); // FIXME: update first in view in updated in model

        let bookmarks = &state.model().bookmarks;
        let bookmarks = bookmarks
            .iter()
            .map(|(bookmark_id, bookmark)| {
                (
                    bookmark_id.0.to_string(),
                    BookmarkView {
                        name: bookmark.name.clone(),
                        nodes: bookmark
                            .nodes_ids
                            .iter()
                            .map(|node_id| node_id.0.to_string())
                            .collect(),
                    },
                )
            })
            .collect();

        let new_view = View {
            graph_entry: graph_entry.clone(),
            nodes,
            flow_edges,
            selected_node_ids,
            selected_command_ids,
            selection,
            command,
            text_commands,
            graph_list,
            highlighted, //does not show updates
            transform,
            transform_screenshot,
            bookmarks,
            solana_net: state.model().solana_net,
            ui_state_debug: DebugData {
                ui_state: serde_json::to_string(&state.ui_state).unwrap(),
                mapping_kind: serde_json::to_string(&state.mapping_kind).unwrap(),
                selected_node_ids: serde_json::to_string(&state.selected_node_ids).unwrap(),
            },
        };

        let node_ids: HashSet<_> = old_view.nodes.keys().chain(new_view.nodes.keys()).collect();
        let changed_nodes_ids: HashMap<String, NodeChange> = node_ids
            .into_iter()
            .filter_map(|node_id| {
                match (
                    old_view.nodes.get(node_id.as_str()),
                    new_view.nodes.get(node_id.as_str()),
                ) {
                    (Some(old_node), Some(new_node)) => {
                        if old_node == new_node {
                            None
                        } else {
                            Some((
                                node_id.clone(),
                                NodeChange {
                                    kind: view::NodeChangeKind::Modified,
                                },
                            ))
                        }
                    }
                    (None, Some(new_node)) => Some((
                        node_id.clone(),
                        NodeChange {
                            kind: view::NodeChangeKind::Added,
                        },
                    )),
                    (Some(old_node), None) => Some((
                        node_id.clone(),
                        NodeChange {
                            kind: view::NodeChangeKind::Removed,
                        },
                    )),
                    (None, None) => unreachable!(),
                }
            })
            .collect();

        let flow_edges_ids: HashSet<_> = old_view
            .flow_edges
            .keys()
            .chain(new_view.flow_edges.keys())
            .collect();

        let changed_flow_edges_ids = flow_edges_ids
            .into_iter()
            .filter(|node_id| {
                old_view.flow_edges.get(node_id.as_str())
                    != new_view.flow_edges.get(node_id.as_str())
            })
            .map(Clone::clone)
            .collect();

        let is_selected_node_ids_changed = old_view.selected_node_ids != new_view.selected_node_ids;
        let is_selection_changed = old_view.selection != new_view.selection;
        let is_command_changed = old_view.command != new_view.command;
        let is_text_commands_changed = old_view.text_commands != new_view.text_commands;
        let is_graph_list_changed = old_view.graph_list != new_view.graph_list;
        let is_highlighted_changed = old_view.highlighted != new_view.highlighted;
        let is_transform_changed = old_view.transform != new_view.transform;
        let is_graph_changed = old_view.graph_entry != new_view.graph_entry;
        let is_bookmark_changed = old_view.bookmarks != new_view.bookmarks;
        let is_transform_screenshot_changed =
            old_view.transform_screenshot != new_view.transform_screenshot;

        let changes = LastViewChanges {
            changed_nodes_ids,
            changed_flow_edges_ids,
            is_selected_node_ids_changed,
            is_selection_changed,
            is_command_changed,
            is_text_commands_changed,
            is_graph_list_changed,
            is_highlighted_changed,
            is_transform_changed,
            is_transform_screenshot_changed,
            is_graph_changed,
            is_bookmark_changed,
        };
        self.view = new_view;
        self.last_view_changes = changes;

        //dbg!(&self.view.nodes);
        //dbg!(&self.view.flow_edges);
        //dbg!(&self.last_view_changes.changed_flow_edges_ids);
        //dbg!(&self.view.highlighted);
        // dbg!(&self.last_view_changes);
        //dbg!(&state.transform);
        //dbg!(&self.view.transform);
    }

    fn refresh_ui_transform(&mut self) {
        let state = self.state.as_ref().unwrap();
        self.view.transform = Self::get_view_tranform(state);
        self.last_view_changes = LastViewChanges {
            changed_nodes_ids: HashMap::new(),
            changed_flow_edges_ids: vec![],
            is_selected_node_ids_changed: false,
            is_selection_changed: false,
            is_command_changed: false,
            is_text_commands_changed: false,
            is_graph_list_changed: false,
            is_highlighted_changed: false,
            is_transform_changed: true,
            is_transform_screenshot_changed: false,
            is_graph_changed: false,
            is_bookmark_changed: false, // FIXME
        };
        /*println!(
            "transform {:10} {:10} {:10}",
            &state.transform.x, &state.transform.y, &state.transform.scale
        );*/
    }

    fn refresh_ui_flow_edges(&mut self) {
        let state = self.state.as_ref().unwrap();
        let mut changes = LastViewChanges::default();

        if let UiState::Edge(input_id, coords) = state.ui_state {
            const DUMMY_EDGE_ID: &'static str = "dummy_edge";
            const DUMMY_NODE_ID: &'static str = "dummy_node";

            let commands_map = commands_map();
            // UPDATE when adding commandse
            let command_by_command_name =
                |command_name: &str| commands_map.get(command_name).unwrap();

            match input_id {
                PortId::Input(input_id) => {
                    let input = state.model().inputs().get(&input_id).unwrap();
                    let node_id = input.parent_node_id;
                    let input_node = state.model().nodes().get(&node_id).unwrap();
                    // dbg!(input_node);
                    let input_data = match input_node {
                        NodeModel::Widget(data) => data,
                        // NodeModel::Data(_) => panic!(),
                    };

                    changes
                        .changed_flow_edges_ids
                        .push(DUMMY_EDGE_ID.to_owned());
                    self.view.flow_edges.insert(
                        DUMMY_EDGE_ID.to_owned(),
                        EdgeView {
                            from: DUMMY_NODE_ID.to_owned(),
                            to: input_id.0.to_string(), //node_id.0.to_string(),
                            edge_type: ViewEdgeType::Flow,
                            // +15 +25 is adjustment for offset port and edge in flutter dragging
                            from_coords_x: coords.x as i64,
                            from_coords_y: coords.y as i64,
                            to_coords_x: (input_data.coords.x + input.local_coords.x) as i64 + 15,
                            to_coords_y: (input_data.coords.y + input.local_coords.y) as i64
                                + INPUT_SIZE / 2,
                        },
                    );

                    changes.changed_nodes_ids.insert(
                        node_id.0.to_string(),
                        NodeChange {
                            kind: NodeChangeKind::Modified,
                        },
                    );
                    self.view
                        .nodes
                        .get_mut(&node_id.0.to_string())
                        .unwrap()
                        .flow_inbound_edges
                        .push(DUMMY_EDGE_ID.to_owned());

                    changes.changed_nodes_ids.insert(
                        DUMMY_NODE_ID.to_owned(),
                        NodeChange {
                            kind: NodeChangeKind::Modified,
                        },
                    );
                    self.view.nodes.insert(
                        DUMMY_NODE_ID.to_owned(),
                        NodeView {
                            index: 0,
                            parent_id: "".to_owned(),
                            origin_x: coords.x as i64,
                            origin_y: coords.y as i64,
                            x: coords.x as i64,
                            y: coords.y as i64,
                            width: 0,
                            height: 0,
                            text: "".to_owned(),
                            outbound_edges: HashMap::new(),
                            widget_type: NodeViewType::DummyEdgeHandle,
                            flow_inbound_edges: vec![],
                            flow_outbound_edges: vec![DUMMY_EDGE_ID.to_owned()],
                            run_state: RunStateView::NotRunning,
                            error: String::new(),
                            print_output: String::new(),
                            elapsed_time: 0,
                            additional_data: String::new(),
                            required: bool::default(),
                            tooltip: String::new(),
                            type_bounds: String::new(),
                            passthrough: bool::default(),
                            default_value: String::new(),
                            has_default: bool::default(),
                        },
                    );

                    // get command_node_id
                    let command_node_id = input.command_id;

                    let command_node = state.model().nodes().get(&command_node_id).unwrap();
                    //dbg!(command_node);
                    let input_data = match command_node {
                        NodeModel::Widget(data) => data,
                        // NodeModel::Data(_) => panic!(),
                    };

                    //dbg!(&input_data.kind);
                    let input_command = input_data
                        .command_name
                        .as_ref()
                        .map(|name| command_by_command_name(&name)); // <-- CommandBlock data
                                                                     //dbg!(&input_command);
                    let command_input = input_command.and_then(|input_command| {
                        input_command
                            .inputs()
                            .iter()
                            .find(|command_input| command_input.name == input.label)
                    });
                    //dbg!(&command_input);

                    if let Some(command_input) = command_input {
                        changes.is_highlighted_changed = true;
                        self.view.highlighted.extend(
                            state
                                .model()
                                .outputs()
                                .iter()
                                .filter(|(_, output)| {
                                    if output.command_id == input.command_id {
                                        return false;
                                    }
                                    let output_node =
                                        state.model().nodes().get(&output.command_id).unwrap();
                                    let output_data = match output_node {
                                        NodeModel::Widget(data) => data,
                                    };
                                    //dbg!(&output_data.kind);
                                    let output_command = output_data
                                        .command_name
                                        .as_ref()
                                        .map(|name| command_by_command_name(&name));

                                    // dbg!(&output_command);
                                    let command_output =
                                        output_command.and_then(|output_command| {
                                            output_command.outputs().iter().find(|command_output| {
                                                command_output.name == output.label
                                            })
                                        });
                                    //dbg!(&command_output);
                                    if let Some(command_output) = command_output {
                                        // dbg!(
                                        //     command_input.acceptable_types(),
                                        //     command_output.r#type
                                        // );
                                        command_input
                                            .acceptable_types()
                                            .contains(&command_output.r#type)
                                    } else {
                                        false
                                    }
                                })
                                .map(|(output_id, _)| output_id.0.to_string()),
                        );
                    }
                }
                PortId::Output(output_id) => {
                    let output = state.model().outputs().get(&output_id).unwrap();
                    let node_id = output.parent_node_id;
                    let output_node = state.model().nodes().get(&node_id).unwrap();
                    let output_data = match output_node {
                        NodeModel::Widget(data) => data,
                        // NodeModel::Data(_) => panic!(),
                    };

                    changes.changed_nodes_ids.insert(
                        node_id.0.to_string(),
                        NodeChange {
                            kind: NodeChangeKind::Modified,
                        },
                    );
                    self.view
                        .nodes
                        .get_mut(&node_id.0.to_string())
                        .unwrap()
                        .flow_outbound_edges
                        .push(DUMMY_EDGE_ID.to_owned());

                    changes.changed_nodes_ids.insert(
                        DUMMY_NODE_ID.to_owned(),
                        NodeChange {
                            kind: NodeChangeKind::Modified,
                        },
                    );
                    self.view.nodes.insert(
                        DUMMY_NODE_ID.to_owned(),
                        NodeView {
                            index: i64::default(),
                            parent_id: "".to_owned(),
                            origin_x: coords.x as i64,
                            origin_y: coords.y as i64,
                            x: coords.x as i64,
                            y: coords.y as i64,
                            width: 100,
                            height: 100,
                            text: "".to_owned(),
                            outbound_edges: HashMap::new(),
                            widget_type: NodeViewType::DummyEdgeHandle,
                            flow_inbound_edges: vec![DUMMY_EDGE_ID.to_owned()],
                            flow_outbound_edges: vec![],
                            run_state: RunStateView::NotRunning,
                            error: String::new(),
                            print_output: String::new(),
                            elapsed_time: 0,
                            additional_data: String::new(),
                            required: bool::default(),
                            tooltip: String::new(),
                            type_bounds: String::new(),
                            passthrough: bool::default(),
                            default_value: String::new(),
                            has_default: bool::default(),
                        },
                    );

                    changes
                        .changed_flow_edges_ids
                        .push(DUMMY_EDGE_ID.to_owned());
                    self.view.flow_edges.insert(
                        DUMMY_EDGE_ID.to_owned(),
                        EdgeView {
                            to: DUMMY_NODE_ID.to_owned(),  // FIXME: "".to_owned(),
                            from: output_id.0.to_string(), // FIXME: output.label.clone(),
                            edge_type: ViewEdgeType::Flow,
                            from_coords_x: (output_data.coords.x + output.local_coords.x) as i64
                                + 35,
                            from_coords_y: (output_data.coords.y + output.local_coords.y) as i64
                                + 25,
                            // +35 +25 is adjustment for offset port and edge in flutter dragging
                            to_coords_x: coords.x as i64,
                            to_coords_y: coords.y as i64,
                        },
                    );

                    // get command_node_id
                    let command_node_id = output.command_id;

                    let command_node = state.model().nodes().get(&command_node_id).unwrap();
                    // dbg!(command_node);
                    let output_data = match command_node {
                        NodeModel::Widget(data) => data,
                        // NodeModel::Data(_) => panic!(),
                    };
                    // dbg!(&output_data.kind);

                    let output_command = output_data
                        .command_name
                        .as_ref()
                        .map(|name| command_by_command_name(&name));
                    // dbg!(&output_command);

                    let command_output = output_command.and_then(|output_command| {
                        output_command
                            .outputs()
                            .iter()
                            .find(|command_output| command_output.name == output.label)
                    });
                    // dbg!(&command_output);

                    if let Some(command_output) = command_output {
                        changes.is_highlighted_changed = true;
                        self.view.highlighted.extend(
                            state
                                .model()
                                .inputs()
                                .iter()
                                .filter(|(_, input)| {
                                    if output.command_id == input.command_id {
                                        return false;
                                    }
                                    let input_node =
                                        state.model().nodes().get(&input.command_id).unwrap();
                                    let input_data = match input_node {
                                        NodeModel::Widget(data) => data,
                                    };
                                    // dbg!(&input_data.kind);
                                    let input_command = input_data
                                        .command_name
                                        .as_ref()
                                        .map(|name| command_by_command_name(&name));

                                    // dbg!(&input_command);
                                    let command_input = input_command.and_then(|input_command| {
                                        input_command
                                            .inputs()
                                            .iter()
                                            .find(|command_input| command_input.name == input.label)
                                    });
                                    // dbg!(&command_input);
                                    if let Some(command_input) = command_input {
                                        // dbg!(
                                        //     command_input.acceptable_types(),
                                        //     command_output.r#type
                                        // );
                                        command_input
                                            .acceptable_types()
                                            .contains(&command_output.r#type)
                                    } else {
                                        false
                                    }
                                })
                                .map(|(input_id, _)| input_id.0.to_string()),
                        );
                    }
                }
            }
        }
        self.last_view_changes = changes;
    }

    fn refresh_ui_transform_screenshot(&mut self) {
        let state = self.state.as_ref().unwrap();
        self.view.transform_screenshot = Self::get_view_tranform_screenshot(state);
        self.last_view_changes = LastViewChanges {
            changed_nodes_ids: HashMap::new(),
            changed_flow_edges_ids: vec![],
            is_selected_node_ids_changed: false,
            is_selection_changed: false,
            is_command_changed: false,
            is_text_commands_changed: false,
            is_graph_list_changed: false,
            is_highlighted_changed: false,
            is_transform_changed: false,
            is_transform_screenshot_changed: true,
            is_graph_changed: false,
            is_bookmark_changed: false, // FIXME
        };
        /*println!(
            "transform {:10} {:10} {:10}",
            &state.transform.x, &state.transform.y, &state.transform.scale
        );*/
    }

    fn get_view_tranform(state: &State) -> crate::view::Camera {
        crate::view::Camera {
            x: Ratio::from(state.transform.x),
            y: Ratio::from(state.transform.y),
            scale: Ratio::from(state.transform.scale),
        }
    }

    fn get_view_tranform_screenshot(state: &State) -> crate::view::Camera {
        crate::view::Camera {
            x: Ratio::from(state.transform_screenshot.x),
            y: Ratio::from(state.transform_screenshot.y),
            scale: Ratio::from(state.transform_screenshot.scale),
        }
    }
}
