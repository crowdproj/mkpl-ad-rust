pub mod cor_closure_step;
pub mod cor_context;
pub mod cor_enums;
pub mod cor_error;
pub mod cor_step;
pub mod cor_struct;
pub mod cor_types;

#[macro_use]
pub mod macros;

use std::sync::{Arc, Mutex};

pub type CtxWrapper<C> = Arc<Mutex<C>>;

pub use cor_closure_step::*;
pub use cor_context::*;
pub use cor_enums::*;
pub use cor_error::*;
pub use cor_step::*;
pub use cor_struct::*;
pub use cor_types::*;
