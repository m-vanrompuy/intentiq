use regex::Regex;
use crate::models::event::Event;

pub fn parse(contents: &str) -> Vec<Event> {
    let mut events = Vec::new();

    let patterns = vec![ 
        (  //e.g. 192.168.1.10 - - [06/May/2026:09:10:01 +0000] "GET /index.html HTTP/1.1" 200 1024
            Regex::new(r#"([\d.]+)\s-\s-\s\[([^\]]*)\]\s\"([^\"]+)\"\s2\d\d\s(\d+)$"#).unwrap(),
            "http_success", //200
        ),
        (
            Regex::new(r#"([\d.]+)\s-\s-\s\[([^\]]*)\]\s\"([^\"]+)\"\s404\s(\d+)$"#).unwrap(),
            "http_not_found", //404
        ),
        (
            Regex::new(r#"([\d.]+)\s-\s-\s\[([^\]]*)\]\s\"([^\"]+)\"\s403\s(\d+)$"#).unwrap(),
            "http_forbidden", //403
        ),
        (
            Regex::new(r#"([\d.]+)\s-\s-\s\[([^\]]*)\]\s\"([^\"]+)\"\s5\d\d\s(\d+)$"#).unwrap(),
            "http_server_error", //500
        ),
    ];

    for line in contents.lines() { 
        for (pattern, event_type) in &patterns {
            if let Some(caps) = pattern.captures(line) {
                
                let timestamp = match caps.get(2) {
                    Some(m) => m.as_str().to_string(),
                    None => "".to_string(),
                };

                let user = None;

                let ip= match caps.get(1) {
                    Some(m) => Some(m.as_str().to_string()),
                    None => Some("".to_string()),
                };

                let command= match caps.get(3) {
                    Some(m) => Some(m.as_str().to_string()),
                    None => Some("".to_string()),
                };

                let size = match caps.get(4) {
                    Some(m) => m.as_str().parse::<i64>().ok(),
                    None => None,
                }; 
                
                let event = Event {
                    timestamp,
                    source: "nginx_access".to_string(),
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