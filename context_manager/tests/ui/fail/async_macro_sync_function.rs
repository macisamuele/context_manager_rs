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
fn sync_foo<'a, T: Debug>(v: &'a T) -> String {
    format!("{:?}", v)
}

fn main() {
    assert_eq!(sync_foo(&10), "10");
}
