const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        input_line.contains(pattern)
    } else if pattern == "\\d" { 
        input_line.chars().any(|c| DIGITS.contains(&c))
    }
    else {
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
    fn match_digit(){
        assert!(match_pattern("1", "\\d"));
        assert!(match_pattern("hell1o", "\\d"));
        assert!(match_pattern("1235a", "\\d"));
        assert!(!match_pattern("hello", "\\d"));
    }
}