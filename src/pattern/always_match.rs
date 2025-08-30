use crate::pattern::{Match, Pattern};

struct AlwaysMatch {}

impl Pattern for AlwaysMatch {
    fn matches_exact(&self, _: &[char]) -> Match {
        Match::at(0, 0)
    }
}

pub fn always_match() -> impl Pattern {
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
