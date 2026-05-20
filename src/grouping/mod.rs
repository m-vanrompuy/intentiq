

use std::collections::HashMap;
use chrono::NaiveDateTime;

use crate::models::event::{ActorSummary, Event, IntentResult};
use crate::utils::parse_timestamp;

// events groeperen per actor (enkele soort)
pub fn group(events: Vec<Event>) -> HashMap<String, Vec<Event>>{

    let mut actors: HashMap<String, Vec<Event>> = HashMap::new();

    for event in events{
        let actor = match &event.ip { 
            Some(ip) => ip.clone(),
            None => match &event.user {
                Some(user) => user.clone(),
                None => continue, 
            }
        };

        actors.entry(actor)
        .or_insert_with(Vec::new)
        .push(event);
    }

   return actors;
}

// linken van actors na een succesfull login en dus geen user meer heeft.
pub fn link_orphan_events(actors: &mut HashMap<String, Vec<Event>>, all_events: &Vec<Event>) {
    let mut successful_logins: Vec<(String, NaiveDateTime)> = Vec::new();
    let mut userless_events:Vec<Event> = Vec::new();

    for (actor, events) in actors.iter(){
        for event in events{
            if event.event_type == "ssh_login_success" {
                if let Some(ts) = parse_timestamp(&event.timestamp) {
                    successful_logins.push((actor.clone(), ts));
                } 
            }
        }
    }
    
    for event in all_events.iter(){
        if event.ip.is_none() && event.user.is_none(){
            userless_events.push(event.clone());
        }

    }

    for event in userless_events{
        if let Some(event_ts) = parse_timestamp(&event.timestamp){

            let matching_actor = successful_logins.iter()
                .filter(|(_, login_ts)| {
                    let diff = event_ts.signed_duration_since(*login_ts);
                    diff.num_seconds() >= 0 && diff.num_seconds() <= 300
                })
                .min_by_key(|(_, login_ts)| {
                    event_ts.signed_duration_since(*login_ts).num_seconds()
                });

            if let Some((actor, _)) = matching_actor { 
                actors.entry(actor.clone())
                .or_insert_with(Vec::new)
                .push(event.clone());
            }
        }
    }
}

//alle soorten intents results groeperen per actor na analyze()
pub fn aggregate_results(results:Vec<IntentResult>) -> HashMap<String, Vec<IntentResult>>{

    let mut results_per_actor: HashMap<String, Vec<IntentResult>> = HashMap::new();

    for result in results {
        results_per_actor
            .entry(result.actor.clone())
            .or_insert_with(Vec::new)
            .push(result);
    }

    return  results_per_actor;
}

//maakt een summary van ALLE intents / results per actor met probalistische confidence totaal
pub fn summarize(actorresults:HashMap<String, Vec<IntentResult>>) -> Vec<ActorSummary>{
    let mut summaries: Vec<ActorSummary> = Vec::new();

    for (actor, results) in actorresults{
        let intents: Vec<String> = results.iter()
        .map(|r| r.intent.clone())
        .collect();

        let evidence: Vec<String> = results.iter()
        .flat_map(|r| r.evidence.clone())
        .collect();

        //probalistische formule e.g. 1 - (1-0.75) * (1-0.85) * (1-0.88) = 0.995
        let total_confidence = 1.0 - results.iter()
        .fold(1.0, |acc, r| acc * (1.0 - r.confidence));

        summaries.push(ActorSummary { 
            actor, 
            intents,
            total_confidence,
            evidence,
        });
    }

    return summaries;
}