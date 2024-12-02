#![allow(
    unused_mut,
    unused_variables,
    clippy::empty_docs,
    clippy::into_iter_on_ref,
    clippy::new_without_default,
    clippy::too_many_arguments
)]

#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod apis;
pub mod models;
