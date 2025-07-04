use crate::engine::types::ScanResult;

pub fn redact_text(scan: &ScanResult) -> String {
    let mut redacted = scan.original.clone();
    let mut sorted_matches = scan.matches.clone();

    sorted_matches.sort_by_key(|m| -(m.position as isize));

    for i in sorted_matches {
        let len = i.value.len();
        redacted.replace_range(i.position.. i.position + len, "REDACTED");
    }
    redacted
}