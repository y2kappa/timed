# Ideas / Plans / Wishlist
- [ ] formatting ``fn=name dur=25sec``
    - allow for custom formatting
- [ ] add possibility to happen only during debug mode, not release
- [ ] allow hooks `|name, duration| {}` to be inserted
- [ ] idea for a new crate: can have a global state (for testing maybe) that tracks / tests / counts how many times something was called and assert during tests

## Chrome tracing
- [ ] Keep accumulating in a global state
- [ ] use timed::dump/flush on stdout/file when useful
- [ ] add documentation show how to use chrome://tracing
- [ ] add CI
- [ ] profile this, with and without to see perf impact, async and multithreading
- [ ] add save to file
- [ ] show stats to give you some sort of state of what should you really optimize for in your function, to keep accumulating the seconds for that given tag / function
- [ ] blog post about tracing

## Done
- [x] Add and test async
- [x] Use syn, remove manual parser
- [x] Attribute to macro such as log level if needed:
    - macro/function to call when timing such as "info", "println", "lambda"
- [x] inspect https://github.com/gustavla/timeit/pulls see how to combine
- [x] add timed macro for blocks
    - timeit does it many times (looks like already done)
    - this is just for profiling
- [x] write another wrapper for info! that dumps to chrome::tracing as well - will combine with tracing
- [x] add possibility to print start, end
    - [x] eventually generate flamegraph out of it
    - [x] or (vertical) timegraph with overlaps
    - [x] use chrome::tracing (need to generate a new file or print to stdout)