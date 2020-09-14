#[macro_use]
extern crate derive_builder;

pub mod bookmark;
pub mod folder;
pub mod netscape;
mod node_ref_ext;

pub use netscape::Netscape;
