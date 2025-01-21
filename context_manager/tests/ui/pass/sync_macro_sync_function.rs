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
fn sync_foo<'a, T: Debug>(v: &'a T) -> String {
    format!("{:?}", v)
}

fn main() {
    assert_eq!(sync_foo(&10), "10");
}
