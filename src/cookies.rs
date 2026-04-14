pub fn parse_cookie_value(cookie_header: &str, name: &str) -> Option<String> {
    cookie_header
        .split(';')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .find_map(|part| {
            let (key, value) = part.split_once('=').unwrap_or((part, ""));
            if key == name {
                Some(value.to_string())
            } else {
                None
            }
        })
}

pub fn build_cookie_header(name: &str, value: &str, days: i32) -> String {
    let max_age = days.saturating_mul(24 * 60 * 60);
    format!("{name}={value}; path=/; max-age={max_age}")
}

#[cfg(test)]
mod tests {
    use super::{build_cookie_header, parse_cookie_value};

    #[test]
    fn parse_cookie_value_finds_first_matching_cookie() {
        let header = "a=1; preferred=it; preferred=en; c=3";
        assert_eq!(parse_cookie_value(header, "preferred"), Some("it".to_string()));
    }

    #[test]
    fn parse_cookie_value_handles_surrounding_spaces() {
        let header = "a=1;   preferred =it ; x=2";
        assert_eq!(parse_cookie_value(header, "preferred "), Some("it".to_string()));
        assert_eq!(parse_cookie_value(header, "preferred"), None);
    }

    #[test]
    fn parse_cookie_value_handles_empty_cookie_value() {
        let header = "a=1; preferred=; c=3";
        assert_eq!(parse_cookie_value(header, "preferred"), Some(String::new()));
    }

    #[test]
    fn parse_cookie_value_returns_none_when_cookie_missing() {
        let header = "a=1; b=2";
        assert_eq!(parse_cookie_value(header, "preferred"), None);
    }

    #[test]
    fn parse_cookie_value_handles_single_cookie_no_separator() {
        assert_eq!(parse_cookie_value("preferred=fr", "preferred"), Some("fr".to_string()));
    }

    #[test]
    fn parse_cookie_value_supports_value_with_equals_sign() {
        let header = "token=a=b=c; x=1";
        assert_eq!(parse_cookie_value(header, "token"), Some("a=b=c".to_string()));
    }

    #[test]
    fn parse_cookie_value_ignores_empty_segments() {
        let header = "a=1;; ; preferred=de; ;";
        assert_eq!(parse_cookie_value(header, "preferred"), Some("de".to_string()));
    }

    #[test]
    fn parse_cookie_value_does_not_match_partial_cookie_name() {
        let header = "preferred_locale=it; preferred=en";
        assert_eq!(parse_cookie_value(header, "preferred"), Some("en".to_string()));
    }

    #[test]
    fn build_cookie_header_contains_expected_parts() {
        let cookie = build_cookie_header("preferred", "it", 365);
        assert_eq!(cookie, "preferred=it; path=/; max-age=31536000");
    }

    #[test]
    fn build_cookie_header_allows_zero_day_expiry() {
        let cookie = build_cookie_header("preferred", "it", 0);
        assert_eq!(cookie, "preferred=it; path=/; max-age=0");
    }

    #[test]
    fn build_cookie_header_saturates_on_large_day_values() {
        let cookie = build_cookie_header("preferred", "it", i32::MAX);
        assert!(cookie.ends_with("max-age=2147483647"));
    }

    #[test]
    fn build_cookie_header_saturates_on_negative_day_values() {
        let cookie = build_cookie_header("preferred", "it", i32::MIN);
        assert!(cookie.ends_with("max-age=-2147483648"));
    }
}
