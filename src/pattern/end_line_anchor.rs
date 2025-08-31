use crate::pattern::{Match, Pattern};

#[derive(Debug)]
struct EndLineAnchor {
    inner_pattern: Box<dyn Pattern>,
}

impl Pattern for EndLineAnchor {
    // todo: this is inefficient, since matches will call this matches_exact function multiple times
    // when really it could just be called once and process the pattern in reverse from the end of the string
    fn matches_exact(&self, chars: &[char]) -> Match {
        self.inner_pattern
            .matches_exact(chars)
            .and_then(|match_struct| {
                if match_struct.end == chars.len() {
                    match_struct.into()
                } else {
                    Match::None
                }
            })
    }

    fn followed_by(self, _: Box<dyn Pattern>) -> Box<dyn Pattern>
    where
        Self: Sized + 'static,
    {
        panic!("End line anchor shouldn't be followed by any other pattern")
    }
}

pub fn end_line_anchor(pattern: Box<dyn Pattern>) -> impl Pattern {
    EndLineAnchor {
        inner_pattern: pattern,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::literal;

    #[test]
    fn test_start_line_anchor() {
        let pattern = end_line_anchor(literal('a').followed_by(Box::new(literal('b'))));
        assert!(pattern.matches("cab"));
        assert!(pattern.matches("ab"));
        assert!(!pattern.matches("abc"));
    }
}
