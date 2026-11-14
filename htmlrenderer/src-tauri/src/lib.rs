use tauri_plugin_shell::ShellExt;
use tauri::WebviewUrl;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let handle = app.handle().clone();
            
            tauri::WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::App("index.html".into())
            )
            .title("Smart HTML Renderer")
            .on_navigation(move |url| {
                if url.scheme() == "http" || url.scheme() == "https" {
                    // Route external links to the default OS browser
                    let _ = handle.shell().open(url.as_str(), None);
                    false // Block the webview from navigating away
                } else {
                    true // Allow local navigation (asset://)
                }
            })
            .build()?;
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}