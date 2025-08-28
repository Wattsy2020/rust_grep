mod always_match;
mod character_pattern;
mod pattern;
mod union_pattern;

pub use character_pattern::{character, literal};
pub use pattern::{Match, Pattern};
pub use always_match::always_match;