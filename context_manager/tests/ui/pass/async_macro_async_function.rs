use context_manager::AsyncWrapContext;
use context_manager_macro::async_wrap;
use std::fmt::Debug;

struct Async;
impl<T> AsyncWrapContext<T> for Async {
    async fn new() -> Self {
        Self
    }
}

#[async_wrap(Async)]
async fn async_foo<'a, T: Debug>(v: &'a T) -> String {
    format!("{:?}", v)
}

#[tokio::main]
async fn main() {
    assert_eq!(async_foo(&10).await, "10");
}
