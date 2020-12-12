use log::info;
use timed::timed;

#[timed(duration(printer = "info!"))]
fn main() {
    pretty_env_logger::init();
    println!("Running main");
}
