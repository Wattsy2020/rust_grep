use crate::pattern::{Match, Pattern};

#[derive(Debug)]
struct StartLineAnchor {
    inner_pattern: Box<dyn Pattern>,
}

impl Pattern for StartLineAnchor {
    fn matches_exact(&self, chars: &[char]) -> Match {
        self.inner_pattern.matches_exact(chars)
    }

    // ensure the pattern only matches from the start of the string
    // by overriding matches to only check from the start of the string
    fn matches(&self, string: &str) -> bool {
        self.matches_exact_str(string).is_match()
    }
}

pub fn start_line_anchor(pattern: Box<dyn Pattern>) -> impl Pattern {
    StartLineAnchor {
        inner_pattern: pattern,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::literal;

    #[test]
    fn test_start_line_anchor() {
        let pattern = start_line_anchor(Box::new(literal('a')));
        assert!(pattern.matches("abcd"));
        assert!(!pattern.matches(" abcd"));
        assert!(!pattern.matches("baaaaa"));
    }
}
