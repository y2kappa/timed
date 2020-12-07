
use crate::hop::Hop;
use crate::chrome_trace;
use crate::statistics;

use thiserror::Error;
use std::collections::HashMap;

pub struct Trace {
    id: String,
    start_timestamp: u128,
}

impl Trace {

    fn register(id: &str) {
        crate::TRACES.lock().unwrap().insert(id.to_string(), vec![]);
    }

    pub fn record(hop: Hop) {
        for trace_group in crate::TRACES.lock().unwrap().iter_mut() {
            trace_group.1.push(hop.clone());
        }
    }

    pub fn new(id: &str) -> Self {
        let trace = Self {
            id: id.to_string(),
            start_timestamp: 0
        };

        Trace::register(id);
        trace
    }

    pub fn chrome_tracing(&self) -> String {
        let mut traces = crate::TRACES.lock().unwrap();
        let entries = traces.entry(self.id.clone()).or_insert(vec![]);
        chrome_trace::from(entries)
    }

    pub fn statistics(&self) -> String {
        let mut traces = crate::TRACES.lock().unwrap();
        let entries = traces.entry(self.id.clone()).or_insert(vec![]);
        statistics::from(entries)
    }
}