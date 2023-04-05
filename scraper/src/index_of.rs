pub fn index_of_with_start(pattern: &str, start_index: usize, text: &str) -> Option<usize> {
    generic_index_of(pattern, text, start_index)
}

pub fn index_of(pattern: &str, text: &str) -> Option<usize> {
    generic_index_of(pattern, text, 0)
}

fn generic_index_of(pattern: &str, text: &str, start_index: usize) -> Option<usize> {
    let substr = &text[start_index..];
    substr.find(pattern).map(|index| index + start_index)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_index_of_with_pattern_and_start() {
        assert_eq!(index_of_with_start("hello", 0, "hello world"), Some(0));
        assert_eq!(index_of_with_start("hello", 5, "hello world"), None);
        assert_eq!(index_of_with_start("hello", 5, "world hello"), Some(6));
        assert_eq!(
            index_of_with_start("안녕하세요", 0, "한글 안녕하세요 반갑습니다"),
            Some(7)
        );
    }

    #[test]
    fn test_index_of_with_pattern_and_text() {
        assert_eq!(index_of("hello", "hello world"), Some(0));
        assert_eq!(index_of("hello", "world hello"), Some(6));
        assert_eq!(index_of("hello", "world"), None);
    }
}
