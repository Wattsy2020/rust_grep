use crate::character_class::CharacterClass;

#[derive(Debug)]
struct WhitespaceCharacterClass;

impl CharacterClass for WhitespaceCharacterClass {
    fn matches(&self, character: char) -> bool {
        character.is_whitespace()
    }
}

pub fn whitespace() -> impl CharacterClass {
    WhitespaceCharacterClass {}
}
