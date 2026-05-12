use crate::models::event::Event;
use crate::models::event::IntentResult;


pub fn analyze(actor: &str, events: &Vec<Event>) -> Vec<IntentResult>{
    let mut results: Vec<IntentResult> = Vec::new();
    let mut login_count: i32 = 0;
    let mut failed_login_messages: Vec<String> = Vec::new();

    for event in events{
        if event.event_type == "ssh_login_failed"{
            login_count += 1;    
            failed_login_messages.push(event.message.clone());   
        }
    }

    if(login_count >=10 ){
        results.push(IntentResult {
            actor: actor.to_string(),
            intent: "reconnaissance".to_string(),
            confidence: 0.75,
            evidence: failed_login_messages,
        });
    }

    return results;
}