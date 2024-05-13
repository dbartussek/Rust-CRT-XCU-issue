#[ctor::ctor]
fn dummy() {
    libc_print::libc_println!("binary");
}

fn main() {
    #[cfg(feature = "dummy_touch")]
    if std::hint::black_box(false) {
        nested_init_test::DUMMY.load(std::sync::atomic::Ordering::SeqCst);
    }

    println!("Main");
}
