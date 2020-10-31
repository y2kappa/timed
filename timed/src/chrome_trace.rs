use std::time::Duration;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct ChromeTraceRecord {
    pub ts: u128,
    pub ph: Phase,
    pub name: String,
}

pub struct ChromeTraceResult {
    pub(crate) records: Vec<ChromeTraceRecord>
}

impl ChromeTraceResult {
    pub(crate) fn new() -> ChromeTraceResult {
        ChromeTraceResult {
            records: vec![]
        }
    }

    pub fn to_string(&self) -> String {
        let mut chrome_trace_string = "[\n".to_string();

        for (i, record) in self.records.iter().enumerate() {
            let is_last = i == self.records.len() - 1;
            let trace = format!(
                "{{ \"pid\": 0, \"ts\": {},  \"ph\": \"{}\", \"name\": \"{}\" }}",
                record.ts,
                record.ph.to_string(),
                record.name
            );
            chrome_trace_string.push_str(&format!(
                "    {}{}\n",
                trace,
                if !is_last { "," } else { "" }
            ));
        }

        chrome_trace_string.push_str("]");

        return chrome_trace_string;
    }
}