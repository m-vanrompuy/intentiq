use crate::models::event::Event;
use crate::models::event::IntentResult;

//kijkt of er iets suspicious is
pub fn analyze(actor: &str, events: &Vec<Event>) -> Vec<IntentResult> {
    let mut results: Vec<IntentResult> = Vec::new();
    let mut login_count: i32 = 0;
    let mut failed_login_messages: Vec<String> = Vec::new();
    let mut success_login_message: Option<String> = None;

    let mut had_session = false;
    let mut had_sudo = false;
    let mut sudo_messages: Vec<String> = Vec::new();
    let mut had_suspicious_binary = false;
    let mut suspicious_binary_messages: Vec<String> = Vec::new();

    let mut firewall_block_count: i32 = 0;
    let mut firewall_block_messages: Vec<String> = Vec::new();

    let mut data_exfiltration: Vec<String> = Vec::new();

    for event in events {
        if event.event_type == "ssh_login_failed" {
            login_count += 1;
            failed_login_messages.push(event.message.clone());
        } 
        
        if event.event_type == "ssh_login_success" {
            success_login_message = Some(event.message.clone());
        }

        if event.event_type == "session_started" {
            had_session = true;
        }

       if event.event_type == "sudo_command" {
            if let Some(cmd) = &event.command {
                if cmd.contains("/tmp") || cmd.contains("/bin/bash") || cmd.contains("wget") {
                    had_sudo = true;
                    sudo_messages.push(event.message.clone());
                }
            }
        }

        if event.event_type == "suspicious_binary" {
            had_suspicious_binary = true; 
            suspicious_binary_messages.push(event.message.clone());
        }

        if event.event_type == "firewall_block" {
            firewall_block_count += 1; 
            firewall_block_messages.push(event.message.clone());
        }

        if event.event_type == "http_success" {
            if let Some(size) = event.size {
                if size >= 50000 {
                    data_exfiltration.push(event.message.clone());
                }
            }
        }


    }

    //reconnaissance rule
    if login_count >= 10 {
        results.push(IntentResult {
            actor: actor.to_string(),
            intent: "reconnaissance".to_string(),
            confidence: 0.75,
            evidence: failed_login_messages.clone(),
        });
    }

    //credential_access rule
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

    if (had_session || success_login_message.is_some()) && had_sudo && had_suspicious_binary {
        results.push(IntentResult {
            actor: actor.to_string(),
            intent: "privilege_escalation + persistence".to_string(),
            confidence: 0.90,
            evidence: {
                let mut e = sudo_messages.clone();
                e.extend(suspicious_binary_messages.clone());
                e
            },
        });
    } else if had_session && had_sudo {
        results.push(IntentResult {
            actor: actor.to_string(),
            intent: "privilege_escalation".to_string(),
            confidence: 0.70,
            evidence: sudo_messages.clone(),
        });
    } else if had_suspicious_binary {
        results.push(IntentResult {
            actor: actor.to_string(),
            intent: "persistence".to_string(),
            confidence: 0.50,
            evidence: suspicious_binary_messages.clone(),
        });
    }

    // reconnaissance rule / port scanning via firewall blocks
    if firewall_block_count >= 5 {
        results.push(IntentResult {
            actor: actor.to_string(),
            intent: "reconnaissance".to_string(),
            confidence: 0.80,
            evidence: firewall_block_messages.clone(),
        });
    }

    // data_exfiltration rule
    if !data_exfiltration.is_empty() {
        results.push(IntentResult {
            actor: actor.to_string(),
            intent: "data_exfiltration".to_string(),
            confidence: 0.88,
            evidence: data_exfiltration.clone(),
        });
    }

    results
}