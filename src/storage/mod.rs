// TODO: add other subscribe functions to indexes
// TODO: or add &storage and &mut storage getters
// TODO: but without exposing put or remove

// TODO: make generic indexes and simplify their declaration

mod model;
mod model_index;
mod raw_storage;

pub use model::*;
pub use model_index::*;
pub use raw_storage::*;
