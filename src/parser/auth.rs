use mongodb::event::command;
use regex::Regex;
use crate::models::event::Event;

pub fn parse(contents: &str) -> Vec<Event> {
    let mut events = Vec::new();

    //vec van tuples (Regex, &str)
    let patterns = vec![ //unwrap later vervangen voor betere error handling? Regex::new() geeft een Result<Regex, Error>

        (   //e.g. May  6 09:12:01 server sshd[1023]: Accepted password for john from 192.168.1.10 port 53422 ssh2
            Regex::new(r"(\w+\s+\d+\s+\d+:\d+:\d+).*sshd.*Accepted password for (\w+) from ([\d.]+)").unwrap(),
            "ssh_login_success", 
        ),
        (
            Regex::new(r"(\w+\s+\d+\s+\d+:\d+:\d+).*sshd.*Failed password for (?:invalid user )?(\w+) from ([\d.]+)").unwrap(),
            "ssh_login_failed",
        ),
        (
            Regex::new(r"(\w+\s+\d+\s+\d+:\d+:\d+).*sudo:\s+(\w+).*COMMAND=(.+)").unwrap(),
            "sudo_command",
        ),
    ];

    for line in contents.lines() { 
        for (pattern, event_type) in &patterns {
            if let Some(caps) = pattern.captures(line) {
                
                let timestamp = match caps.get(1) {
                    Some(m) => m.as_str().to_string(),
                    None => "".to_string(),
                };

                let user = match caps.get(2) {
                    Some(m) => Some(m.as_str().to_string()),
                    None => None,
                };

                let (ip, command) = match* event_type {
                    "sudo_command" => (None, match caps.get(3){
                        Some(m) => Some(m.as_str().to_string()),
                        None => None,
                     }),
                    
                   _ => (match caps.get(3){
                        Some(m) => Some(m.as_str().to_string()),
                        None => None,
                     } , None),
                };
                
                let event = Event {
                    timestamp,
                    source: "auth.log".to_string(),
                    user,
                    ip,
                    event_type: event_type.to_string(),
                    command,
                    message: line.to_string(),
                };
                events.push(event);
                
                break;

            }
        }
    }

    events
}