use std::sync::{Arc, Mutex};
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

pub struct RecordBuffer {
    data: Vec<TraceRecord>
}

impl RecordBuffer {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(RecordBuffer { data: vec![] }))
    }

    pub fn add(&mut self, record: TraceRecord) {
        self.data.push(record);
    }

    pub fn drain(&mut self) -> Vec<TraceRecord> {
        self.data.drain(..).collect()
    }
}

pub struct TraceCollectorChain {
    pub buffers: Vec<Arc<Mutex<RecordBuffer>>>,
}

impl TraceCollectorChain {
    pub fn new() -> TraceCollectorChain {
        TraceCollectorChain { buffers: vec![] }
    }

    pub fn chain_output(&mut self, buffer: Arc<Mutex<RecordBuffer>>) -> &mut TraceCollectorChain {
        self.buffers.push(buffer);
        self
    }
}