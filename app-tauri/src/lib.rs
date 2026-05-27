use tauri::{ActivationPolicy, Manager};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

fn hotkey() -> Shortcut {
    Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::Space)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcut(hotkey())
                .unwrap()
                .with_handler(|app, shortcut, event| {
                    if shortcut == &hotkey() && event.state == ShortcutState::Pressed {
                        if let Some(win) = app.get_webview_window("overlay") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                })
                .build(),
        )
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
