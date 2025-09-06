use crate::pattern::{ChainablePattern, Match, Pattern};
use crate::pattern::union_pattern::union;

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

impl ChainablePattern for WildcardPattern {
    fn followed_by(self, pattern: Box<dyn ChainablePattern>) -> Box<dyn ChainablePattern> {
        union(self, pattern)
    }
}
