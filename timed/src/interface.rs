use std::sync::{Arc, Mutex};

use crate::TraceRecord;

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