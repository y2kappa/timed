use crate::{Trace, ChromeTraceResult, StatisticsResult, ChromeTraceRecord};
use std::sync::Mutex;

#[derive(Copy, Clone, Default)]
pub struct TraceOptions {
    pub statistics: Option<fn(&StatisticsResult)>,
    pub chrome_trace: Option<fn(&ChromeTraceResult)>,
}

impl TraceOptions {
    pub fn new() -> TraceOptions {
        TraceOptions::default()
    }

    pub fn with_statistics(&mut self, f: fn(&StatisticsResult)) -> &mut TraceOptions {
        self.statistics = Some(f);
        self
    }

    pub fn with_chrome_trace(&mut self, f: fn(&ChromeTraceResult)) -> &mut TraceOptions {
        self.chrome_trace = Some(f);
        self
    }
}