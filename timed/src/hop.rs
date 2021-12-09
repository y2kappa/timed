use std::time::Duration;

#[derive(Clone, Debug)]
pub enum Phase {
    Start,
    Finish(Duration),
}

impl std::fmt::Display for Phase {
    // These are B and E for chrome tracing
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Phase::Start => "B".to_string(),
                Phase::Finish(_) => "E".to_string(),
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct Hop {
    pub function_name: String,
    pub timestamp: u128,
    pub phase: Phase,
}

impl Hop {
    pub fn to_chrome_trace(&self) -> String {
        format!(
            "{{ \"pid\": 0, \"ts\": {},  \"ph\": \"{}\", \"name\": \"{}\" }}",
            self.timestamp,
            self.phase,
            self.function_name
        )
    }
}
