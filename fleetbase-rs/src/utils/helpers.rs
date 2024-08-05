use regex::Regex;

pub fn is_phone(s: &str) -> bool {
    let re = Regex::new(r"^\+?[\d\s-]+$").unwrap();
    re.is_match(s)
}
