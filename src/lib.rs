#![warn(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
        unused_qualifications)]

//! Wrapper for the Groupme Bots HTTP API.
//!
//! This library interacts with the bots sections of the Groupme v3 API.
//! Information about the API is [here](https://dev.groupme.com/docs/v3#bots),
//! and a bots specific example is [here](https://dev.groupme.com/tutorials/bots).
//!
//! ## Example
//!
//! ```rust
//! use groupme::{Groupme, Bot}
//!
//! let groupme = Groupme::new(None);
//! let bot = groupme.bot("Secret bot_id");
//!
//! bot.post("Hello, world!");
//! ```
extern crate reqwest;
extern crate serde;
extern crate serde_json;

pub use self::groupme::Groupme;
pub use self::bot::Bot;
pub use self::error::GroupmeError;
pub use self::bot_builder::BotBuilder;

mod groupme;
mod bot;
mod error;
mod client;
mod bot_builder;
