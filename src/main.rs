mod parser;
mod models;
mod grouping;
mod detection;
mod storage;
mod output;

// use models::event::{Event, IntentResult};
use std::fs::read_to_string;
use parser::auth::parse as parse_auth;
use parser::syslog::parse as parse_syslog;
use parser::ufw::parse as parse_ufw;
use parser::nginx::parse as parse_nginx;
use grouping::group;
use detection::rules::analyze;
use storage::mongo;




#[tokio::main]
async fn main() {
    println!(r#"
 _       _             _   _         
(_)_ __ | |_ ___ _ __ | |_(_) __ _  
| | '_ \| __/ _ \ '_ \| __| |/ _` | 
| | | | | ||  __/ | | | |_| | (_| | 
|_|_| |_|\__\___|_| |_|\__|_|\__, | 
                               |___/  
"#);
    println!("  cybersecurity log analyzer\n");

    let client = mongo::connect().await;

    let authcontents = read_to_string("logs/auth.log").expect("Could not read file");
    let parsed_content1 = parse_auth(&authcontents);
    // println!("{:#?}", parsed_content1);

    let syslogcontents = read_to_string("logs/syslog").expect("Could not read file");
    let parsed_content2 = parse_syslog(&syslogcontents);
    // println!("{:#?}", parsed_content2);

    let ufwcontents = read_to_string("logs/ufw.log").expect("Could not read file");
    let parsed_content3 = parse_ufw(&ufwcontents);
    // println!("{:#?}", parsed_content3);

    let nginxcontents = read_to_string("logs/nginx/access.log").expect("Could not read file");
    let parsed_content4 = parse_nginx(&nginxcontents);
    // println!("{:#?}", parsed_content4);

    let mut all_events = Vec::new();
    all_events.extend(parsed_content1);
    all_events.extend(parsed_content2);
    all_events.extend(parsed_content3);
    all_events.extend(parsed_content4);
    let all_events_copy = all_events.clone();
    let actors = group(all_events);
    // println!("{:#?}",actors);

    let mut all_results= Vec::new();

    for (actor, events) in &actors {
    let results = analyze(actor, events);
        if !results.is_empty() {
            all_results.extend(results.clone());
            // println!("{:#?}", results);  
        }
    }

    let results_per_actor = aggregate_results(all_results.clone());
    let actor_total_summary =summarize(results_per_actor) ;
    // println!("{:#?}", actor_total_summary); 

    save_events(&client, all_events_copy).await;
    save_results(&client, all_results).await;
    save_actorsummary(&client, actor_total_summary).await;
}