use regex::Regex;
use crate::models::event::Event;

pub fn parse(contents: &str) -> Vec<Event> {
    let mut events = Vec::new();

    //vec van tuples (Regex, &str)
    let patterns = vec![ 
        (   //e.g. May  6 09:16:10 server systemd[1]: Started Session 102 of user root.
            Regex::new(r"(\w+\s+\d+\s+\d+:\d+:\d+).*Started Session \d+ of user (\w+)").unwrap(),
            "session_started", 
        ),
        (
            Regex::new(r"(\w+\s+\d+\s+\d+:\d+:\d+).*suspicious binary executed: (\S+)").unwrap(),
            "suspicious_binary",
        ),
        (
            Regex::new(r"(\w+\s+\d+\s+\d+:\d+:\d+).*Started Process ID \d+ \((\S+)\)").unwrap(),
            "process_started",
        ),
    ];

    for line in contents.lines() { 
        for (pattern, event_type) in &patterns {
            if let Some(caps) = pattern.captures(line) {
                
                let timestamp = match caps.get(1) {
                    Some(m) => m.as_str().to_string(),
                    None => "".to_string(),
                };
                
                let ip = None;
            
                let (user, command) = match *event_type {
                    "session_started" => (match caps.get(2) {
                        Some(m) => Some(m.as_str().to_string()),
                        None => None,
                    }, None),
                    _ => (None, match caps.get(2) {
                        Some(m) => Some(m.as_str().to_string()),
                        None => None,
                    }),
                };

                let size = None;

                let event = Event {
                    timestamp,
                    source: "syslog".to_string(),
                    user,
                    ip,
                    event_type: event_type.to_string(),
                    command,
                    message: line.to_string(),
                    size,
                };
            
                events.push(event);
                
                break;

            }
        }
    }
    events
}