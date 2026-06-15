use crate::models::event::ActorSummary;

pub fn print_report(summaries: &Vec<ActorSummary>) {
    for summary in summaries {
        println!("[ACTOR] {}", summary.actor);
        println!("[CONFIDENCE] {:.1}%", summary.total_confidence * 100.0);
        println!("[INTENTS] {}", summary.intents.join(", "));
        println!("[EVIDENCE]");
        for evidence in &summary.evidence {
            println!("  • {}", evidence);
        }
        println!("----------------------------------------");
    }
}