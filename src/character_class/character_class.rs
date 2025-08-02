pub trait CharacterClass {
    fn matches(&self, character: &char) -> bool;

    fn union<T: CharacterClass>(self, other: T) -> UnionCharacterClass<Self, T>
    where
        Self: Sized,
    {
        UnionCharacterClass::of(self, other)
    }
}

pub struct UnionCharacterClass<T1: CharacterClass, T2: CharacterClass> {
    first: T1,
    second: T2,
}

impl<T1: CharacterClass, T2: CharacterClass> UnionCharacterClass<T1, T2> {
    pub fn of(first: T1, second: T2) -> UnionCharacterClass<T1, T2> {
        UnionCharacterClass { first, second }
    }
}

impl<T1: CharacterClass, T2: CharacterClass> CharacterClass for UnionCharacterClass<T1, T2> {
    fn matches(&self, character: &char) -> bool {
        self.first.matches(character) || self.second.matches(character)
    }
}
