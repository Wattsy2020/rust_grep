use crate::pattern::{Match, Pattern};

struct UnionPattern {
    first: Box<dyn Pattern>,
    second: Box<dyn Pattern>,
}

pub fn union(first: Box<dyn Pattern>, second: Box<dyn Pattern>) -> impl Pattern {
    UnionPattern { first, second }
}

impl Pattern for UnionPattern {
    fn matches_exact(&self, chars: &[char]) -> Match {
        match self.first.matches_exact(chars) {
            Match::None => Match::None,
            Match::Match {
                start: first_start,
                end: first_end,
            } => match self.second.matches_exact(&chars[first_end..]) {
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
        let combined = literal('a').followed_by(Box::new(literal('b')));
        assert_eq!(
            combined.matches_exact_str("abcd"),
            Match::Match { start: 0, end: 2 }
        );
        assert_eq!(combined.matches_exact_str("aabcd"), Match::None);
        assert_eq!(combined.matches_exact_str("a"), Match::None);
        assert_eq!(combined.matches_exact_str(""), Match::None);
    }

    #[test]
    fn test_literal_combination_of_three_characters() {
        let combined3 = literal('a')
            .followed_by(Box::new(literal('b')))
            .followed_by(Box::new(literal('c')));
        assert_eq!(
            combined3.matches_exact_str("abcd"),
            Match::Match { start: 0, end: 3 }
        );
        assert_eq!(combined3.matches_exact_str("aabcd"), Match::None);
        assert_eq!(combined3.matches_exact_str("ab"), Match::None);
        assert_eq!(combined3.matches_exact_str("a"), Match::None);
        assert_eq!(combined3.matches_exact_str(""), Match::None);
    }

    #[test]
    fn test_union_handles_unicode() {
        let combined = literal('#')
            .followed_by(Box::new(literal('-')))
            .followed_by(Box::new(literal('×')))
            .followed_by(Box::new(literal('_')));
        assert_eq!(
            combined.matches_exact_str("#-×_=%-"),
            Match::Match { start: 0, end: 4 }
        )
    }
}
