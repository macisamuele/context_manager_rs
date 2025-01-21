use context_manager::SyncWrapContext;
use context_manager_macro::wrap;

struct Sync;
impl<T> SyncWrapContext<T> for Sync {
    fn new() -> Self {
        Self
    }
}

#[wrap(Sync)]
const fn sync_foo<'a, T: ?Sized>(v: &'a T) -> &'a T {
    v
}

fn main() {
    assert_eq!(sync_foo("10"), "10");
}
