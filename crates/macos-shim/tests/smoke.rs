//! Compile-only smoke test. Real integration test lives in Phase 7's timing harness.
#[cfg(target_os = "macos")]
#[test]
fn it_compiles() {
    let _f = macos_shim::capture_frontmost_pid;
    let _r = macos_shim::restore_focus;
    let _p = macos_shim::paste_text;
}
