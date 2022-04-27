use std::collections::HashSet;

use serde::Serialize;

use crate::command::commands_map;
use crate::event::Coords;
use crate::event::Event;
use crate::input::{CapturedLifetime, Context, Input, MappingKind};

use crate::model::NodeModel;
use crate::model::PortId;
use crate::model::WidgetKind;
use crate::model::{Model, NodeId};

#[derive(Debug)]
pub struct State {
    model: Model,
    input: Input,
    pub ui_state: UiState,
    pub selected_node_ids: HashSet<NodeId>,
    pub active_node: Option<NodeId>,
    pub transform: Transform,
    pub transform_screenshot: Transform,
    pub canvas: Canvas,
    pub mapping_kind: MappingKind,
    // pub req_id: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub x: f64,
    pub y: f64,
    pub scale: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Canvas {
    pub width: u64,
    pub height: u64,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale: 1.0,
        }
    }
}

impl State {
    pub fn new(db_path: String, log_path: String, canvas_width: u64, canvas_height: u64) -> Self {
        let model = Model::new(db_path, log_path);

        Self {
            model,
            input: Input::default(),
            ui_state: UiState::Default,
            selected_node_ids: HashSet::new(),
            active_node: None,
            transform: Transform::default(), // req_id: u64::default(),
            transform_screenshot: Transform::default(), // req_id: u64::default(),
            canvas: Canvas {
                width: canvas_width,
                height: canvas_height,
            },
            mapping_kind: MappingKind::Mouse,
        }
    }

    /// GETTERS
    ///

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn ui_state(&self) -> &UiState {
        &self.ui_state
    }

    pub fn model(&self) -> &Model {
        &self.model
    }

    pub fn model_mut(&mut self) -> &mut Model {
        &mut self.model
    }

    /// RESET STATE
    ///
    pub fn reset(&mut self) {
        self.clear_selection();
        self.ui_state = UiState::Default;
    }

    /// SELECTION

    pub fn selected_node_ids(&self) -> impl Iterator<Item = &NodeId> {
        self.selected_node_ids.iter()
    }

    pub fn clear_selection(&mut self) {
        self.selected_node_ids.clear();
        self.active_node = None;
        //dbg!(&self.selected_node_ids, self.active_node);
        // self.update_selection();
    }

    pub fn add_to_selection(&mut self, node_id: NodeId) {
        self.selected_node_ids.insert(node_id);
        self.active_node = Some(node_id);
        //dbg!(&self.selected_node_ids, self.active_node);
    }

    pub fn add_or_remove_from_selection(&mut self, node_id: NodeId) {
        if self.selected_node_ids.contains(&node_id) {
            self.selected_node_ids.remove(&node_id);
            if self.active_node == Some(node_id) { //FIXME removes active node and panics
                if self.selected_node_ids.clone().len()>0{

                    self.active_node = self.selected_node_ids.clone().into_iter().next(); //fixme 
                }else{
                    self.active_node =None;
                }

            }
            // dbg!(&self.selected_node_ids, self.active_node);
        } else {
            self.selected_node_ids.insert(node_id);
            self.active_node = Some(node_id);
            // dbg!(&self.selected_node_ids, self.active_node);
        }
    }

    /*pub fn update_active_node(&mut self, node_id: NodeId) {
        self.active_node = Some(node_id);
    }*/

    pub fn on_flutter_mouse_event<'a>(
        &'a mut self,
        msg: &str,
    ) -> impl CapturedLifetime<'a> + Iterator<Item = Event> {
        self.input.on_flutter_mouse_event(
            msg,
            Context {
                model: &self.model,
                transform: self.transform,
                ui_state: &self.ui_state,
                selected_node_ids: &self.selected_node_ids,
                mapping_kind: self.mapping_kind,
            },
        )
    }

    pub fn on_flutter_keyboard_event<'a>(
        &'a mut self,
        msg: &str,
    ) -> impl CapturedLifetime<'a> + Iterator<Item = Event> {
        self.input.on_flutter_keyboard_event(
            msg,
            Context {
                model: &self.model,
                transform: self.transform,
                ui_state: &self.ui_state,
                selected_node_ids: &self.selected_node_ids,
                mapping_kind: self.mapping_kind,
            },
        )
    }

    pub fn set_node_coords(&mut self, node_id: &NodeId, coords: Coords) {
        self.model.set_node_coords(node_id, coords)
    }

    pub fn apply_command(&mut self, node_id: &str, command_name: &str) -> Result<(), ()> {
        println!("apply command input: {}", &command_name);
        self.ui_state = UiState::Default;

        // get current node from selection
        // dbg!(
        //     self.selected_node_ids().collect::<Vec<_>>(),
        //     self.active_node
        // );
        let node_id = NodeId(node_id.parse().unwrap());
        //let node_id = self.active_node.unwrap();
        let node_model = self.model().nodes().get(&node_id).unwrap();

        // coords
        let coords = match node_model {
            NodeModel::Widget(widget_node_data) => widget_node_data.coords,
            // NodeModel::Data(_) => panic!(),
        };

        let commands_map = commands_map();
        let command = commands_map.get(command_name);
        if let Some(command) = command {
            let config = command.config();
            let widget_name = command.widget_name();
            let dimensions = command.dimensions().clone();

            let command_kind = WidgetKind::Command(config.clone());
            // ideally we should store command_name in CommandConfig (without option)
            self.model_mut().into_command_block(
                node_id,
                coords,
                command_name,
                command_kind,
                dimensions,
            );
            Ok(())
        } else {
            Err(())
        }

        //self.refresh_ui();
        //rid::post(Confirm::RefreshUI(req_id));
    }
}

#[derive(Clone, Debug, Serialize)]
pub enum UiState {
    Default,
    MaybeSelection(Coords),
    Selection(Coords, Coords),
    MaybeTransformMove(Coords),
    TransformMove(Coords),
    MaybeNodeMove(NodeId, Coords),
    NodeMove(Coords, Coords),
    MaybeEdge(PortId),
    Edge(PortId, Coords),
    UiInput,
    CommandInput(String),
}

impl Default for UiState {
    fn default() -> Self {
        Self::Default
    }
}
