use directories::UserDirs;
use prompt_store::FsPromptStore;
use tauri::{ActivationPolicy, Manager};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

pub mod commands;

use commands::AppState;

fn hotkey() -> Shortcut {
    Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::Space)
}

fn prompts_root() -> Result<std::path::PathBuf, String> {
    let user_dirs = UserDirs::new().ok_or_else(|| "no home directory".to_string())?;
    Ok(user_dirs.home_dir().join(".prompts"))
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
        .invoke_handler(tauri::generate_handler![
            commands::search,
            commands::select_prompt,
            commands::hide_overlay,
        ])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            let root = prompts_root()?;
            let store = FsPromptStore::new(root)?;
            app.manage(AppState::new(store));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
