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

impl TraceRecord {
    pub fn to_chrome_trace(&self) -> String {
        format!(
            "{{ \"pid\": 0, \"ts\": {},  \"ph\": \"{}\", \"name\": \"{}\" }}",
            self.timestamp,
            self.phase.to_string(),
            self.function_name
        )
    }
}

pub struct ChromeTraceResult {
    pub(crate) records: Vec<TraceRecord>
}

impl ChromeTraceResult {
    pub(crate) fn new() -> ChromeTraceResult {
        ChromeTraceResult {
            records: vec![]
        }
    }

    pub fn to_chrome_trace(&self) -> String {
        let mut chrome_trace_string = "[\n".to_string();

        for (i, record) in self.records.iter().enumerate() {
            let is_last = i == self.records.len() - 1;
            chrome_trace_string.push_str(&format!(
                "    {}{}\n",
                record.to_chrome_trace(),
                if !is_last { "," } else { "" }
            ));
        }

        chrome_trace_string.push_str("]");

        return chrome_trace_string;
    }
}