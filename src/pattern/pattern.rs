use crate::pattern::union_pattern::union;
use std::fmt::Debug;
use std::ops::Deref;

#[derive(Debug, Eq, PartialEq)]
pub enum Match {
    Match {
        /// Start index of the match (inclusive)
        start: usize,
        /// End index of the match (exclusive)
        end: usize,
    },
    None,
}

impl Match {
    pub fn is_match(&self) -> bool {
        match self {
            Match::Match { .. } => true,
            Match::None => false,
        }
    }
}

pub trait Pattern {
    /// Whether the Pattern matches starting from the first character
    fn matches_exact(&self, chars: &[char]) -> Match;

    /// Whether the Pattern matches starting from the first character of the string
    fn matches_exact_str(&self, string: &str) -> Match {
        let chars: Box<[char]> = string.chars().collect();
        self.matches_exact(&chars)
    }

    /// Whether the Pattern matches starting from any character in the string
    fn matches(&self, string: &str) -> bool {
        let chars: Box<[char]> = string.chars().collect();
        (0..string.len()).any(|i| self.matches_exact(&chars[i..]).is_match())
    }

    /// Create a new pattern that matches when this pattern and the next pattern match consecutively
    fn followed_by(self, pattern: Box<dyn Pattern>) -> Box<dyn Pattern>
    where
        Self: Sized + 'static,
    {
        Box::new(union(Box::new(self), pattern))
    }
}

impl Pattern for Box<dyn Pattern> {
    fn matches_exact(&self, chars: &[char]) -> Match {
        self.deref().matches_exact(chars)
    }

    fn followed_by(self, pattern: Box<dyn Pattern>) -> Box<dyn Pattern>
    where
        Self: 'static,
    {
        Box::new(union(self, pattern))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::character_pattern::literal;

    #[test]
    fn test_matches() {
        let pattern = literal('a');
        assert!(!pattern.matches_exact_str("bacd").is_match());
        assert!(pattern.matches("bacd"));
        assert!(pattern.matches("a"));
        assert!(pattern.matches("super long sentence with a"));
    }

    #[test]
    fn test_matches_handles_unicode() {
        assert!(literal('×').matches("#-×_=%-"));
        assert!(literal('-').matches("#-×_=%-"));
        assert!(!literal('a').matches("#-×_=%-"));
    }
}
