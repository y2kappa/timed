use crate::chrome_trace;
use crate::hop::{Hop, Phase};
use crate::statistics;

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
        let start_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros();

        let trace = Self {
            id: id.to_string(),
            start_timestamp,
        };

        Trace::register(id);
        Trace::record(Hop {
            function_name: id.to_string(),
            timestamp: start_timestamp,
            phase: Phase::Start,
        });

        // record the original too
        trace
    }

    fn generate_current_end_hop(&self) -> Hop {
        let end_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros();

        let duration =
            std::time::Duration::from_micros((end_timestamp - self.start_timestamp) as u64);

        Hop {
            function_name: self.id.to_string(),
            timestamp: end_timestamp,
            phase: Phase::Finish(duration),
        }
    }

    pub fn chrome_tracing(&self) -> String {
        let mut traces = crate::TRACES.lock().unwrap();
        let entries = traces.entry(self.id.clone()).or_insert_with(Vec::new);

        entries.push(self.generate_current_end_hop());

        chrome_trace::from(entries)
    }

    pub fn statistics(&self) -> String {
        let mut traces = crate::TRACES.lock().unwrap();
        let entries = traces.entry(self.id.clone()).or_insert_with(Vec::new);

        entries.push(self.generate_current_end_hop());

        statistics::from(entries)
    }
}
