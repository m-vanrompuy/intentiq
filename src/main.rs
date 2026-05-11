mod parser;
mod models;
mod grouping;
mod detection;
mod storage;
mod output;

use models::event::{Event, IntentResult};
use std::fs::read_to_string;
use parser::auth::parse as parse_auth;
use parser::syslog::parse as parse_syslog;
use parser::ufw::parse as parse_ufw;


fn main() { 
    println!("intentiq - cybersecurity log analyzer");
    // let authcontents = read_to_string("logs/auth.log").expect("Could not read file");
    // let parsed_content = parse_auth(&authcontents);
    // println!("{:#?}", parsed_content);

    // let syslogcontents = read_to_string("logs/syslog").expect("Could not read file");
    // let parsed_content = parse_syslog(&syslogcontents);
    // println!("{:#?}", parsed_content);

    let ufwcontents = read_to_string("logs/ufw.log").expect("Could not read file");
    let parsed_content = parse_ufw(&ufwcontents);
    println!("{:#?}", parsed_content);
}