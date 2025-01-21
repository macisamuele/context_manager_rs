# Context Manager

[![Build and test](https://github.com/macisamuele/context_manager_rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/macisamuele/context_manager_rs/actions/workflows/ci.yaml) [![crates.io](https://img.shields.io/crates/v/context_manager.svg)](https://crates.io/crates/context_manager) [![docs.rs](https://img.shields.io/docsrs/context_manager)](https://docs.rs/context_manager)

Library offering an easy access Python like decorators in rust

Via this library we can easily alter the content of a defined function, by injecting code to be executed
before/after the original function, without altering the original function itself.

For example, let's assume that we are interested on logging the duration of the execution of a function.

```no_run
fn function_to_be_decorated() -> usize {
    # let do_something_expensive = || 1234;
    do_something_expensive()
}
```

In order to achieve so we would need to manually modify it similarly to

```no_run
fn function_to_be_decorated() -> usize {
    # let do_something_expensive = || 1234;
    let start = std::time::Instant::now();
    let return_value = do_something_expensive();
    println!("Duration of function_to_be_decorated is: {}ms", start.elapsed().as_millis());
    return_value
}
```

As far as this could be very simple, it is tedious and requires to actually modify the original function.
While having a simple decorator that would do it for us would be much more convenient.
Leading to something like

```rust
# use context_manager::SyncWrapContext;
# struct PrintDuration;
# impl<T> SyncWrapContext<T> for PrintDuration { fn new() -> Self { Self } }
#[context_manager::wrap(PrintDuration)]
fn function_to_be_wrapped() -> usize {
    # let do_something_expensive = || 1234;
    do_something_expensive()
}
```

## How does the library work internally?

The library exposes 2 main traits that allows users to customize the wrapping logic

* [`SyncWrapContext`] for synchronous context management (which is supported for decorating sync and async functions)
* [`AsyncWrapContext`] for asynchronous context management (which is supported *ONLY* for async functions)

and 2 main attribute macros ([`wrap`] and [`async_wrap`]) that allow an easy plug-and-play of the logic into the code

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
