use tauri_plugin_shell::ShellExt;
use tauri::WebviewUrl;
use std::env;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let handle = app.handle().clone();
            
            // Read the first argument passed to the CLI (index 0 is the exe itself)
            let cli_path = env::args().nth(1).unwrap_or_default();
            
            // Escape backslashes for Windows paths so it doesn't break the JS string
            let safe_path = cli_path.replace("\\", "\\\\").replace("\"", "\\\"");
            let init_script = format!("window.__CLI_FILE_PATH__ = \"{}\";", safe_path);
            
            tauri::WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::App("index.html".into())
            )
            .title("Smart HTML Renderer")
            .initialization_script(&init_script) // Inject the path into JS before page loads
            .on_navigation(move |url| {
                let scheme = url.scheme();
                let host = url.host_str().unwrap_or("");
                
                // Added "asset.localhost" so Windows allows the local file to load in-app
                if host == "tauri.localhost" || host == "localhost" || host == "asset.localhost" || scheme == "asset" || scheme == "tauri" {
                    return true;
                }

                // Intercept normal external web links and send them to default OS browser
                if scheme == "http" || scheme == "https" {
                    let _ = handle.shell().open(url.as_str(), None);
                    return false;
                }
                
                true
            })
            .build()?;
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}