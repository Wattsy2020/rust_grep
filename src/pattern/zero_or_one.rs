use crate::pattern::match_struct::combine_match;
use crate::pattern::{ChainablePattern, Match, Pattern};

#[derive(Debug)]
struct ZeroOrOnePattern {
    inner_pattern: Box<dyn Pattern>,
}

impl Pattern for ZeroOrOnePattern {
    fn matches_exact(&self, chars: &[char]) -> Match {
        self.inner_pattern
            .matches_exact(chars)
            .match_or(Match::at(0, 0)) // match inner pattern, or match zero times
    }
}

impl ChainablePattern for ZeroOrOnePattern {
    fn followed_by(self, pattern: Box<dyn ChainablePattern>) -> Box<dyn ChainablePattern>
    where
        Self: Sized + 'static,
    {
        Box::new(ZeroOrOneFollowedByPattern {
            zero_or_one: self,
            next_pattern: pattern,
        })
    }
}

#[derive(Debug)]
struct ZeroOrOneFollowedByPattern {
    zero_or_one: ZeroOrOnePattern,
    next_pattern: Box<dyn Pattern>,
}

impl Pattern for ZeroOrOneFollowedByPattern {
    fn matches_exact(&self, chars: &[char]) -> Match {
        match self.zero_or_one.inner_pattern.matches_exact(chars) {
            Match::None => self.next_pattern.matches_exact(chars),
            Match::Match(first_match) => {
                match self.next_pattern.matches_exact(&chars[first_match.end..]) {
                    Match::None => self.next_pattern.matches_exact(chars),
                    Match::Match(second_match) => combine_match(&first_match, &second_match),
                }
            }
        }
    }
}

impl ChainablePattern for ZeroOrOneFollowedByPattern {}

pub fn zero_or_one(pattern: Box<dyn ChainablePattern>) -> impl ChainablePattern {
    ZeroOrOnePattern {
        inner_pattern: pattern,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::character_pattern::{literal, literal_str};

    #[test]
    fn test_isolated_pattern() {
        let pattern = zero_or_one(Box::new(literal('a')));
        assert_eq!(pattern.matches_exact_str("a"), Match::at(0, 1));
        assert_eq!(pattern.matches_exact_str(""), Match::at(0, 0));
        assert_eq!(pattern.matches_exact_str("aa"), Match::at(0, 1));
        assert_eq!(pattern.matches_exact_str("b"), Match::at(0, 0));
    }

    #[test]
    fn test_complex_isolated_pattern() {
        let pattern = zero_or_one(literal_str("ab"));
        assert_eq!(pattern.matches_exact_str("a"), Match::at(0, 0));
        assert_eq!(pattern.matches_exact_str(""), Match::at(0, 0));
        assert_eq!(pattern.matches_exact_str("aa"), Match::at(0, 0));
        assert_eq!(pattern.matches_exact_str("b"), Match::at(0, 0));
        assert_eq!(pattern.matches_exact_str("ab"), Match::at(0, 2));
        assert_eq!(pattern.matches_exact_str("abab"), Match::at(0, 2));
    }

    #[test]
    fn test_chaining_patterns() {
        let pattern = literal('a')
            .followed_by(zero_or_one(Box::new(literal('b'))).followed_by(literal_str("bc")));
        assert_eq!(pattern.matches_exact_str("ac"), Match::None);
        assert_eq!(pattern.matches_exact_str("abc"), Match::at(0, 3));
        assert_eq!(pattern.matches_exact_str("abbc"), Match::at(0, 4));
    }

    #[test]
    fn test_chaining_multiple() {
        let pattern = literal('a').followed_by(
            zero_or_one(Box::new(literal('b')))
                .followed_by(zero_or_one(Box::new(literal('b'))).followed_by(literal_str("bc"))),
        );
        assert_eq!(pattern.matches_exact_str("ac"), Match::None);
        assert_eq!(pattern.matches_exact_str("abc"), Match::at(0, 3));
        assert_eq!(pattern.matches_exact_str("abbc"), Match::at(0, 4));
        assert_eq!(pattern.matches_exact_str("abbbc"), Match::at(0, 5));
    }
}
