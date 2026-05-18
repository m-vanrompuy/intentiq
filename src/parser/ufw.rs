use regex::Regex;
use crate::models::event::Event;

pub fn parse(contents: &str) -> Vec<Event> {
    let mut events = Vec::new();

    //vec van tuples (Regex, &str)
    let patterns = vec![ 
        (   //May  6 09:20:01 server kernel: [UFW BLOCK] IN=eth0 SRC=45.33.32.156 DST=192.168.1.5 PROTO=TCP SPT=34567 DPT=80
            Regex::new(r"(\w+\s+\d+\s+\d+:\d+:\d+).*SRC=([\d.]+)\s+DST=").unwrap(),
            "firewall_block", 
        ),
    ];

    for line in contents.lines() { 
        for (pattern, event_type) in &patterns {
            if let Some(caps) = pattern.captures(line) {
                
                let timestamp = match caps.get(1) {
                    Some(m) => m.as_str().to_string(),
                    None => "".to_string(),
                };
                
                let user = None;

                let ip = match caps.get(2) {
                    Some(m) => Some(m.as_str().to_string()),
                    None => None,
                };

                let command = None;

                let size = None;

                let event = Event {
                    timestamp,
                    source: "ufw".to_string(),
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