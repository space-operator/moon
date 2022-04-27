use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::model::{InputId, NodeId, OutputId, PortId};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Unselect,
    SelectNode(NodeId),
    AddOrRemoveNodeToSelection(NodeId),
    CreateNode(Coords),
    EditNode(NodeId),
    RemoveNodes(HashSet<NodeId>),
    //
    MaybeStartSelection(Coords),
    NotASelection,
    StartSelection(Coords, Coords),
    ContinueSelection(Coords, Coords), // start_coords, new_coords
    EndSelection(Coords, Coords),      // start_coords, new_coords
    CancelSelection,
    //
    MaybeStartTransformMove(Coords),
    NotATransformMove,
    StartTransformMove(Coords, Coords), // start_cords, new_coords
    ContinueTransformMove(Coords, Coords), // start_cords, new_coords
    EndTransformMove(Coords, Coords),   // start_cords, new_coords
    CancelTransformMove,
    //
    MaybeStartNodeMove(NodeId, Coords),
    NotANodeMove,
    StartNodeMove(NodeId, Coords, Coords),
    ContinueNodeMove(Coords, Coords),
    EndNodeMove(Coords, Coords),
    CancelNodeMove,
    //
    MaybeStartEdge(PortId),
    NotAEdge,
    StartEdge(PortId, Coords),
    ContinueEdge(PortId, Coords),
    EndEdge(InputId, OutputId),
    CancelEdge(PortId),
    //
    // x, y, multiplier
    ScrollZoom(f64, f64, f64),
    ScrollMoveScreen(f64, f64),
    //
    //StartCommandInput(String),
    //ModifyCommandInput(String),
    //ApplyCommandInput(String),
    //CancelCommandInput
}
