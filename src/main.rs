mod parser;
mod models;
mod grouping;
mod detection;
mod storage;
mod output;

use models::event::{Event, IntentResult};
use std::fs::read_to_string;
use parser::auth::parse;

fn main() {
    //sudo command fixen!! 
    println!("intentiq - cybersecurity log analyzer");
    let contents = read_to_string("logs/auth.log").expect("Could not read file");
    let parsed_content = parse(&contents);
    println!("{:#?}", parsed_content);

    
}