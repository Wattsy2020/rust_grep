use crate::pattern::{Match, Pattern};

struct AlwaysMatch {}

impl Pattern for AlwaysMatch {
    fn matches_exact(&self, string: &str) -> Match {
        Match::Match { start: 0, end: 0 }
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
        assert!(pattern.matches_exact("hello").is_match());
        assert!(pattern.matches_exact("h").is_match());
        assert!(pattern.matches_exact("").is_match());
    }
}
