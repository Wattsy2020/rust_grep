use crate::pattern::match_struct::combine_match;
use crate::pattern::ChainablePattern;
use crate::pattern::{Match, Pattern};

#[derive(Debug)]
struct UnionPattern {
    first: Box<dyn Pattern>,
    second: Box<dyn Pattern>,
}

pub fn union<T: ChainablePattern + Sized + 'static>(
    first: T,
    second: Box<dyn ChainablePattern>,
) -> Box<dyn ChainablePattern> {
    Box::new(UnionPattern { first: Box::new(first), second })
}

impl Pattern for UnionPattern {
    fn matches_exact(&self, chars: &[char]) -> Match {
        self.first.matches_exact(chars).and_then(|first_match| {
            self.second
                .matches_exact(&chars[first_match.end..])
                .and_then(|second_match| combine_match(first_match, second_match))
        })
    }
}

impl ChainablePattern for UnionPattern {
    fn followed_by(self, pattern: Box<dyn ChainablePattern>) -> Box<dyn ChainablePattern> {
        union(self, pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::character_pattern::literal;

    #[test]
    fn test_literal_combination() {
        let combined = literal('a').followed_by(Box::new(literal('b')));
        assert_eq!(combined.matches_exact_str("abcd"), Match::at(0, 2));
        assert_eq!(combined.matches_exact_str("aabcd"), Match::None);
        assert_eq!(combined.matches_exact_str("a"), Match::None);
        assert_eq!(combined.matches_exact_str(""), Match::None);
    }

    #[test]
    fn test_literal_combination_of_three_characters() {
        let combined3 = literal('a')
            .followed_by(Box::new(literal('b')))
            .followed_by(Box::new(literal('c')));
        assert_eq!(combined3.matches_exact_str("abcd"), Match::at(0, 3));
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
        assert_eq!(combined.matches_exact_str("#-×_=%-"), Match::at(0, 4))
    }
}
