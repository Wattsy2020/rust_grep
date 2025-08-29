use crate::character_class::CharacterClass;
use crate::pattern::{Match, Pattern};

struct CharacterPattern {
    character_class: Box<dyn CharacterClass>,
}

pub fn character(character_class: Box<dyn CharacterClass>) -> impl Pattern {
    CharacterPattern { character_class }
}

pub fn literal(char: char) -> impl Pattern {
    character(Box::new(crate::character_class::literal(char)))
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
