use crate::pattern::{Match, Pattern};

pub struct UnionPattern {
    first: Box<dyn Pattern>,
    second: Box<dyn Pattern>,
}

impl UnionPattern {
    pub fn union(first: Box<dyn Pattern>, second: Box<dyn Pattern>) -> UnionPattern {
        UnionPattern {
            first,
            second,
        }
    }
}
    
impl Pattern for UnionPattern {
    fn matches_exact(&self, string: &str) -> Match {
        match self.first.matches_exact(string) {
            Match::None => Match::None,
            Match::Match { start: first_start, end } => match self.second.matches_exact(&string[end..]) {
                Match::None => Match::None,
                Match::Match { start: _, end: second_end } => Match::Match { start: first_start, end: second_end }
            }
        }
    }
}
