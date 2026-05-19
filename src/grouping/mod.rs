use std::collections::HashMap;

use crate::models::event::{ActorSummary, Event, IntentResult};

// events groeperen per actor
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

//intents / results groeperen per actor
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