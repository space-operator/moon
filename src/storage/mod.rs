// TODO: macro for simple module definition
// TODO: wrappers for easy graph operation
// TODO: we want to replicate the functionality in this project: https://heptabase.com/

// MAYBE TODO: add auto-indexes for storage with Weak in storage and Arc in index
// DECLINED: because too complex basic bahavior

// MAYBE TODO: ValueRef mutate with check is really changed
// MAYBE TODO: ValueRef on drop or consume send changes to subscribers
// DECLINED: because we should send both old and new values to subscribers

mod change;
mod index;
mod macros;
mod model;
mod model_index;
mod read_only_storage;
mod storage;

pub use change::*;
pub use index::*;
pub use macros::*;
pub use model::*;
pub use model_index::*;
pub use read_only_storage::*;
pub use storage::*;
