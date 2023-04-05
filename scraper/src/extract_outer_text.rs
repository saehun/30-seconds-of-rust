use crate::index_of::*;

pub fn extract_outer_text(target: &str, lefts: &[&str], right: &str) -> String {
    let left_start_idx = find_start_index(lefts, 0, target);
    if left_start_idx == usize::MAX {
        return "".to_string();
    }

    let left_end_idx = left_start_idx - lefts.last().unwrap().len();
    let right_start_idx = if !right.is_empty() {
        index_of_with_start(right, left_start_idx, target).unwrap_or(usize::MAX)
    } else {
        target.len()
    };

    if right_start_idx == usize::MAX {
        return "".to_string();
    }

    let right_end_idx = right_start_idx + right.len();
    target[left_end_idx..right_end_idx].to_string()
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
    fn test_extract_outer_text_case_1() {
        let data = "안녕하세요. 저는 개발자 김개발이라고 합니다.";
        let actual = extract_outer_text(data, &["개발자"], "이라고");
        assert_eq!(actual, "개발자 김개발이라고");
    }

    #[test]
    fn test_extract_outer_text_case_2() {
        let data = "안녕하세요. 저는 개발자 김개발이라고 합니다.";
        let actual = extract_outer_text(data, &["저는", "개발자"], "이라고");
        assert_eq!(actual, "개발자 김개발이라고");
    }

    #[test]
    fn test_extract_outer_text_case_3() {
        let data = r#"<div id="test_case" value="test result">"#;
        let actual = extract_outer_text(data, &["<div", "value=", r#"""#], r#"""#);
        assert_eq!(actual, r#""test result""#);
    }

    #[test]
    fn test_extract_outer_text_case_6() {
        let data = r#"["value1","value2","value3","value4"]"#;
        let actual = extract_outer_text(data, &[",", ",", r#"""#], r#"""#);
        assert_eq!(actual, r#""value3""#);
    }

    #[test]
    fn test_extract_outer_text_case_7() {
        let data = "||||result||";
        let actual = extract_outer_text(data, &["|", "|", "|", "|"], "|");
        assert_eq!(actual, "|result|");
    }

    #[test]
    fn test_extract_outer_text_no_match() {
        let data = "||||result||";
        let actual = extract_outer_text(data, &["|", ">"], "|");
        assert_eq!(actual, "");
    }
}
