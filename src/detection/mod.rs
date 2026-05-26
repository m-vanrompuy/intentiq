pub mod rules;
pub mod llm;

use crate::models::event::{ActorSummary};


pub fn filter_low_confidence(summaries: &Vec<ActorSummary>) -> Vec<ActorSummary>{
    let mut lowconf_results : Vec<ActorSummary> = Vec::new();

    for result in summaries{
        if result.total_confidence > 0.3 && result.total_confidence < 0.95{
            lowconf_results.push(result.clone());
        }
    }

    lowconf_results
}