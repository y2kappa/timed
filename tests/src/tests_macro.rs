use timed::{ChromeTraceResult, StatisticsResult};

fn main() {
    let _ = timed::init_tracing(*timed::TraceOptions::new()
        .with_chrome_trace(|x: &ChromeTraceResult| println!("{}", x.to_string()))
        .with_statistics(|x: &StatisticsResult| println!("{:?}", x)));
}
