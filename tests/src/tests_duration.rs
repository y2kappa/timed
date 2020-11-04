// use std::{thread, time};
// use timed::timed;
//
// use std::sync::Once;
// #[allow(dead_code)]
// static INIT: Once = Once::new();
//
// /// Setup function that is only run once, even if called multiple times.
// #[allow(dead_code)]
// fn init_logger() {
//     INIT.call_once(|| {
//         pretty_env_logger::init();
//     });
// }
//
// #[timed]
// fn foo(x: i32, y: i32) -> i32 {
//     thread::sleep(time::Duration::from_millis(100));
//     x + y
// }
//
// #[timed]
// fn bar(x: i32) -> i32 {
//     thread::sleep(time::Duration::from_millis(100));
//     x
// }
//
// #[timed]
// fn baz() -> i32 {
//     42
// }
//
// #[timed]
// fn foobar() {
//     thread::sleep(time::Duration::from_millis(100));
// }
//
// #[timed]
// fn add(x: i32, y: i32) -> i32 {
//     x + y
// }
//
// #[timed]
// fn mul(x: i32, y: i32) -> i32 {
//     x * y
// }
//
// #[timed(printer = "println!")]
// fn mul_println(x: i32, y: i32) -> i32 {
//     x * y
// }
//
// #[timed(printer = "info!")]
// fn mul_info(x: i32, y: i32) -> i32 {
//     x * y
// }
//
// #[timed(printer = "warn!")]
// fn mul_error(x: i32, y: i32) -> i32 {
//     x * y
// }
//
// #[timed]
// async fn ping_google() {
//     reqwest::get("https://google.com").await.unwrap();
// }
//
// #[test]
// fn simple() {
//     foo(1, 2);
//     bar(1);
//     baz();
//     foobar();
//
//     assert_eq!(add(1, 2), 3);
//     assert_eq!(mul(1, 2), 2);
// }
//
// #[test]
// fn with_logging() {
//     init_logger();
//
//     mul_info(1, 1);
//     mul_info(1, 1);
//     mul_error(1, 1);
// }
//
// #[tokio::test]
// async fn with_async() {
//     ping_google().await
// }
//
// #[tokio::test]
// async fn test_async_quotes() {
//     use crate::tests_utils::get_random_quote;
//     get_random_quote().await
// }
