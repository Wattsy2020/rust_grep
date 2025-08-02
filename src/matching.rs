use crate::character_class;
use crate::character_class::CharacterClass;

pub fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let pattern_chars: Vec<char> = pattern.chars().into_iter().collect();
    match pattern_chars.as_slice() {
        [_] => input_line.contains(pattern),
        ['\\', 'd'] => {
            let class = character_class::digits();
            input_line.chars().any(|c| class.matches(&c))
        }
        ['\\', 'w'] => {
            let class = character_class::alphanumeric();
            input_line.chars().any(|c| class.matches(&c))
        }
        ['[', chars @ .., ']'] => {
            let class = character_class::characters(chars);
            input_line.chars().any(|c| class.matches(&c))
        }
        _ => panic!("Unhandled pattern: {}", pattern),
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
}
