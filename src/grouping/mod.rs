

use std::collections::HashMap;
use chrono::NaiveDateTime;

use crate::models::event::{ActorSummary, Event, IntentResult};
use crate::utils::parse_timestamp;

// events groeperen per actor (enkele soort)
pub fn group_events_per_actor(events: Vec<Event>) -> HashMap<String, Vec<Event>>{

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

    actors
}

// linken van actors na een succesfull login en dus geen user meer heeft.
pub fn link_orphan_events(actors: &mut HashMap<String, Vec<Event>>, all_events: &Vec<Event>) {
    let successful_logins = collect_successful_logins(actors);
    let orphan_events = collect_orphan_events(all_events);
    let mut linked_users: Vec<String> = Vec::new();

    for event in orphan_events {
        if let Some(actor) = find_matching_actor(&event, &successful_logins) {
            track_linked_user(&event, &mut linked_users);
            actors.entry(actor).or_insert_with(Vec::new).push(event);
        }
    }

    for user in linked_users {
        actors.remove(&user);
    }
}

    
fn track_linked_user(event: &Event, linked_users: &mut Vec<String>) {
    if let Some(user) = &event.user {
        if !linked_users.contains(user) {
            linked_users.push(user.clone());
        }
    }
}
    //wie heeft wanneer ingelogd lijst
fn collect_successful_logins(actors: &HashMap<String, Vec<Event>>) -> Vec<(String, NaiveDateTime)> {
    actors.iter()
        .flat_map(|(actor, events)| {
            events.iter()
                .filter(|e| e.event_type == "ssh_login_success")
                .filter_map(|e| parse_timestamp(&e.timestamp).map(|ts| (actor.clone(), ts)))
        })
        .collect()
}

    //lijst van events zonder ip en zonder user
fn collect_orphan_events(all_events: &Vec<Event>) -> Vec<Event> {
    all_events.iter()
        .filter(|e| e.ip.is_none() && (e.user.is_none() || e.event_type == "sudo_command"))
        .cloned()
        .collect()
}

    //gegeven een orphan event: zoekt dichtst bijzijnde login binnen 5 min
fn find_matching_actor(event: &Event, logins: &Vec<(String, NaiveDateTime)>) -> Option<String> {
    let event_ts = parse_timestamp(&event.timestamp)?;
    logins.iter()
        .filter(|(_, login_ts)| {
            let diff = event_ts.signed_duration_since(*login_ts);
            diff.num_seconds() >= 0 && diff.num_seconds() <= 300
        })
        .min_by_key(|(_, login_ts)| event_ts.signed_duration_since(*login_ts).num_seconds())
        .map(|(actor, _)| actor.clone())
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

    results_per_actor
}

//maakt een summary van ALLE intents / results per actor met avg confidence totaal
pub fn summarize(actorresults:HashMap<String, Vec<IntentResult>>) -> Vec<ActorSummary>{
    let mut summaries: Vec<ActorSummary> = Vec::new();

    for (actor, results) in actorresults{
        let mut intents: Vec<String> = results.iter()
        .map(|r| r.intent.clone())
        .collect();
        intents.sort();
        intents.dedup();

        let mut evidence: Vec<String> = results.iter()
        .flat_map(|r| r.evidence.clone())
        .collect();
        evidence.sort();
        evidence.dedup();

        //avg formule
        let total_confidence = results.iter().map(|r| r.confidence)
        .sum::<f64>() / results.len() as f64;

        summaries.push(ActorSummary { 
            actor, 
            intents,
            total_confidence,
            evidence,
        });
    }

    summaries
}