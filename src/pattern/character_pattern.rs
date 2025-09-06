use crate::character_class;
use crate::character_class::CharacterClass;
use crate::pattern::{always_match, ChainablePattern, Match, Pattern};

#[derive(Debug)]
struct CharacterPattern {
    character_class: Box<dyn CharacterClass>,
}

pub fn literal(char: char) -> impl ChainablePattern {
    character(Box::new(character_class::literal(char)))
}

pub fn digits() -> impl ChainablePattern {
    character(Box::new(character_class::digits()))
}

pub fn alphanumeric() -> impl ChainablePattern {
    character(Box::new(character_class::alphanumeric()))
}

fn character(character_class: Box<dyn CharacterClass>) -> impl ChainablePattern {
    CharacterPattern { character_class }
}

/// Parse a character group pattern that supports matching any single character listed
/// Chars must not be empty
pub fn union(chars: &[char]) -> impl ChainablePattern {
    character(match chars {
        ['^', chars @ ..] => Box::new(parse_character_group_pattern(chars).expect("chars must not be empty").negate()),
        _ => Box::new(parse_character_group_pattern(chars).expect("chars must not be empty")),
    })
}

fn parse_character_group_pattern(chars: &[char]) -> Option<Box<dyn CharacterClass>> {
    let (pattern, remaining) = match chars {
        ['\\', char, remaining @ ..] => 
            (match char {
                'd' => Box::new(character_class::digits()) as Box<dyn CharacterClass>,
                'w' => Box::new(character_class::alphanumeric()) as Box<dyn CharacterClass>,
                // if escape isn't followed by an escaped character, assume it is a literal escape
                _ => {
                    Box::new(character_class::literal('\\').union(character_class::literal(*char)))
                        as Box<dyn CharacterClass>
                }
            }, remaining),
        [char, remaining @ ..] => (Box::new(character_class::literal(*char)) as Box<dyn CharacterClass>, remaining),
        [] => None?,
    };
    match parse_character_group_pattern(remaining) {
        None => Some(pattern),
        Some(other_character_class) => Some(Box::new(pattern.union(other_character_class)))
    }
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
