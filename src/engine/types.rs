use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MatchType {
    Email,
    Phone,
    CreditCard,
    Name
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Match {
    pub type_: MatchType,
    pub value: String,
    pub position: usize
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScanResult {
    pub original: String,
    pub matches: Vec<Match>
}