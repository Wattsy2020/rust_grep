use crate::character_class::{alphanumeric, characters, digits, CharacterClass};
use crate::parse::split_at;
use crate::pattern::{always_match, character, literal, Pattern};

fn from_character_class(class: impl CharacterClass + 'static) -> Box<dyn Pattern> {
    Box::new(character(Box::new(class)))
}

fn construct_pattern(pattern_chars: &[char]) -> Box<dyn Pattern> {
    match pattern_chars {
        ['\\', char, remaining @ ..] => (match char {
            'd' => from_character_class(digits()),
            'w' => from_character_class(alphanumeric()),
            // if escape isn't followed by an escaped character, assume it is a literal escape
            _ => literal('\\').followed_by(Box::new(literal(*char))),
        })
        .followed_by(construct_pattern(remaining)),
        // we match starting from the '[' part, and then manually check for the first closing ']'
        ['[', remaining @ ..] => match split_at(remaining, ']') {
            None => panic!("Unmatched square bracket"),
            Some((chars, remaining)) => (match chars {
                ['^', chars @ ..] => from_character_class(characters(chars).negate()),
                _ => from_character_class(characters(chars)),
            })
            .followed_by(construct_pattern(remaining)),
        },
        [char, remaining @ ..] => literal(*char).followed_by(construct_pattern(remaining)),
        [] => Box::new(always_match()), // an empty pattern matches anything
    }
}

pub fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let pattern_chars: Vec<char> = pattern.chars().into_iter().collect();
    let pattern = construct_pattern(&pattern_chars);
    pattern.matches(input_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_single_char() {
        assert!(match_pattern("a", "a"));
        assert!(match_pattern("hello", "e"));
        assert!(!match_pattern("hello", "a"));
    }

    #[test]
    fn match_digit() {
        assert!(match_pattern("1", "\\d"));
        assert!(match_pattern("hell1o", "\\d"));
        assert!(match_pattern("1235a", "\\d"));
        assert!(!match_pattern("hello", "\\d"));
    }

    #[test]
    fn match_alpha() {
        assert!(match_pattern("a", "\\w"));
        assert!(match_pattern("hell1o", "\\w"));
        assert!(match_pattern("1235a", "\\w"));
        assert!(match_pattern("hello", "\\w"));
        assert!(match_pattern("1", "\\w"));
        assert!(match_pattern("_", "\\w"));
        assert!(!match_pattern("[]/.,", "\\w"));
    }

    #[test]
    fn match_character_groups() {
        assert!(match_pattern("a", "[abc]"));
        assert!(match_pattern("123cd5", "[abc]"));
        assert!(match_pattern("12b2", "[abc]"));
        assert!(!match_pattern("hello", "[abc]"));
    }

    #[test]
    fn match_negative_character_groups() {
        assert!(!match_pattern("a", "[^abc]"));
        assert!(!match_pattern("cab", "[^abc]"));
        assert!(match_pattern("scab", "[^abc]"));
        assert!(match_pattern("123cd5", "[^abc]"));
        assert!(match_pattern("12b2", "[^abc]"));
        assert!(match_pattern("hello", "[^abc]"));
    }

    #[test]
    fn match_consecutive_characters() {
        assert!(match_pattern("hello world", "world"));
        assert!(!match_pattern("hello worl d", "world"));
        assert!(match_pattern("1 apple", "\\d apple"));
        assert!(!match_pattern("1a apple", "\\d apple"));
        assert!(match_pattern("100 apples", "\\d\\d\\d apple"));
        assert!(!match_pattern("10 apples", "\\d\\d\\d apple"));
        assert!(!match_pattern("two apples", "\\d\\d\\d apple"));
        assert!(match_pattern("2 dogs", "\\d \\w\\w\\ws"));
        assert!(match_pattern("3 cats", "\\d \\w\\w\\ws"));
        assert!(!match_pattern("1 dog", "\\d \\w\\w\\ws"));
    }

    #[test]
    fn match_consecutive_character_groups() {
        assert!(match_pattern("ad", "[abc][def]"));
        assert!(match_pattern("bf", "[abc][def]"));
        assert!(!match_pattern("da", "[abc][def]"));
        assert!(match_pattern("a 1z d", "[abc] \\d\\w [def]"));
        assert!(!match_pattern("a 1z g", "[abc] \\d\\w [def]"));
    }

    #[test]
    fn handle_unicode() {
        assert!(match_pattern("#-×_=%-", "\\w"));
    }
}
