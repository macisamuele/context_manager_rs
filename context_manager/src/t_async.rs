use std::future::Future;

use crate::CallerContext;
#[cfg(doc)] // Imports needed only for doc purposes
use crate::{wrap, SyncWrapContext};

/// Context Manager definition (async hooks)
///
/// The defined context, is suitable for initialisation, before and after the execution that requires the execution of asynchronous code.
/// If you have context context initialisation or before/after hooks all being syncrhonous, then please consider using [`SyncWrapContext`] instead.
///
/// **IMPORTANT**: [`AsyncWrapContext`] does not support running synchronous blocks.
/// This is intentional in order to avoid possibly stalling the async-runtime in use.
/// Please consider wrapping yourself the synchronous code in an async block, or using [`SyncWrapContext`] whether possible.
///
/// Implementers are then expected to be used via the [`wrap`] macro
/// ```
/// # use context_manager::{async_wrap, AsyncWrapContext};
/// struct AsyncPrintDuration;
/// impl<T> AsyncWrapContext<T> for AsyncPrintDuration {
///    async fn new() -> Self { Self }
/// }
///
/// #[async_wrap(AsyncPrintDuration)]
/// async fn async_foo() -> usize {
///     # let do_something_expensive = || async { 1234 };
///     do_something_expensive().await
/// }
/// ```
///
/// or via the [`AsyncWrapContext::run`] associated function.
/// ```
/// # use context_manager::{AsyncWrapContext, CallerContext};
/// struct AsyncPrintDuration;
/// impl<T> AsyncWrapContext<T> for AsyncPrintDuration {
///   async fn new() -> Self { Self }
/// }
///
/// # async fn foo() {
/// let async_run_output: &'static str = AsyncPrintDuration::run(CallerContext::new("manual"), async {
///     "async"
/// }).await;
/// # }
/// ```
pub trait AsyncWrapContext<T> {
    /// Initialize the context
    #[allow(async_fn_in_trait)]
    async fn new() -> Self
    where
        Self: Sized;

    /// Execute the code before the execution of the wrapped body
    ///
    /// Parameters:
    /// - `caller_context`: Context of the caller (including the name of the function that is being wrapped)
    #[allow(async_fn_in_trait, unused_variables, clippy::unused_async)]
    async fn before(&self, caller_context: &CallerContext) {}

    /// Execute the code after the execution of the wrapped body, it provides also the result of the wrapped body
    ///
    /// Parameters:
    /// - `caller_context`: Context of the caller (including the name of the function that is being wrapped)
    /// - `result`: The result of the wrapped body
    #[allow(async_fn_in_trait, unused_variables, clippy::unused_async)]
    async fn after(self, caller_context: &CallerContext, result: &T)
    where
        Self: Sized,
    {
    }

    /// Execute a asynchronous block of code wrapped by the context
    ///
    /// This will lead to context initialisation and execution of before/after hooks
    ///
    /// Parameters:
    /// - `caller_context`: Context of the caller (including the name of the function that is being wrapped)
    /// - `block`: the future to wrap and execute
    ///
    /// Usage example:
    /// ```
    /// # use context_manager::{AsyncWrapContext, CallerContext};
    /// struct PrintDuration;
    /// impl<T> AsyncWrapContext<T> for PrintDuration {
    ///   async fn new() -> Self { Self }
    /// }
    ///
    /// # async fn foo() {
    /// let async_run_output: &'static str = PrintDuration::run(CallerContext::new("manual"), async {
    ///     "async"
    /// }).await;
    /// # }
    /// ```
    #[allow(async_fn_in_trait)]
    async fn run(caller_context: CallerContext, block: impl Future<Output = T>) -> T
    where
        Self: Sized,
    {
        let context = Self::new().await;
        context.before(&caller_context).await;
        let result = block.await;
        context.after(&caller_context, &result).await;
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::CallerContext;

    use super::AsyncWrapContext;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    #[tokio::test]
    async fn wrapper_usage_on_async_function() {
        static VALUE: AtomicUsize = AtomicUsize::new(100);

        struct Async;
        impl AsyncWrapContext<usize> for Async {
            async fn new() -> Self {
                Self
            }

            async fn before(&self, _: &CallerContext) {
                // Reset the value to 0
                VALUE.store(0, Ordering::Relaxed);
                // Which will be verified in the function execution
            }

            async fn after(self, _: &CallerContext, result: &usize) {
                VALUE.store(2 * *result, Ordering::Relaxed);
            }
        }

        assert_eq!(
            Async::run(CallerContext::new("test"), async {
                assert_eq!(VALUE.load(Ordering::Relaxed), 0);
                42
            },)
            .await,
            42
        );

        // The return value is doubled in the after hook
        assert_eq!(VALUE.load(Ordering::Relaxed), 84);
    }
}
