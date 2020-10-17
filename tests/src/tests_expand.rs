// #[timed::timed(tracing=true)]
fn foo() {
    std::thread::sleep(std::time::Duration::from_micros(1000));
}

fn main() {
    foo();
}