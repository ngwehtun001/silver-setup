//! `SilverSetup` desktop composition root.

/// Starts the native application without exposing product commands.
///
/// # Errors
///
/// Returns a Tauri runtime error if the desktop host cannot start or exits unexpectedly.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> tauri::Result<()> {
    tauri::Builder::default().run(tauri::generate_context!())
}
