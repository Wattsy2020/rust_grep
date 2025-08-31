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
