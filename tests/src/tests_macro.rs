fn main() {
    timed::TraceOptions::new()
        .with_chrome_trace(|x: &str| println!("{}", x))
        .with_statistics(|x: &str| println!("{}", x))
        .build_named("Hei");
}
