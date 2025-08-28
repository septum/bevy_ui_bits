//! A mingy and opinionated collection of UI components for Bevy

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::cargo)]
// TODO: Remove this
// #![allow(clippy::multiple_crate_versions)]

mod components;
mod layout;

pub use components::*;
pub use layout::*;
