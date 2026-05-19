use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub timestamp: String,
    pub source: String,
    pub user: Option<String>,
    pub ip: Option<String>,
    pub event_type: String,
    pub command: Option<String>,
    pub message: String,
    pub size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntentResult {
    pub actor: String,
    pub intent: String,
    pub confidence: f64,
    pub evidence: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActorSummary {
    pub actor: String,
    pub intents: Vec<String>,
    pub total_confidence: f64,
    pub evidence: Vec<String>,
}