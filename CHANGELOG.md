Changelog
=========

0.1.3 (2025-01-28)
------------------

* Update [`CallerContext`] APIs to be const and [`CallerContext::fn_name`] returns `&'static str`

0.1.2 (2025-01-21)
------------------

* Define [`CallerContext`] to provide [`AsyncWrapContext`] and [`SyncWrapContext`] the ability to have more details on the wrapped function
* Improve documentation of [`context_manager_macro`] crate to avoid misleading content on [crates.io] ([before](https://crates.io/crates/context_manager_macro/0.1.1), [after](https://crates.io/crates/context_manager_macro/0.1.2))

0.1.1 (2025-01-21)
------------------

* Minor update of package documentation (no code changes)

0.1.0 (2025-01-21)
------------------

**Initial release**

The library provides a simple access point to `#[wrap(_)]` and `#[async_wrap(_)]` macros to enable end-users to wrap a function with custom logic.

<!-- Links -->
[`AsyncWrapContext`]: https://docs.rs/context_manager/latest/context_manager/trait.AsncWrapContext.html
[`CallerContext::fn_name`]: https://docs.rs/context_manager/latest/context_manager/struct.CallerContext.html#method.fn_name
[`CallerContext`]: https://docs.rs/context_manager/latest/context_manager/struct.CallerContext.html
[`context_manager_macro`]: https://docs.rs/context_manager_macro
[`context_manager`]: https://docs.rs/context_manager
[`SyncWrapContext`]: https://docs.rs/context_manager/latest/context_manager/trait.SyncWrapContext.html
[crates.io]: https://crates.io
