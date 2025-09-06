use crate::pattern::{ChainablePattern, Match, Pattern};

#[derive(Debug)]
struct WildcardPattern {}

pub fn wildcard() -> impl ChainablePattern {
    WildcardPattern {}
}

impl Pattern for WildcardPattern {
    fn matches_exact(&self, chars: &[char]) -> Match {
        if chars.is_empty() {
            Match::None
        } else {
            Match::at(0, 1)
        }
    }
}

impl ChainablePattern for WildcardPattern {}
