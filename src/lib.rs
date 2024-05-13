use std::sync::atomic::{AtomicBool, Ordering};

pub static DUMMY: AtomicBool = AtomicBool::new(false);

#[ctor::ctor]
fn dummy() {
    libc_print::libc_println!("lib");
}

#[inline(never)]
pub fn touch_the_dummy() -> bool {
    DUMMY.load(Ordering::SeqCst)
}
