use crate::character_class::CharacterClass;
use crate::pattern::{always_match, ChainablePattern, Match, Pattern};

#[derive(Debug)]
struct CharacterPattern {
    character_class: Box<dyn CharacterClass>,
}

pub fn character(character_class: Box<dyn CharacterClass>) -> impl ChainablePattern {
    CharacterPattern { character_class }
}

pub fn literal(char: char) -> impl ChainablePattern {
    character(Box::new(crate::character_class::literal(char)))
}

#[allow(dead_code)] // this is useful for tests
pub fn literal_str(string: &str) -> Box<dyn ChainablePattern> {
    let chars: Box<[char]> = string.chars().collect();
    chars
        .iter()
        .fold(Box::new(always_match()), |pattern, char| {
            pattern.followed_by(Box::new(literal(*char)))
        })
}

impl Pattern for CharacterPattern {
    fn matches_exact(&self, chars: &[char]) -> Match {
        match chars.first() {
            None => Match::None,
            Some(char) => match self.character_class.matches(*char) {
                false => Match::None,
                true => Match::at(0, 1),
            },
        }
    }
}

impl ChainablePattern for CharacterPattern {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match() {
        let pattern = literal('a');
        assert_eq!(pattern.matches_exact_str("abcd"), Match::at(0, 1));
        assert_eq!(pattern.matches_exact_str("babcd"), Match::None);
        assert_eq!(pattern.matches_exact_str("b"), Match::None);
        assert_eq!(pattern.matches_exact_str(""), Match::None);
    }
}
