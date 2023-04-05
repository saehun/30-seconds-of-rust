use crate::index_of::*;
use crate::string_utils::*;

pub fn extract_outer_texts(target: &str, left: &str, right: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut start_idx = 0;

    while let Some(left_idx) = index_of_with_start(left, start_idx, target) {
        let start = left_idx + left.len();
        if let Some(right_idx) = index_of_with_start(right, start, target) {
            let outer_text = format!("{}{}{}", left, &target[start..right_idx], right);
            results.push(outer_text);
            start_idx = right_idx + strip_last_char(right).len();
        } else {
            break;
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_outer_texts_case_1() {
        let target = "동해물과 백두산이 마르고 닳도록, 백두산이 손발이 닳도록";
        let left = "백두산이";
        let right = "닳도록";
        let expected = vec!["백두산이 마르고 닳도록", "백두산이 손발이 닳도록"];
        assert_eq!(extract_outer_texts(target, left, right), expected);
    }

    #[test]
    fn test_extract_outer_texts_case_4() {
        let target = "|1|2|3|4|";
        let left = "|";
        let right = "|";
        let expected = vec!["|1|", "|2|", "|3|", "|4|"];
        assert_eq!(extract_outer_texts(target, left, right), expected);
    }

    #[test]
    fn test_extract_outer_texts_case_6() {
        let target = "|1|2|3|4|";
        let left = "<td";
        let right = "|";
        let expected = Vec::<String>::new();
        assert_eq!(extract_outer_texts(target, left, right), expected);
    }

    #[test]
    fn test_extract_outer_texts_case_7() {
        let target = "|1|2|3|4|";
        let left = "|";
        let right = "<td";
        let expected = Vec::<String>::new();
        assert_eq!(extract_outer_texts(target, left, right), expected);
    }

    #[test]
    fn test_extract_outer_texts_case_8() {
        let target = "s += ';DENx3uLXFWFG7F81wapfDBnGTZA=';";
        let left = "s += '";
        let right = "';";
        let expected = vec!["s += ';DENx3uLXFWFG7F81wapfDBnGTZA=';"];
        assert_eq!(extract_outer_texts(target, left, right), expected);
    }

    #[test]
    fn test_extract_outer_texts_case_10() {
        let target = "111김인성111";
        let left = "11";
        let right = "11";
        let expected = vec!["111김인성11"];
        assert_eq!(extract_outer_texts(target, left, right), expected);
    }

    #[test]
    fn test_extract_outer_texts_case_11() {
        let target = "111김인성111";
        let left = "1";
        let right = "111";
        let expected = vec!["111김인성111"];
        assert_eq!(extract_outer_texts(target, left, right), expected);
    }
}
