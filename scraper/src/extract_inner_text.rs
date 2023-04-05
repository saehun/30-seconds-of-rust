use crate::index_of::*;

pub fn extract_inner_text(target: &str, lefts: &[&str], right: &str) -> String {
    let start_idx = find_start_index(lefts, 0, target);
    if start_idx == usize::MAX {
        return "".to_string();
    }

    let end_idx = if !right.is_empty() {
        index_of_with_start(right, start_idx, target).unwrap_or(usize::MAX)
    } else {
        target.len()
    };

    if end_idx == usize::MAX {
        return "".to_string();
    }

    target[start_idx..end_idx].to_string()
}

fn find_start_index(lefts: &[&str], start_idx: usize, target: &str) -> usize {
    let mut before_start_idx = start_idx;
    for &left in lefts {
        let idx = index_of_with_start(left, before_start_idx, target);
        match idx {
            Some(i) => {
                before_start_idx = i + left.len();
            }
            None => {
                return usize::MAX;
            }
        }
    }
    before_start_idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_inner_text_case_1() {
        let data = "안녕하세요. 저는 개발자 김인성이라고 합니다.";
        let actual = extract_inner_text(data, &["개발자 "], "이라고");
        assert_eq!(actual, "김인성");
    }

    #[test]
    fn test_extract_inner_text_case_2() {
        let data = r#"안녕하세요. 저는 개발자 "김인성" 이라고 합니다."#;
        let actual = extract_inner_text(data, &["개발자", "\""], "\" ");
        assert_eq!(actual, "김인성");
    }

    #[test]
    fn test_extract_inner_text_case_3() {
        let data = r#"<div id="test_case" value="test result">"#;
        let actual = extract_inner_text(data, &["<div", "value=", "\""], "\"");
        assert_eq!(actual, "test result");
    }

    #[test]
    fn test_extract_inner_text_case_4() {
        let data = r#"["value1","value2","value3","value4"]"#;
        let actual = extract_inner_text(data, &[",", ",", "\""], "\"");
        assert_eq!(actual, "value3");
    }

    #[test]
    fn test_extract_inner_text_case_5() {
        let data = "||||result||";
        let actual = extract_inner_text(data, &["|", "|", "|", "|"], "|");
        assert_eq!(actual, "result");
    }

    #[test]
    fn test_extract_inner_text_no_match() {
        let data = "||||result||";
        let actual = extract_inner_text(data, &["|", ">"], "|");
        assert_eq!(actual, "");
    }
}
