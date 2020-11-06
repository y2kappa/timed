use crate::{RecordBuffer, TraceRecord};

pub trait ChromeTraceExt {
    fn get_chrome_trace(&mut self) -> ChromeTraceResult;
}

pub trait ChromeTraceRecordExt {
    fn to_chrome_trace(&self) -> String;
}

impl ChromeTraceRecordExt for TraceRecord {
    fn to_chrome_trace(&self) -> String {
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
    pub(crate) fn new(data: &Vec<TraceRecord>) -> ChromeTraceResult {
        ChromeTraceResult {
            records: data.clone()
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

impl ChromeTraceExt for RecordBuffer {
    fn get_chrome_trace(&mut self) -> ChromeTraceResult {
        ChromeTraceResult::new(&self.drain())
    }
}