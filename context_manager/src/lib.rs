#![deny(
    dead_code,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    rustdoc::all,
    rustdoc::missing_crate_level_docs,
    unreachable_pub,
    unused_imports,
    unused_variables
)]
// Use README.md file as module documentation
// This makes easy to have a proper home for the github project as well as
// ensuring that the content is always up to date and tested
#![doc = include_str!("../README.md")]
//!
#![doc = include_str!("../CHANGELOG.md")]

mod t_async;
mod t_sync;
pub use crate::t_async::AsyncWrapContext;
pub use crate::t_sync::SyncWrapContext;

/// Context about the caller propagated into the context.
#[derive(Debug)]
#[non_exhaustive]
pub struct CallerContext {
    /// Name of the wrapped function
    fn_name: &'static str,
}

impl CallerContext {
    /// Create a new instance of the `CallerContext`
    #[must_use]
    pub fn new(fn_name: &'static str) -> Self {
        Self { fn_name }
    }

    /// Name of the wrapped function
    #[must_use]
    pub fn fn_name(&self) -> &str {
        self.fn_name
    }
}

/// Procedural macro that will decorate the incoming async function with the provided context.
///
/// The context is expected to be a type that implements the `AsyncWrapContext` trait.
///
/// Usage example:
/// ```
/// # use context_manager_macro::async_wrap;
/// struct AsyncPrintDuration;
/// impl<T> context_manager::AsyncWrapContext<T> for AsyncPrintDuration {
///   async fn new() -> Self { Self }
/// }
///
/// #[async_wrap(AsyncPrintDuration)]
/// async fn foo<'a, T>(int_value: usize, str_ref: &'a str, generic: T) -> usize {
///     let type_name = std::any::type_name::<T>();
///     println!("Async call with int_value={int_value}, str_ref={str_ref}, type_of(T)={type_name}");
///     10
/// }
/// ```
///
/// The decorator does not induce limits on the shape of the incoming function, in terms
/// of generics, sync/async, lifetime, etc.
///
/// The decorator will expand the incoming function by adding the context handling
/// rendering something similar to
/// ```
/// # use context_manager::{AsyncWrapContext, CallerContext};
/// # struct AsyncPrintDuration;
/// # impl<T> AsyncWrapContext<T> for AsyncPrintDuration {
/// #   async fn new() -> Self { Self }
/// # }
/// async fn foo<'a, T>(int_value: usize, str_ref: &'a str, generic: T) -> usize {
///     AsyncPrintDuration::run(CallerContext { fn_name: "foo" }, async {
///         let type_name = std::any::type_name::<T>();
///         println!("Async call with int_value={int_value}, str_ref={str_ref}, type_of(T)={type_name}");
///         10
///     }).await
/// }
/// ```
///
/// The structuring of the generated code is though to avoid any clone/copy of data,
/// as well as reducing the number of jumps needed to execute the original code.
///
/// # Possible compile errors
/// ## Passing a type that does not implement `AsyncWrapContext` trait will lead to compile errors.
/// ```compile_fail
/// # use context_manager_macro::async_wrap;
/// struct PrintDuration;
/// impl<T> context_manager::SyncWrapContext<T> for PrintDuration {
///   fn new() -> Self { Self }
/// }
///
/// #[async_wrap(PrintDuration)]
/// async fn foo() {}
/// ```
/// would lead to the following compile error
/// ```text
/// error[E0277]: the trait bound `PrintDuration: AsyncWrapContext<_>` is not satisfied
///   --> src/lib.rs:206:1
///    |
/// 11 | #[async_wrap(PrintDuration)]
///    |                  ^^^^^^^^^^^^^ the trait `AsyncWrapContext<_>` is not implemented for `PrintDuration`
///    |
///    = help: the trait `AsyncWrapContext<T>` is implemented for `AsyncTraceDuration`
/// ```
///
/// ## Decorating a synchronous function
/// This is intentional in order to avoid possibly stalling the async-runtime in use.
/// Please consider wrapping yourself the synchronous code in an async block, or using `#[decorate]` whether possible.
/// ```compile_fail
/// # use context_manager_macro::async_wrap;
/// struct AsyncPrintDuration;
/// impl<T> context_manager::AsyncWrapContext<T> for AsyncPrintDuration {
///   async fn new() -> Self { Self }
/// }
///
/// #[async_wrap(PrintDuration)]
/// fn foo() {}
/// ```
/// would lead to the following error
/// ```text
/// error: #[async_wrap] cannot operate on sync functions. Please consider using a #[decorate] macro or converting/wrapping the function to be async.
///   --> src/lib.rs:228:1
///    |
/// 11 | #[async_wrap(PrintDuration)]
///    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
///    |
/// ```
pub use context_manager_macro::async_wrap;

/// Procedural macro that will decorate the incoming function with the provided context.
///
/// The context is expected to be a type that implements the `SyncWrapContext` trait.
///
/// Usage example:
/// ```
/// # use context_manager_macro::wrap;
/// struct PrintDuration;
/// impl<T> context_manager::SyncWrapContext<T> for PrintDuration {
///   fn new() -> Self { Self }
/// }
///
/// #[wrap(PrintDuration)]
/// fn sync_foo<'a, T>(int_value: usize, str_ref: &'a str, generic: T) -> usize {
///     let type_name = std::any::type_name::<T>();
///     println!("Sync call with int_value={int_value}, str_ref={str_ref}, type_of(T)={type_name}");
///     10
/// }
///
/// #[wrap(PrintDuration)]
/// async fn async_foo<'a, T>(int_value: usize, str_ref: &'a str, generic: T) -> usize {
///     let type_name = std::any::type_name::<T>();
///     println!("Async call with int_value={int_value}, str_ref={str_ref}, type_of(T)={type_name}");
///     10
/// }
/// ```
///
/// The decorator does not induce limits on the shape of the incoming function, in terms
/// of generics, sync/async, lifetime, etc.
///
/// The decorator will expand the incoming function by adding the context handling
/// rendering something similar to
/// ```
/// # use context_manager::{CallerContext, SyncWrapContext};
/// # struct PrintDuration;
/// # impl<T> SyncWrapContext<T> for PrintDuration {
/// #   fn new() -> Self { Self }
/// # }
/// fn sync_foo<'a, T>(int_value: usize, str_ref: &'a str, generic: T) -> usize {
///     PrintDuration::run_sync(CallerContext { fn_name: "sync_foo" }, move || {
///         let type_name = std::any::type_name::<T>();
///         println!("Sync call with int_value={int_value}, str_ref={str_ref}, type_of(T)={type_name}");
///         10
///     })
/// }
///
/// async fn async_foo<'a, T>(int_value: usize, str_ref: &'a str, generic: T) -> usize {
///     PrintDuration::run_async(CallerContext { fn_name: "async_foo" }, async {
///         let type_name = std::any::type_name::<T>();
///         println!("Async call with int_value={int_value}, str_ref={str_ref}, type_of(T)={type_name}");
///         10
///     }).await
/// }
/// ```
///
/// The structuring of the generated code is though to avoid any clone/copy of data,
/// as well as reducing the number of jumps needed to execute the original code.
///
/// # Possible compile errors
/// ## Passing a type that does not implement `SyncWrapContext` trait will lead to compile errors.
/// ```compile_fail
/// # use context_manager_macro::wrap;
/// struct AsyncPrintDuration;
/// impl<T> context_manager::AsyncWrapContext<T> for AsyncPrintDuration {
///   async fn new() -> Self { Self }
/// }
///
/// #[wrap(AsyncPrintDuration)]
/// fn foo() {}
/// ```
/// would lead to the following compile error
/// ```text
/// ---- src/lib.rs - decorate (line 98) stdout ----
/// error[E0277]: the trait bound `AsyncPrintDuration: SyncWrapContext<_>` is not satisfied
///   --> src/lib.rs:106:12
///    |
/// 11 | #[wrap(AsyncPrintDuration)]
///    |            ^^^^^^^^^^^^^^^^^^ the trait `SyncWrapContext<_>` is not implemented for `AsyncPrintDuration`
///    |
///    = help: the trait `SyncWrapContext<T>` is implemented for `TraceDuration`
/// ```
///
/// ## Decorating a constant function
/// Const functions are not supported for decoration.
/// This is a side-effect of embedding code that is not const compatible (like async blocks and closures).
///
/// ```compile_fail
/// # use context_manager_macro::wrap;
/// struct PrintDuration;
/// impl<T> context_manager::SyncWrapContext<T> for PrintDuration {
///   fn new() -> Self { Self }
/// }
///
/// #[wrap(PrintDuration)]
/// const fn foo() {}
/// ```
/// would lead to the following error
/// ```text
/// error: #[wrap] cannot operate on const functions
///   --> context_manager_macro/src/lib.rs:131:1
///    |
/// 11 | #[wrap(PrintDuration)]
///    | ^^^^^^^^^^^^^^^^^^^^^^
///    |
/// ```
pub use context_manager_macro::wrap;

#[cfg(test)]
mod tests {
    use trybuild::TestCases;

    #[test]
    fn procedural_macros_ui_tests() {
        let t = TestCases::new();
        t.pass("tests/ui/pass/*.rs");
        t.compile_fail("tests/ui/fail/*.rs");
    }
}
