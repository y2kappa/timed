## Timeit

```rust
extern crate timeit;

#[timeit::timeit]
fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[timeit::timeit]
fn mul(x: i32, y: i32) -> i32 {
    x * y
}


#[test]
fn timing() {

    assert_eq!(add(1, 2), 3);

    assert_eq!(mul(1, 2), 2);
}
```

```
$ cargo test -- --nocapture
running 1 test

function=add duration=114ns
function=mul duration=97ns


test timing ... ok
```

foobar, foo, bar, baz, qux, quux, quuz, corge, grault, garply, waldo, fred, plugh, xyzzy, and thud, Wibble, wobble, wubble, and flob

## TODO:
- [ ] attribute to macro such as log level if needed
- [ ] formatting ``fn=name dur=25sec``

## References
- https://github.com/dtolnay/quote
- https://github.com/alexcrichton/proc-macro2#usage

