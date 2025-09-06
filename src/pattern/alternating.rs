use crate::pattern::match_struct::combine_match;
use crate::pattern::{ChainablePattern, Match, Pattern};
use crate::pattern::union_pattern::union;

#[derive(Debug)]
struct AlternatingPattern {
    first_option: Box<dyn Pattern>,
    second_option: Box<dyn Pattern>,
}

pub fn alternating(
    first_option: Box<dyn ChainablePattern>,
    second_option: Box<dyn ChainablePattern>,
) -> impl ChainablePattern {
    AlternatingPattern {
        first_option,
        second_option,
    }
}

impl Pattern for AlternatingPattern {
    fn matches_exact(&self, chars: &[char]) -> Match {
        self.first_option
            .matches_exact(chars)
            .match_or(self.second_option.matches_exact(chars))
    }
}

impl ChainablePattern for AlternatingPattern {
    fn followed_by(self, pattern: Box<dyn ChainablePattern>) -> Box<dyn ChainablePattern>
    where
        Self: Sized + 'static,
    {
        Box::new(AlternatingPatternFollowedBy {
            alternating_pattern: self,
            next_pattern: pattern,
        })
    }
}

#[derive(Debug)]
struct AlternatingPatternFollowedBy {
    alternating_pattern: AlternatingPattern,
    next_pattern: Box<dyn Pattern>,
}

impl Pattern for AlternatingPatternFollowedBy {
    fn matches_exact(&self, chars: &[char]) -> Match {
        self.alternating_pattern
            .first_option
            .matches_exact(chars)
            .and_then(|first_match| {
                self.next_pattern
                    .matches_exact(&chars[first_match.end..])
                    .and_then(|second_match| combine_match(first_match, second_match))
            })
            .match_or(
                self.alternating_pattern
                    .second_option
                    .matches_exact(chars)
                    .and_then(|first_match| {
                        self.next_pattern
                            .matches_exact(&chars[first_match.end..])
                            .and_then(|second_match| combine_match(first_match, second_match))
                    }),
            )
    }
}

impl ChainablePattern for AlternatingPatternFollowedBy {
    fn followed_by(self, pattern: Box<dyn ChainablePattern>) -> Box<dyn ChainablePattern> {
        union(self, pattern)
    }
}
