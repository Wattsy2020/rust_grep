use crate::character_class::CharacterClass;

#[derive(Debug)]
pub struct CharacterGroupClass<const N: usize> {
    valid_chars: [char; N],
}

#[derive(Debug)]
pub struct CharacterGroupClassBoxed {
    valid_chars: Box<[char]>,
}

impl<const N: usize> CharacterClass for CharacterGroupClass<N> {
    fn matches(&self, character: char) -> bool {
        self.valid_chars.contains(&character)
    }
}

impl<const N: usize> CharacterGroupClass<N> {
    fn union<const M: usize>(self, other: CharacterGroupClass<M>) -> CharacterGroupClassBoxed {
        CharacterGroupClassBoxed {
            valid_chars: self
                .valid_chars
                .into_iter()
                .chain(other.valid_chars.into_iter())
                .collect(),
        }
    }
}

impl CharacterClass for CharacterGroupClassBoxed {
    fn matches(&self, character: char) -> bool {
        self.valid_chars.contains(&character)
    }
}

impl<const N: usize> From<CharacterGroupClass<N>> for CharacterGroupClassBoxed {
    fn from(value: CharacterGroupClass<N>) -> Self {
        CharacterGroupClassBoxed {
            valid_chars: Box::new(value.valid_chars),
        }
    }
}

impl CharacterGroupClassBoxed {
    pub fn of(character: char) -> CharacterGroupClassBoxed {
        CharacterGroupClassBoxed {
            valid_chars: Box::new([character]),
        }
    }

    pub fn union(self, other: CharacterGroupClassBoxed) -> CharacterGroupClassBoxed {
        CharacterGroupClassBoxed {
            valid_chars: [self.valid_chars, other.valid_chars]
                .concat()
                .into_iter()
                .collect(),
        }
    }
}

pub const fn digits() -> CharacterGroupClass<10> {
    DIGITS
}

pub const fn lowercase() -> CharacterGroupClass<26> {
    LOWERCASE
}

pub const fn uppercase() -> CharacterGroupClass<26> {
    UPPERCASE
}

pub fn alphabetical() -> CharacterGroupClassBoxed {
    lowercase().union(uppercase())
}

pub fn alphanumeric() -> CharacterGroupClassBoxed {
    alphabetical()
        .union(CharacterGroupClassBoxed::of('_'))
        .union(DIGITS.into())
}

const DIGITS: CharacterGroupClass<10> = CharacterGroupClass {
    valid_chars: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
};

const LOWERCASE: CharacterGroupClass<26> = CharacterGroupClass {
    valid_chars: [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ],
};

const UPPERCASE: CharacterGroupClass<26> = CharacterGroupClass {
    valid_chars: [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ],
};
