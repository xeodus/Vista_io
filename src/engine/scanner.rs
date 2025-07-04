use regex::Regex;
use crate::engine::types::{Match, ScanResult};

pub fn scan_text(input: &str) -> ScanResult {
    let mut matches = Vec::new();
    let email_re = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-z]{2,}").unwrap();

    for i in email_re.find_iter(input) {
        matches.push(Match {
            type_: super::types::MatchType::Email,
            value: i.as_str().to_string(),
            position: i.start()
        });
    }

    let phone_re = Regex::new(r"\b\d{10}\b").unwrap();

    for j in phone_re.find_iter(input) {
        matches.push(Match {
            type_: super::types::MatchType::Phone,
            value: j.as_str().to_string(),
            position: j.start()
        });
    }

    let creditcard_re = Regex::new(r"\b(?:\d[\s-]?){13,19}\b").unwrap();

    for k in creditcard_re.find_iter(input) {
        matches.push(Match {
            type_: super::types::MatchType::CreditCard,
            value: k.as_str().to_string(),
            position: k.start()
        });
    }

    let name_re = Regex::new(r"\b(?:\d[\s-]?){13,19}\b").unwrap();

    for l in name_re.find_iter(input) {
        matches.push(Match {
            type_: super::types::MatchType::Name,
            value:l.as_str().to_string(),
            position: l.start()
        });
    }

    ScanResult { original: input.to_string(), matches }
}