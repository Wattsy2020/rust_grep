/// Split chars at the separator, returning a tuple of (chars before separator, chars after separator)
/// Or None if the separator is not found
pub fn split_at(chars: &[char], separator: char) -> Option<(&[char], &[char])> {
    let separator_idx = chars.iter().position(|char| *char == separator)?;
    let (before, after) = chars.split_at(separator_idx);
    let after_with_separator_removed = after
        .split_first()
        .map(|(_char, remaining)| remaining)
        .unwrap_or_default();
    Some((before, after_with_separator_removed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_at() {
        let chars: Vec<char> = "[hello] world".chars().collect();
        let result = split_at(&chars, ']');
        assert!(matches!(
            result,
            Some((
                ['[', 'h', 'e', 'l', 'l', 'o'],
                [' ', 'w', 'o', 'r', 'l', 'd']
            ))
        ));
    }

    #[test]
    fn test_split_at_end_separator() {
        let chars: Vec<char> = "[hello]".chars().collect();
        let result = split_at(&chars, ']');
        assert!(matches!(result, Some((['[', 'h', 'e', 'l', 'l', 'o'], []))));
    }

    #[test]
    fn test_split_at_empty() {
        let chars: Vec<char> = vec![']'];
        let result = split_at(&chars, ']');
        assert!(matches!(result, Some(([], []))));
        
        assert!(matches!(split_at(&[], ']'), None));
    }
}
