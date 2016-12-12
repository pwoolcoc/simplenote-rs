//! Simplenote API
#![feature(proc_macro)]
extern crate reqwest;
extern crate base64;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
#[macro_use] extern crate error_chain;

mod errors;
mod api;
mod model;

pub use api::Simplenote;
pub use model::Note;
pub use errors::{Result, Error};
