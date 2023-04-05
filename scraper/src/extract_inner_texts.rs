use crate::index_of::*;

pub fn extract_inner_texts(target: &str, left: &str, right: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut start_idx = 0;

    while let Some(left_idx) = index_of_with_start(left, start_idx, target) {
        let start = left_idx + left.len();
        if let Some(right_idx) = index_of_with_start(right, start, target) {
            results.push(target[start..right_idx].to_string());
            start_idx = right_idx + right.len() - 1;
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
    fn test_extract_inner_texts_case_1() {
        let data = "<tr><td>1번째 줄</td><td>2번째 줄</td><td>3번째 줄</td><td>4번째 줄</td><tr>";
        let actual = extract_inner_texts(data, "<td>", "</td>");
        assert_eq!(actual, vec!["1번째 줄", "2번째 줄", "3번째 줄", "4번째 줄"]);
    }

    #[test]
    fn test_extract_inner_texts_case_2() {
        let data = "|1|2|3|4|";
        let actual = extract_inner_texts(data, "|", "|");
        assert_eq!(actual, vec!["1", "2", "3", "4"]);
    }

    #[test]
    fn test_extract_inner_texts_case_3() {
        let data = "|1|2|3|4|";
        let actual = extract_inner_texts(data, "<td", "|");
        assert_eq!(actual, Vec::<String>::new());
    }

    #[test]
    fn test_extract_inner_texts_case_4() {
        let data = "|1|2|3|4|";
        let actual = extract_inner_texts(data, "|", "<td");
        assert_eq!(actual, Vec::<String>::new());
    }

    #[test]
    fn test_extract_inner_texts_case_5() {
        let raw = "s += ';DENx3uLXFWFG7F81wapfDBnGTZA=';";
        let actual = extract_inner_texts(raw, "s += '", "';");
        assert_eq!(actual, vec![";DENx3uLXFWFG7F81wapfDBnGTZA="]);
    }
}
