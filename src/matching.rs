use crate::character_class::{alphanumeric, characters, digits, CharacterClass};
use crate::matching::ParsePatternError::{
    InvalidEndLineAnchor, InvalidStartLineAnchor, UnmatchedBracket,
};
use crate::parse::split_at;
use crate::pattern::{
    always_match, character, end_line_anchor, literal, start_line_anchor, ChainablePattern, Pattern,
};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
enum ParsePatternError {
    #[error(
        "Start line anchor must be at the start of the pattern, instead found it at col number {0}"
    )]
    InvalidStartLineAnchor(usize),
    #[error(
        "End line anchor must be at the end of the pattern, instead found it at col number {0}"
    )]
    InvalidEndLineAnchor(usize),
    #[error("Unmatched opening bracket at col number {0}")]
    UnmatchedBracket(usize),
}

fn from_character_class(class: impl CharacterClass + 'static) -> Box<dyn ChainablePattern> {
    Box::new(character(Box::new(class)))
}

fn construct_pattern(
    pattern_chars: &[char],
    char_idx: usize,
) -> Result<Box<dyn ChainablePattern>, ParsePatternError> {
    match pattern_chars {
        ['^', ..] => Err(InvalidStartLineAnchor(char_idx)),
        ['$', ..] => Err(InvalidEndLineAnchor(char_idx)),
        ['\\', char, remaining @ ..] => Ok((match char {
            'd' => from_character_class(digits()),
            'w' => from_character_class(alphanumeric()),
            // if escape isn't followed by an escaped character, assume it is a literal escape
            _ => literal('\\').followed_by(Box::new(literal(*char))),
        })
        .followed_by(construct_pattern(remaining, char_idx + 2)?)),
        // we match starting from the '[' part, and then manually check for the first closing ']'
        ['[', remaining @ ..] => match split_at(remaining, ']') {
            None => Err(UnmatchedBracket(char_idx)),
            Some((chars, remaining)) => Ok((match chars {
                ['^', chars @ ..] => from_character_class(characters(chars).negate()),
                _ => from_character_class(characters(chars)),
            })
            .followed_by(construct_pattern(remaining, char_idx + chars.len() + 2)?)),
        },
        [char, remaining @ ..] => {
            Ok(literal(*char).followed_by(construct_pattern(remaining, char_idx + 1)?))
        }
        [] => Ok(Box::new(always_match())), // an empty pattern matches anything
    }
}

// Handle start and end line anchors in the pattern
// This is a separate function because start and end line anchors don't implement ChainablePattern
// and construct_pattern needs to return ChainablePattern for its recursion to work
fn construct_pattern_with_anchors(
    pattern_chars: &[char],
    char_idx: usize,
) -> Result<Box<dyn Pattern>, ParsePatternError> {
    match pattern_chars {
        ['^', remaining @ ..] => Ok(Box::new(start_line_anchor(construct_pattern_with_anchors(
            remaining,
            char_idx + 1,
        )?))),
        [remaining @ .., '$'] => Ok(Box::new(end_line_anchor(construct_pattern(
            remaining, char_idx,
        )?))),
        ['$', ..] => Err(InvalidEndLineAnchor(char_idx)),
        _ => construct_pattern(pattern_chars, char_idx).map(|p| p as Box<dyn Pattern>),
    }
}

fn construct_pattern_from_str(pattern: &str) -> Result<Box<dyn Pattern>, ParsePatternError> {
    let pattern_chars: Box<[char]> = pattern.chars().collect();
    construct_pattern_with_anchors(&pattern_chars, 0)
}

pub fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match construct_pattern_from_str(pattern) {
        Err(error) => panic!("Invalid pattern: {error:?}"),
        Ok(pattern) => {
            let pattern_str = format!("{pattern:?}");
            println!("Parsed pattern: {pattern_str}");
            pattern.matches(input_line)
        }
    }
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
        assert!(!match_pattern("%=#÷+×", "\\w"));
    }

    #[test]
    fn handle_start_line_anchor() {
        assert!(match_pattern("abcd", "^a"));
        assert!(!match_pattern(" abcd", "^a"));
        assert!(!match_pattern("baaaa", "^a"));
    }

    #[test]
    fn handle_end_line_anchor() {
        assert!(match_pattern("dog", "dog$"));
        assert!(match_pattern("one dog", "dog$"));
        assert!(!match_pattern("dogs", "dog$"));
        assert!(!match_pattern("two dogs", "dog$"));
    }

    #[test]
    fn handle_start_and_end_anchors() {
        assert!(match_pattern("dog", "^dog$"));
        assert!(!match_pattern("dog dog", "^dog$"));
    }

    #[test]
    fn report_unmatching_brackets() {
        let error = construct_pattern_from_str("[abcde").err();
        assert_eq!(error, Some(UnmatchedBracket(0)));
        let error = construct_pattern_from_str("ab[cde").err();
        assert_eq!(error, Some(UnmatchedBracket(2)));
        let error = construct_pattern_from_str("[ab][cde").err();
        assert_eq!(error, Some(UnmatchedBracket(4)));
    }

    #[test]
    fn report_invalid_start_line_anchor() {
        let error = construct_pattern_from_str("a^bcde").err();
        assert_eq!(error, Some(InvalidStartLineAnchor(1)));
        let error = construct_pattern_from_str(" ^ab[cde").err();
        assert_eq!(error, Some(InvalidStartLineAnchor(1)));
        let error = construct_pattern_from_str("[ab]^cde").err();
        assert_eq!(error, Some(InvalidStartLineAnchor(4)));
    }

    #[test]
    fn report_invalid_end_line_anchor() {
        let error = construct_pattern_from_str("abcd$e").err();
        assert_eq!(error, Some(InvalidEndLineAnchor(4)));
        let error = construct_pattern_from_str(" a$b[cde").err();
        assert_eq!(error, Some(InvalidEndLineAnchor(2)));
        let error = construct_pattern_from_str("[ab]c$de").err();
        assert_eq!(error, Some(InvalidEndLineAnchor(5)));
    }
}
