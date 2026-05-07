use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub timestamp: String,
    pub source: String,
    pub user: Option<String>,
    pub ip: Option<String>,
    pub event_type: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntentResult {
    pub actor: String,
    pub intent: String,
    pub confidence: f64,
    pub evidence: Vec<String>,
}