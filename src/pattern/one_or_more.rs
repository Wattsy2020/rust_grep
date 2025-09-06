use crate::pattern::match_struct::combine_match;
use crate::pattern::{ChainablePattern, Match, Pattern};
use crate::pattern::union_pattern::union;

#[derive(Debug)]
struct OneOrMorePattern {
    inner_pattern: Box<dyn Pattern>,
}

impl Pattern for OneOrMorePattern {
    fn matches_exact(&self, chars: &[char]) -> Match {
        self.inner_pattern
            .matches_exact(chars)
            .and_then(
                |first_match| match self.matches_exact(&chars[first_match.end..]) {
                    Match::None => first_match.into(),
                    Match::Match(second_match) => combine_match(first_match, &second_match),
                },
            )
    }
}

impl ChainablePattern for OneOrMorePattern {
    // Override followed by to ensure that OneOrMorePattern stops early enough to allow the next pattern to match
    fn followed_by(self, pattern: Box<dyn ChainablePattern>) -> Box<dyn ChainablePattern>
    where
        Self: Sized + 'static,
    {
        Box::new(OneOrMoreFollowedByPattern {
            one_or_more: self,
            second: pattern,
        })
    }
}

#[derive(Debug)]
struct OneOrMoreFollowedByPattern {
    one_or_more: OneOrMorePattern,
    second: Box<dyn Pattern>,
}

impl Pattern for OneOrMoreFollowedByPattern {
    // matches the one_or_more pattern as many times as possible, while still allowing the second pattern to match
    fn matches_exact(&self, chars: &[char]) -> Match {
        self.one_or_more
            .inner_pattern
            .matches_exact(chars)
            .and_then(|first_match| {
                let remaining = &chars[first_match.end..];
                match self.matches_exact(remaining) {
                    Match::None => self
                        .second
                        .matches_exact(remaining)
                        .and_then(|second_match| combine_match(first_match, second_match)),
                    Match::Match(second_match) => combine_match(first_match, &second_match),
                }
            })
    }
}

impl ChainablePattern for OneOrMoreFollowedByPattern {
    fn followed_by(self, pattern: Box<dyn ChainablePattern>) -> Box<dyn ChainablePattern> {
        union(self, pattern)
    }
}

pub fn one_or_more(pattern: Box<dyn ChainablePattern>) -> impl ChainablePattern {
    OneOrMorePattern {
        inner_pattern: pattern,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::character_pattern::{literal, literal_str};

    #[test]
    fn test_simple_repeat() {
        let pattern = one_or_more(Box::new(literal('a')));
        assert_eq!(pattern.matches_exact_str(""), Match::None);
        assert_eq!(pattern.matches_exact_str("a"), Match::at(0, 1));
        assert_eq!(pattern.matches_exact_str("aaa"), Match::at(0, 3));
        assert_eq!(pattern.matches_exact_str("aaab"), Match::at(0, 3));
    }

    #[test]
    fn test_complex_repeat() {
        let pattern = one_or_more(literal_str("ab"));
        assert_eq!(pattern.matches_exact_str(""), Match::None);
        assert_eq!(pattern.matches_exact_str("ab"), Match::at(0, 2));
        assert_eq!(pattern.matches_exact_str("abab"), Match::at(0, 4));
        assert_eq!(pattern.matches_exact_str("ababa"), Match::at(0, 4));
        assert_eq!(pattern.matches_exact_str("abaab"), Match::at(0, 2));
    }

    #[test]
    fn test_repeat_followed_by_pattern() {
        let pattern = one_or_more(Box::new(literal('a'))).followed_by(literal_str("ab"));
        assert_eq!(pattern.matches_exact_str(""), Match::None);
        assert_eq!(pattern.matches_exact_str("a"), Match::None);
        assert_eq!(pattern.matches_exact_str("aa"), Match::None);
        assert_eq!(pattern.matches_exact_str("ab"), Match::None);
        assert_eq!(pattern.matches_exact_str("aab"), Match::at(0, 3));
        assert_eq!(pattern.matches_exact_str("aaab"), Match::at(0, 4));
    }

    #[test]
    fn test_complex_repeat_followed_by_pattern() {
        let pattern = one_or_more(literal_str("ab")).followed_by(literal_str("ab"));
        assert_eq!(pattern.matches_exact_str(""), Match::None);
        assert_eq!(pattern.matches_exact_str("ab"), Match::None);
        assert_eq!(pattern.matches_exact_str("aba"), Match::None);
        assert_eq!(pattern.matches_exact_str("abab"), Match::at(0, 4));
        assert_eq!(pattern.matches_exact_str("ababab"), Match::at(0, 6));
    }
}
