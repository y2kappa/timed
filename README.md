# Rust `timed` macro to quickly profile your programs

✅ Works with `async`

✅ Works with `main`

✅ Custom printers, like `println!`, `info!`, or your own function.

✅ Profile your program with chrome tracing, build a flamegraph.

✅ New! Show timing statistics between your functions

### Usage `duration`

```toml
[dependencies]
timed = "0.1.5"
log = "0.4"
```

```rust
use timed::timed;
#[macro_use] extern crate log;

#[timed]
fn add(x: i32, y: i32) -> i32 { x + y }

#[timed]
fn mul(x: i32, y: i32) -> i32 { x * y }

#[timed(duration(printer = "println!"))]
fn mul_println(x: i32, y: i32) -> i32 { x * y}

#[timed(duration(printer = "info!"))]
fn mul_info(x: i32, y: i32) -> i32 { x * y }


#[test]
fn timing() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(mul(1, 2), 2);
    assert_eq!(mul_println(1, 2), 2);
    assert_eq!(mul_info(1, 2), 2);
}
```


Output:

```
$ cargo test -- --nocapture
running 1 test

function=add duration=36ns
function=mul duration=36ns
function=mul_println duration=31ns
 INFO  demo_duration > function=mul_info duration=326ns

test timing ... ok
```

Also works with main and tokio:

```rust
#[tokio::main]
#[timed]
async fn main() {
    println!("Running main");
    reqwest::get("https://google.com").await;
}
```

Output:

```
Running main
Calling https://type.fit/api/quotes
Quote of the day:
Genius is one percent inspiration and ninety-nine percent perspiration. - Thomas Edison
function=get_random_quote duration=455.291753ms
function=main duration=456.452412ms
```

### Usage chrome::tracing

```rust
#[timed::timed(tracing(enabled = true), duration(disabled = true))]
fn foo() {
    bar();
    baz();
}

#[timed::timed(tracing(enabled = true), duration(disabled = true))]
fn baz() {
    println!("Hello")
}

#[timed::timed(tracing(enabled = true), duration(disabled = true))]
fn bar() {
    baz();
}

#[timed::timed(tracing(enabled = true), duration(disabled = true))]
fn main() {
    let trace = timed::TraceOptions::new()
        .with_chrome_trace(|x: &str| println!("{}", x))
        .build_named("Main");

    foo();

    trace.finish();
}

```

```shell script
Hello
Hello
[
    { "pid": 0, "ts": 1603026625248670,  "ph": "B", "name": "foo" },
    { "pid": 0, "ts": 1603026625248676,  "ph": "B", "name": "bar" },
    { "pid": 0, "ts": 1603026625248677,  "ph": "B", "name": "baz" },
    { "pid": 0, "ts": 1603026625248721,  "ph": "E", "name": "baz" },
    { "pid": 0, "ts": 1603026625248725,  "ph": "E", "name": "bar" },
    { "pid": 0, "ts": 1603026625248727,  "ph": "B", "name": "baz" },
    { "pid": 0, "ts": 1603026625248732,  "ph": "E", "name": "baz" },
    { "pid": 0, "ts": 1603026625248735,  "ph": "E", "name": "foo" }
]

```

Save the json dump between `[` and `]` to file `tracing.json` then open in Chrome `chrome://tracing` and drag the file:
<img src="docs/tracing_demo.png" />

### Usage statistics

```rust
#[timed::timed(tracing(enabled = true), duration(disabled = true))]
fn main() {
    let trace = timed::Trace::new("Main");

    foo();
    std::thread::sleep(std::time::Duration::from_millis(10));

    println!("{}", trace.statistics());
}
```

```shell script
+----------------------------------------+-------+--------------+-----------+-----------+-----------+-----------+-----------+
| function name                          | calls | overall time | avg time  | max time  | p90 time  | p50 time  | p10 time  |
+----------------------------------------+-------+--------------+-----------+-----------+-----------+-----------+-----------+
| Main                                   | 1     | 10.955ms     | 10.955ms  | 10.955ms  | 10.955ms  | 10.955ms  | 10.955ms  |
+----------------------------------------+-------+--------------+-----------+-----------+-----------+-----------+-----------+
| demo_statistics::foo                   | 1     | 112.184µs    | 112.184µs | 112.184µs | 112.184µs | 112.184µs | 112.184µs |
+----------------------------------------+-------+--------------+-----------+-----------+-----------+-----------+-----------+
| demo_statistics::bar                   | 1     | 99.452µs     | 99.452µs  | 99.452µs  | 99.452µs  | 99.452µs  | 99.452µs  |
+----------------------------------------+-------+--------------+-----------+-----------+-----------+-----------+-----------+
| demo_statistics::baz                   | 11    | 85.069µs     | 7.733µs   | 8.403µs   | 5.738µs   | 5.895µs   | 19.525µs  |
+----------------------------------------+-------+--------------+-----------+-----------+-----------+-----------+-----------+
| demo_statistics::one::two::three::deep | 33    | 691ns        | 20ns      | 25ns      | 23ns      | 17ns      | 23ns      |
+----------------------------------------+-------+--------------+-----------+-----------+-----------+-----------+-----------+
```

## Contribution
Contributions are welcome. Please submit PR.
See [TODO](TODO.md)

### Run / Build

To build specific packages / examples / binaries, follow the usual workflow. To do a full build on everything, run the below:

```shell script
$ cd tests && cargo make all
$ cd timed && cargo make all
$ cd timed_proc_macros && cargo make all
```