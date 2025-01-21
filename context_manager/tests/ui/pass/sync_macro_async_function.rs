use context_manager::SyncWrapContext;
use context_manager_macro::wrap;
use std::fmt::Debug;

struct Sync;
impl<T> SyncWrapContext<T> for Sync {
    fn new() -> Self {
        Self
    }
}

#[wrap(Sync)]
async fn async_foo<'a, T: Debug>(v: &'a T) -> String {
    format!("{:?}", v)
}

#[tokio::main]
async fn main() {
    assert_eq!(async_foo(&10).await, "10");
}
