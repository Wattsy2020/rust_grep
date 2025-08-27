use crate::pattern::{Match, Pattern};

struct UnionPattern {
    first: Box<dyn Pattern>,
    second: Box<dyn Pattern>,
}

pub fn union(first: Box<dyn Pattern>, second: Box<dyn Pattern>) -> impl Pattern {
    UnionPattern { first, second }
}

impl Pattern for UnionPattern {
    fn matches_exact(&self, string: &str) -> Match {
        match self.first.matches_exact(string) {
            Match::None => Match::None,
            Match::Match {
                start: first_start,
                end: first_end,
            } => match self.second.matches_exact(&string[first_end..]) {
                Match::None => Match::None,
                Match::Match {
                    start: _,
                    end: second_end,
                } => Match::Match {
                    start: first_start,
                    // because the second.matches_exact sees the string starting at first_end
                    // the actual idx in the original string is first_end + second_end
                    end: first_end + second_end,
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::character_pattern::literal;

    #[test]
    fn test_literal_combination() {
        let combined = union(Box::new(literal('a')), Box::new(literal('b')));
        assert_eq!(
            combined.matches_exact("abcd"),
            Match::Match { start: 0, end: 2 }
        );
        assert_eq!(combined.matches_exact("aabcd"), Match::None);
        assert_eq!(combined.matches_exact("a"), Match::None);
        assert_eq!(combined.matches_exact(""), Match::None);
    }

    #[test]
    fn test_literal_combination_of_three_characters() {
        let combined2 = union(Box::new(literal('a')), Box::new(literal('b')));
        let combined3 = union(Box::new(combined2), Box::new(literal('c')));
        assert_eq!(
            combined3.matches_exact("abcd"),
            Match::Match { start: 0, end: 3 }
        );
        assert_eq!(combined3.matches_exact("aabcd"), Match::None);
        assert_eq!(combined3.matches_exact("ab"), Match::None);
        assert_eq!(combined3.matches_exact("a"), Match::None);
        assert_eq!(combined3.matches_exact(""), Match::None);
    }
}
