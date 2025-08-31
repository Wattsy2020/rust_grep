use crate::pattern::union_pattern::union;
use crate::pattern::Match;
use std::fmt::Debug;

pub trait Pattern: Debug {
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
        (0..chars.len()).any(|i| self.matches_exact(&chars[i..]).is_match())
    }
}

/// A pattern that can be followed by or preceded by another pattern
/// Examples of Patterns that aren't chainable are the start and end line anchor patterns
pub trait ChainablePattern: Pattern {
    /// Create a new pattern that matches when this pattern and the next pattern match consecutively
    fn followed_by(self, pattern: Box<dyn ChainablePattern>) -> Box<dyn ChainablePattern>
    where
        Self: Sized + 'static,
    {
        Box::new(union(Box::new(self), pattern))
    }
}

impl Pattern for Box<dyn Pattern> {
    fn matches_exact(&self, chars: &[char]) -> Match {
        (**self).matches_exact(chars)
    }

    fn matches_exact_str(&self, string: &str) -> Match {
        (**self).matches_exact_str(string)
    }

    fn matches(&self, string: &str) -> bool {
        (**self).matches(string)
    }
}

impl Pattern for Box<dyn ChainablePattern> {
    fn matches_exact(&self, chars: &[char]) -> Match {
        (**self).matches_exact(chars)
    }

    fn matches_exact_str(&self, string: &str) -> Match {
        (**self).matches_exact_str(string)
    }

    fn matches(&self, string: &str) -> bool {
        (**self).matches(string)
    }
}

impl ChainablePattern for Box<dyn ChainablePattern> {
    fn followed_by(self, pattern: Box<dyn ChainablePattern>) -> Box<dyn ChainablePattern>
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
