use std::{thread, time};

#[timeit::timeit]
fn foo(x: i32, y :i32) -> i32 {
    thread::sleep(time::Duration::from_millis(100));
    x + y
}

#[timeit::timeit]
fn bar(x: i32) -> i32 {
    thread::sleep(time::Duration::from_millis(100));
    x
}

#[timeit::timeit]
fn baz() -> i32 {
    // thread::sleep(time::Duration::from_millis(100));
    42
}

#[timeit::timeit]
fn foobar() {
    thread::sleep(time::Duration::from_millis(100));
}

#[test]
fn works() {
    foo(1, 2);
    bar(1);
    baz();
    foobar();
}