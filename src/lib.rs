//! A mingy and opinionated collection of UI components for Bevy

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::cargo_common_metadata)]
#![deny(clippy::negative_feature_names)]
#![deny(clippy::redundant_feature_names)]
#![deny(clippy::wildcard_dependencies)]

mod button;
mod layout;
mod text;

pub use button::*;
pub use layout::*;
pub use text::*;
