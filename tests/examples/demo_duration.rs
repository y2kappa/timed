use timed::timed;
#[macro_use]
extern crate log;

#[timed]
fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[timed]
fn mul(x: i32, y: i32) -> i32 {
    x * y
}

#[timed(printer = "println!")]
fn mul_println(x: i32, y: i32) -> i32 {
    x * y
}

#[timed(printer = "info!")]
fn mul_info(x: i32, y: i32) -> i32 {
    x * y
}

#[test]
fn timing() {
    pretty_env_logger::init();

    assert_eq!(add(1, 2), 3);
    assert_eq!(mul(1, 2), 2);
    assert_eq!(mul_println(1, 2), 2);
    assert_eq!(mul_info(1, 2), 2);
}
