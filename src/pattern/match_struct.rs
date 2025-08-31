use std::ops::Deref;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MatchIndices {
    /// Start index of the match (inclusive)
    pub start: usize,
    /// End index of the match (exclusive)
    pub end: usize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Match {
    Match(MatchIndices),
    None,
}

impl Match {
    pub fn at(start: usize, end: usize) -> Match {
        (MatchIndices { start, end }).into()
    }

    pub fn is_match(&self) -> bool {
        match self {
            Match::Match(_) => true,
            Match::None => false,
        }
    }

    pub fn and_then<F: FnOnce(&MatchIndices) -> Match>(&self, f: F) -> Match {
        match self {
            Match::None => Match::None,
            Match::Match(indices) => f(indices),
        }
    }
}

impl Default for Match {
    fn default() -> Self {
        Match::None
    }
}

impl From<MatchIndices> for Match {
    fn from(value: MatchIndices) -> Self {
        Match::Match(value)
    }
}

impl<T: Deref<Target = MatchIndices>> From<T> for Match {
    fn from(value: T) -> Self {
        Match::Match(value.clone())
    }
}

/// Combine two matches, only used by the pattern module as it makes assumptions about how the matches are made
pub fn combine_match(first_match: &MatchIndices, second_match: &MatchIndices) -> Match {
    // because the second.matches_exact sees the string starting at first_end
    // the actual ending idx in the original string is first_end + second_end
    Match::at(first_match.start, first_match.end + second_match.end)
}
