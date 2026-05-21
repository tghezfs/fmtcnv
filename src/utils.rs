use chrono::{DateTime, NaiveDate, NaiveDateTime};

pub fn is_iso_8601(s: &str) -> bool {
    DateTime::parse_from_rfc3339(&s).is_ok()
        || NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S").is_ok()
        || NaiveDate::parse_from_str(&s, "%Y-%m-%d").is_ok()
}

pub fn is_valid_path_str(path: &str) -> bool {
    if path.is_empty() || path.as_bytes().contains(&0) {
        return false;
    }

    path.chars()
        .all(|c| c.is_alphanumeric() || c == ' ' || matches!(c, '/' | '.' | '-' | '_'))
}

#[cfg(test)]
mod tests_iso_8601 {
    use super::*;

    #[test]
    fn test_valid_rfc3339_with_timezone() {
        assert!(is_iso_8601("2023-12-25T10:30:00Z"));
        assert!(is_iso_8601("2023-12-25T10:30:00+00:00"));
        assert!(is_iso_8601("2023-12-25T10:30:00-05:00"));
        assert!(is_iso_8601("2023-12-25T10:30:00+05:30"));
    }

    #[test]
    fn test_valid_rfc3339_with_fractional_seconds() {
        assert!(is_iso_8601("2023-12-25T10:30:00.123Z"));
        assert!(is_iso_8601("2023-12-25T10:30:00.123456Z"));
        assert!(is_iso_8601("2023-12-25T10:30:00.123+00:00"));
    }

    #[test]
    fn test_valid_naive_datetime() {
        assert!(is_iso_8601("2023-12-25T10:30:00"));
        assert!(is_iso_8601("2023-01-01T00:00:00"));
        assert!(is_iso_8601("2023-12-31T23:59:59"));
    }

    #[test]
    fn test_valid_date_only() {
        assert!(is_iso_8601("2023-12-25"));
        assert!(is_iso_8601("2023-01-01"));
        assert!(is_iso_8601("2023-12-31"));
        assert!(is_iso_8601("2024-02-29"));
    }

    #[test]
    fn test_invalid_formats() {
        assert!(!is_iso_8601("2023-13-25"));
        assert!(!is_iso_8601("2023-12-32"));
        assert!(!is_iso_8601("2023-02-29"));
        assert!(!is_iso_8601("2023-12-25T25:00:00"));
        assert!(!is_iso_8601("2023-12-25T10:60:00"));
    }

    #[test]
    fn test_invalid_strings() {
        assert!(!is_iso_8601("hello world"));
        assert!(!is_iso_8601(""));
        assert!(!is_iso_8601("2023"));
        assert!(!is_iso_8601("2023-12"));
        assert!(!is_iso_8601("20231225"));
    }

    #[test]
    fn test_edge_cases() {
        assert!(is_iso_8601("0000-01-01"));
        assert!(is_iso_8601("9999-12-31"));
        assert!(is_iso_8601("1970-01-01T00:00:00Z"));
        assert!(!is_iso_8601("2023-12-25T10:30:00Z extra"));
    }

    #[test]
    fn test_timezone_variations() {
        assert!(is_iso_8601("2023-12-25T10:30:00+00:00"));
        assert!(is_iso_8601("2023-12-25T10:30:00-05:00"));
        assert!(is_iso_8601("2023-12-25T10:30:00+05:30"));
        assert!(is_iso_8601("2023-12-25T10:30:00Z"));
    }

    #[test]
    fn test_midnight_cases() {
        assert!(is_iso_8601("2023-12-25T00:00:00"));
        assert!(is_iso_8601("2023-12-25T00:00:00Z"));
    }

    #[test]
    fn test_leap_year_handling() {
        assert!(is_iso_8601("2024-02-29"));
        assert!(!is_iso_8601("2023-02-29"));
        assert!(is_iso_8601("2000-02-29"));
        assert!(!is_iso_8601("1900-02-29"));
    }

    #[test]
    fn test_month_boundaries() {
        assert!(is_iso_8601("2023-01-31"));
        assert!(is_iso_8601("2023-03-31"));
        assert!(is_iso_8601("2023-04-30"));
        assert!(is_iso_8601("2023-05-31"));
        assert!(!is_iso_8601("2023-04-31"));
        assert!(!is_iso_8601("2023-02-30"));
        assert!(!is_iso_8601("2023-11-31"));
    }

    #[test]
    fn test_various_delimiters() {
        assert!(is_iso_8601("2023-12-25T10:30:00Z"));
        assert!(!is_iso_8601("2023/12/25"));
        assert!(!is_iso_8601("20231225"));
    }
}

#[cfg(test)]
mod tests_valid_out_path_str {
    use super::*;

    #[test]
    fn test_lowercase_letters() {
        assert!(is_valid_path_str("file.txt"));
    }

    #[test]
    fn test_uppercase_letters() {
        assert!(is_valid_path_str("MyFile.TXT"));
    }

    #[test]
    fn test_numbers() {
        assert!(is_valid_path_str("file123.txt"));
    }

    #[test]
    fn test_hyphen() {
        assert!(is_valid_path_str("my-file.txt"));
    }

    #[test]
    fn test_underscore() {
        assert!(is_valid_path_str("my_file.txt"));
    }

    #[test]
    fn test_dot() {
        assert!(is_valid_path_str("file.tar.gz"));
    }

    #[test]
    fn test_slash_directory() {
        assert!(is_valid_path_str("dir/subdir/file.txt"));
    }

    #[test]
    fn test_space() {
        assert!(is_valid_path_str("my file.txt"));
    }

    #[test]
    fn test_unicode_letters() {
        assert!(is_valid_path_str("año.txt"));
        assert!(is_valid_path_str("cañón.txt"));
        assert!(is_valid_path_str("文件.txt"));
    }

    #[test]
    fn test_absolute_path() {
        assert!(is_valid_path_str("/home/user/file.txt"));
    }

    #[test]
    fn test_only_default_name() {
        assert!(is_valid_path_str("output.txt"));
    }

    #[test]
    fn test_empty() {
        assert!(!is_valid_path_str(""));
    }

    #[test]
    fn test_null_byte() {
        assert!(!is_valid_path_str("\0invalid"));
    }

    #[test]
    fn test_null_byte_in_middle() {
        assert!(!is_valid_path_str("fil\0e.txt"));
    }

    #[test]
    fn test_backslash() {
        assert!(!is_valid_path_str("file\\bad.txt"));
    }

    #[test]
    fn test_asterisk() {
        assert!(!is_valid_path_str("*.txt"));
    }

    #[test]
    fn test_question_mark() {
        assert!(!is_valid_path_str("file?.txt"));
    }

    #[test]
    fn test_double_quotes() {
        assert!(!is_valid_path_str("file\"bad\".txt"));
    }

    #[test]
    fn test_single_quotes() {
        assert!(!is_valid_path_str("file'bad'.txt"));
    }

    #[test]
    fn test_less_than() {
        assert!(!is_valid_path_str("file<bad>.txt"));
    }

    #[test]
    fn test_greater_than() {
        assert!(!is_valid_path_str("file>bad.txt"));
    }

    #[test]
    fn test_pipe() {
        assert!(!is_valid_path_str("file|bad.txt"));
    }

    #[test]
    fn test_dollar() {
        assert!(!is_valid_path_str("file$bad.txt"));
    }

    #[test]
    fn test_backtick() {
        assert!(!is_valid_path_str("file`bad`.txt"));
    }

    #[test]
    fn test_exclamation() {
        assert!(!is_valid_path_str("file!bad.txt"));
    }

    #[test]
    fn test_at() {
        assert!(!is_valid_path_str("file@bad.txt"));
    }

    #[test]
    fn test_hash() {
        assert!(!is_valid_path_str("file#bad.txt"));
    }

    #[test]
    fn test_percent() {
        assert!(!is_valid_path_str("file%bad.txt"));
    }

    #[test]
    fn test_caret() {
        assert!(!is_valid_path_str("file^bad.txt"));
    }

    #[test]
    fn test_ampersand() {
        assert!(!is_valid_path_str("file&bad.txt"));
    }

    #[test]
    fn test_open_parenthesis() {
        assert!(!is_valid_path_str("file(bad.txt"));
    }

    #[test]
    fn test_close_parenthesis() {
        assert!(!is_valid_path_str("file)bad.txt"));
    }

    #[test]
    fn test_plus() {
        assert!(!is_valid_path_str("file+bad.txt"));
    }

    #[test]
    fn test_equals() {
        assert!(!is_valid_path_str("file=bad.txt"));
    }

    #[test]
    fn test_open_bracket() {
        assert!(!is_valid_path_str("file[bad.txt"));
    }

    #[test]
    fn test_close_bracket() {
        assert!(!is_valid_path_str("file]bad.txt"));
    }

    #[test]
    fn test_open_brace() {
        assert!(!is_valid_path_str("file{bad.txt"));
    }

    #[test]
    fn test_close_brace() {
        assert!(!is_valid_path_str("file}bad.txt"));
    }

    #[test]
    fn test_colon() {
        assert!(!is_valid_path_str("file:bad.txt"));
    }

    #[test]
    fn test_semicolon() {
        assert!(!is_valid_path_str("file;bad.txt"));
    }

    #[test]
    fn test_comma() {
        assert!(!is_valid_path_str("file,bad.txt"));
    }

    #[test]
    fn test_tilde() {
        assert!(!is_valid_path_str("~/file.txt"));
    }

    #[test]
    fn test_tab() {
        assert!(!is_valid_path_str("file\tbad.txt"));
    }

    #[test]
    fn test_newline() {
        assert!(!is_valid_path_str("file\nbad.txt"));
    }

    #[test]
    fn test_carriage_return() {
        assert!(!is_valid_path_str("file\rbad.txt"));
    }

    #[test]
    fn test_multiple_invalid_chars() {
        assert!(!is_valid_path_str("file!@#$.txt"));
    }

    #[test]
    fn test_only_invalid_chars() {
        assert!(!is_valid_path_str("!@#$%^&*()"));
    }
}
