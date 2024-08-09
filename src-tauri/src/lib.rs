// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn request_get(link: &str) -> Result<String, String> {
    let response = reqwest::get(link)
        .await
        .map_err(|e| e.to_string())
        .expect("Failed to send request");
    let body = response
        .text()
        .await
        .map_err(|e| e.to_string())
        .expect("Failed to read response body");
    Ok(body)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![request_get])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
