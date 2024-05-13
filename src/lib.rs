use std::sync::atomic::AtomicBool;

pub static DUMMY: AtomicBool = AtomicBool::new(false);

#[ctor::ctor]
fn dummy() {
    libc_print::libc_println!("lib");
}
