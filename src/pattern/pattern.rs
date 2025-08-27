use crate::pattern::union_pattern::UnionPattern;

pub enum Match {
    Match { 
        /// Start index of the match (inclusive)
        start: usize, 
        /// End index of the match (exclusive)
        end: usize 
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
    /// Whether the Pattern matches starting from the first character of the string
    fn matches_exact(&self, string: &str) -> Match;

    /// Whether the Pattern matches starting from any character in the string
    fn matches(&self, string: &str) -> bool {
        (0..string.len()).any(|i| self.matches_exact(&string[i..]).is_match())
    }

    /// Create a new pattern that matches when this pattern and the next pattern match consecutively
    fn followed_by(self, pattern: Box<dyn Pattern>) -> Box<dyn Pattern> where Self: Sized + 'static {
        Box::new(UnionPattern::union(Box::new(self), pattern))
    }
}
