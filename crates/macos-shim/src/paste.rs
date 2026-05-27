use std::thread;
use std::time::Duration;

use arboard::Clipboard;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};

/// kVK_ANSI_V virtual keycode for the V key on macOS.
/// enigo 0.2 Key::Unicode('v') traps with SIGTRAP when Cmd is held.
const KVK_ANSI_V: u16 = 9;

pub fn paste_text(text: &str) -> anyhow::Result<()> {
    let mut clipboard = Clipboard::new()?;
    let prior = clipboard.get_text().ok();

    clipboard.set_text(text)?;
    tracing::debug!("clipboard set");

    // Synthesize Cmd+V. Wait for focus to settle first (150ms — spike-validated).
    thread::sleep(Duration::from_millis(150));
    let mut enigo = Enigo::new(&Settings::default())?;
    enigo.key(Key::Meta, Direction::Press)?;
    enigo.key(Key::Other(KVK_ANSI_V as u32), Direction::Click)?;
    enigo.key(Key::Meta, Direction::Release)?;
    tracing::debug!("cmd+v synthesized");

    // Restore prior clipboard after paste settles (200ms — spike).
    if let Some(prior) = prior {
        thread::sleep(Duration::from_millis(200));
        clipboard.set_text(prior)?;
        tracing::debug!("clipboard restored");
    }
    Ok(())
}
