use std::time::Duration;

#[derive(Clone, Debug)]
pub enum Phase {
    Start,
    Finish(Duration),
}

impl Phase {
    // These are B and E for chrome tracing
    pub(crate) fn to_string(&self) -> String {
        match self {
            Phase::Start => "B".to_string(),
            Phase::Finish(_) => "E".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TraceRecord {
    pub function_name: String,
    pub timestamp: u128,
    pub phase: Phase,
}