use crate::pattern::{ChainablePattern, Match, Pattern};
use crate::pattern::union_pattern::union;

#[derive(Debug)]
struct AlwaysMatch {}

impl Pattern for AlwaysMatch {
    fn matches_exact(&self, _: &[char]) -> Match {
        Match::at(0, 0)
    }
}

impl ChainablePattern for AlwaysMatch {
    fn followed_by(self, pattern: Box<dyn ChainablePattern>) -> Box<dyn ChainablePattern> {
        union(self, pattern)
    }
}

pub fn always_match() -> impl ChainablePattern {
    AlwaysMatch {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_matches() {
        let pattern = always_match();
        assert!(pattern.matches_exact_str("hello").is_match());
        assert!(pattern.matches_exact_str("h").is_match());
        assert!(pattern.matches_exact_str("").is_match());
    }
}
