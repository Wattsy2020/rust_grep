mod always_match;
mod character_pattern;
mod match_struct;
mod pattern;
mod union_pattern;

pub use always_match::always_match;
pub use character_pattern::{character, literal};
pub use match_struct::Match;
pub use pattern::Pattern;
