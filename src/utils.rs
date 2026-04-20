use chrono::{ DateTime, NaiveDateTime, NaiveDate };

pub fn is_iso_8601(s: &str) -> bool {
     DateTime::parse_from_rfc3339(&s).is_ok() || 
        NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S").is_ok() ||
        NaiveDate::parse_from_str(&s, "%Y-%m-%d").is_ok()
}
