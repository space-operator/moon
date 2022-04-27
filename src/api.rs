use serde::{Deserialize, Serialize};

use crate::input::FlutterPointerEvent;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum Request {
    Initialize(InitializeRequest),
    Mouse(FlutterPointerEvent),
    TextInput(InputRequest),
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum Response {
    ModifyText {
        node_id: String,
        text: String,
        cursor: u32,
    },
    StopTextEditing,
    RefreshUi,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InitializeRequest {
    path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputRequest {
    node_id: String,
    text: String,
    cursor: u32,
}

/*
pub trait HandleRequest {
    type Response;

    fn handle(self) -> Self::Response;
}

impl HandleRequest for InitializeRequest {
    type Response = ();

    fn handle(self) -> Self::Response {
        todo!();
    }
}

impl HandleRequest for FlutterPointerEvent {
    type Response = ();

    fn handle(self) -> Self::Response {
        todo!();
    }
}
*/
