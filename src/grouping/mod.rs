use std::collections::HashMap;

use crate::models::event::Event;

pub fn group(events: Vec<Event>) -> HashMap<String, Vec<Event>>{

    let mut actors: HashMap<String, Vec<Event>> = HashMap::new();

    // toevoegen ip / user , event type
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
