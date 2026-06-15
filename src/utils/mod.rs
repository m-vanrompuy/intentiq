use chrono::NaiveDateTime;

pub fn parse_timestamp(ts: &str) -> Option<NaiveDateTime> {
    // auth.log/syslog formaat: "May  6 09:16:01" → voeg jaar toe
    let ts_with_year = format!("{} 2026", ts);
    NaiveDateTime::parse_from_str(&ts_with_year, "%b %_d %H:%M:%S %Y")
        .ok()
        .or_else(|| NaiveDateTime::parse_from_str(ts, "%d/%b/%Y:%H:%M:%S %z").ok())
}