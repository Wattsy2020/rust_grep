pub trait CharacterClass {
    fn matches(&self, character: &char) -> bool;

    fn union<T: CharacterClass>(self, other: T) -> UnionCharacterClass<Self, T>
    where
        Self: Sized,
    {
        UnionCharacterClass {
            first: self,
            second: other,
        }
    }

    fn negate(self) -> NegativeCharacterClass<Self>
    where
        Self: Sized,
    {
        NegativeCharacterClass { class: self }
    }
}

pub struct UnionCharacterClass<T1: CharacterClass, T2: CharacterClass> {
    first: T1,
    second: T2,
}

impl<T1: CharacterClass, T2: CharacterClass> CharacterClass for UnionCharacterClass<T1, T2> {
    fn matches(&self, character: &char) -> bool {
        self.first.matches(character) || self.second.matches(character)
    }
}

pub struct NegativeCharacterClass<T: CharacterClass> {
    class: T,
}

impl<T: CharacterClass> CharacterClass for NegativeCharacterClass<T> {
    fn matches(&self, character: &char) -> bool {
        !self.class.matches(character)
    }
}
