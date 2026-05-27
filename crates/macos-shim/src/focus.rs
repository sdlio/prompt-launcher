use objc2_app_kit::{NSApplicationActivationOptions, NSRunningApplication, NSWorkspace};

/// NSApplicationActivateIgnoringOtherApps = 1 << 1.
/// objc2-app-kit 0.2 exposes this as a `bitflags` constant but flags it `#[deprecated]`
/// — Apple deprecated `ignoringOtherApps` in macOS 14, yet the spike verified it is
/// still the load-bearing constant for focus-return on Sonoma+. Construct the
/// underlying NSUInteger directly to bypass the deprecation lint.
const ACTIVATE_IGNORING_OTHER_APPS: NSApplicationActivationOptions =
    NSApplicationActivationOptions(1 << 1);

pub fn capture_frontmost_pid() -> Option<i32> {
    unsafe {
        let workspace = NSWorkspace::sharedWorkspace();
        let app = workspace.frontmostApplication()?;
        Some(app.processIdentifier())
    }
}

pub fn restore_focus(pid: i32) -> bool {
    unsafe {
        let Some(app) = NSRunningApplication::runningApplicationWithProcessIdentifier(pid) else {
            tracing::warn!(pid, "no running app for pid");
            return false;
        };
        app.activateWithOptions(ACTIVATE_IGNORING_OTHER_APPS)
    }
}
