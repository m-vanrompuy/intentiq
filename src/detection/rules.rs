use crate::models::event::Event;
use crate::models::event::IntentResult;

pub fn analyze(actor: &str, events: &Vec<Event>) -> Vec<IntentResult> {
    let mut results: Vec<IntentResult> = Vec::new();
    let mut login_count: i32 = 0;
    let mut failed_login_messages: Vec<String> = Vec::new();
    let mut success_login_message: Option<String> = None;

    for event in events {
        if event.event_type == "ssh_login_failed" {
            login_count += 1;
            failed_login_messages.push(event.message.clone());
        } else if event.event_type == "ssh_login_success" {
            success_login_message = Some(event.message.clone());
        }
    }

    if login_count >= 10 {
        results.push(IntentResult {
            actor: actor.to_string(),
            intent: "reconnaissance".to_string(),
            confidence: 0.75,
            evidence: failed_login_messages.clone(),
        });
    }

    if login_count > 3 && success_login_message.is_some() {
        let mut evidence = failed_login_messages.clone();
        if let Some(msg) = &success_login_message {
            evidence.push(msg.clone());
        }
        results.push(IntentResult {
            actor: actor.to_string(),
            intent: "credential_access".to_string(),
            confidence: 0.85,
            evidence,
        });
    }

    return results;
}