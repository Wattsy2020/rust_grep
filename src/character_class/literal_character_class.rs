use crate::character_class::CharacterClass;

struct LiteralCharacterClass {
    literal: char,
}

pub fn literal(character: char) -> impl CharacterClass {
    LiteralCharacterClass { literal: character }
}

impl CharacterClass for LiteralCharacterClass {
    fn matches(&self, character: char) -> bool {
        character == self.literal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_matching() {
        let class = literal('a');
        assert!(class.matches('a'));
        assert!(!class.matches('b'));
    }
}
