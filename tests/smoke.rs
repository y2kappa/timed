use std::{thread, time};

#[timed::timed]
fn foo(x: i32, y :i32) -> i32 {
    thread::sleep(time::Duration::from_millis(100));
    x + y
}

#[timed::timed]
fn bar(x: i32) -> i32 {
    thread::sleep(time::Duration::from_millis(100));
    x
}

#[timed::timed]
fn baz() -> i32 {
    // thread::sleep(time::Duration::from_millis(100));
    42
}

#[timed::timed]
fn foobar() {
    thread::sleep(time::Duration::from_millis(100));
}

#[timed::timed]
fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[timed::timed]
fn mul(x: i32, y: i32) -> i32 {
    x * y
}

#[test]
fn works() {
    foo(1, 2);
    bar(1);
    baz();
    foobar();

    assert_eq!(add(1, 2), 3);
    assert_eq!(mul(1, 2), 2);
}