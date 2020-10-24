fn main() {
    timed::init_tracing!("Hei", timed::TraceOptions::new()
    .with_chrome_trace(
        |x: &str| println!("{}", x)
    ).build());
}
