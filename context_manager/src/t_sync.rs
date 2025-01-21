use std::future::Future;

#[cfg(doc)] // Imports needed only for doc purposes
use crate::{wrap, AsyncWrapContext};

/// Context Manager definition (sync hooks)
///
/// The defined context, is suitable for initialisation, before and after the execution that requires the execution of synchronous code.
/// If you need to have context initialisation or before/after hooks to be asynchronous, please consider using [`AsyncWrapContext`] instead.
///
/// Implementers are then expected to be used via the [`wrap`] macro
/// ```
/// # use context_manager::{wrap, SyncWrapContext};
/// struct PrintDuration;
/// impl<T> SyncWrapContext<T> for PrintDuration {
///    fn new() -> Self { Self }
/// }
///
/// #[wrap(PrintDuration)]
/// fn sync_foo() -> usize {
///     # let do_something_expensive = || 1234;
///     do_something_expensive()
/// }
///
/// #[wrap(PrintDuration)]
/// async fn async_foo() -> usize {
///     # let do_something_expensive = || async { 1234 };
///     do_something_expensive().await
/// }
/// ```
///
/// or via the [`SyncWrapContext::run_sync`] and [`SyncWrapContext::run_async`] associated functions.
/// ```
/// # use context_manager::SyncWrapContext;
/// struct PrintDuration;
/// impl<T> SyncWrapContext<T> for PrintDuration {
///   fn new() -> Self { Self }
/// }
///
/// # async fn foo() {
/// let sync_run_output: &'static str = PrintDuration::run_sync(|| {
///     "sync"
/// });
/// let async_run_output: &'static str = PrintDuration::run_async(async {
///     "async"
/// }).await;
/// # }
/// ```
///
pub trait SyncWrapContext<T> {
    /// Initialize the context
    fn new() -> Self
    where
        Self: Sized;

    /// Execute the code before the execution of the wrapped body
    fn before(&self) {}

    /// Execute the code after the execution of the wrapped body, it provides also the result of the wrapped body
    #[allow(unused_variables)]
    fn after(self, result: &T)
    where
        Self: Sized,
    {
    }

    /// Execute a synchronous block of code wrapped by the context
    ///
    /// This will lead to context initialisation and execution of before/after hooks
    /// Usage example:
    /// ```
    /// # use context_manager::SyncWrapContext;
    /// struct PrintDuration;
    /// impl<T> SyncWrapContext<T> for PrintDuration {
    ///   fn new() -> Self { Self }
    /// }
    ///
    /// # async fn foo() {
    /// let async_run_output: &'static str = PrintDuration::run_sync(|| {
    ///     "sync"
    /// });
    /// # }
    /// ```
    fn run_sync(block: impl FnOnce() -> T) -> T
    where
        Self: Sized,
    {
        let context = Self::new();
        context.before();
        let result = block();
        context.after(&result);
        result
    }

    /// Execute a asynchronous block of code wrapped by the context
    ///
    /// This will lead to context initialisation and execution of before/after hooks
    ///
    /// Usage example:
    /// ```
    /// # use context_manager::SyncWrapContext;
    /// struct PrintDuration;
    /// impl<T> SyncWrapContext<T> for PrintDuration {
    ///   fn new() -> Self { Self }
    /// }
    ///
    /// # async fn foo() {
    /// let async_run_output: &'static str = PrintDuration::run_async(async {
    ///     "async"
    /// }).await;
    /// # }
    /// ```
    #[allow(async_fn_in_trait)]
    async fn run_async(block: impl Future<Output = T>) -> T
    where
        Self: Sized,
    {
        let context = Self::new();
        context.before();
        let result = block.await;
        context.after(&result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::SyncWrapContext;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    #[test]
    fn wrapper_usage_on_sync_function() {
        static VALUE: AtomicUsize = AtomicUsize::new(100);

        struct Sync;
        impl SyncWrapContext<usize> for Sync {
            fn new() -> Self {
                Self
            }

            fn before(&self) {
                // Reset the value to 0
                VALUE.store(0, Ordering::Relaxed);
                // Which will be verified in the function execution
            }

            fn after(self, result: &usize) {
                VALUE.store(2 * (*result), Ordering::Relaxed);
            }
        }

        assert_eq!(
            Sync::run_sync(|| {
                assert_eq!(VALUE.load(Ordering::Relaxed), 0);
                42
            }),
            42,
        );

        // The return value is doubled in the after hook
        assert_eq!(VALUE.load(Ordering::Relaxed), 84);
    }

    #[tokio::test]
    async fn wrapper_usage_on_async_function() {
        static VALUE: AtomicUsize = AtomicUsize::new(100);

        struct Sync;
        impl SyncWrapContext<usize> for Sync {
            fn new() -> Self {
                Self
            }

            fn before(&self) {
                // Reset the value to 0
                VALUE.store(0, Ordering::Relaxed);
                // Which will be verified in the function execution
            }

            fn after(self, result: &usize) {
                VALUE.store(2 * *result, Ordering::Relaxed);
            }
        }

        assert_eq!(
            Sync::run_async(async {
                assert_eq!(VALUE.load(Ordering::Relaxed), 0);
                42
            })
            .await,
            42
        );

        // The return value is doubled in the after hook
        assert_eq!(VALUE.load(Ordering::Relaxed), 84);
    }
}
