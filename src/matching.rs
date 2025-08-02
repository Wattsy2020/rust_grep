use crate::character_class;
use crate::character_class::CharacterClass;

pub fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        input_line.contains(pattern)
    } else if pattern == "\\d" {
        let class = character_class::digits();
        input_line.chars().any(|c| class.matches(&c))
    } else if pattern == "\\w" {
        let class = character_class::alphanumeric();
        input_line.chars().any(|c| class.matches(&c))
    } else {
        panic!("Unhandled pattern: {}", pattern)
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
}
