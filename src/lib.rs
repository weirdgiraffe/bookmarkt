//! A [Netscape Bookmark File format] parser.
//!
//! The name is a subtile *mélange* between *bookmark* and *book mart*. It also works as a mashup with the english *book* and the german *markt*.
//!
//! [Netscape Bookmark File format]: https://docs.microsoft.com/en-us/previous-versions/windows/internet-explorer/ie-developer/platform-apis/aa753582(v=vs.85)
//!
//! # Quick start
//!
//! In order to parse a *bookmark file*, you should use the [Netscape] struct.
//!
//! ```rust
//! use bookmarkt::netscape::Netscape;
//! use serde_json;
//! use std::path::Path;
//!
//! let parsed = Netscape::from_file(Path::new("./res/netscape.html")).unwrap();
//! println!("{:?}", serde_json::to_string(&parsed).unwrap());
//! ```
//!
//! # Acknowledgment
//!
//! `bookmarkt` uses the following dependencies :
//! * [kuchiki](https://github.com/kuchiki-rs/kuchiki)
//! * [html5ever](https://github.com/servo/html5ever)
//!
//! I really appreciate these works and I hope you appreciate them too ;).
//!
//! # Contributing
//!
//! Open a ticket on the [`bookmarkt` tracker](https://todo.sr.ht/~vlnk/bookmarkt).

#[macro_use]
extern crate derive_builder;

pub mod bookmark;
pub mod folder;
pub mod item;
pub mod netscape;
mod node_ref_ext;

pub use netscape::Netscape;
