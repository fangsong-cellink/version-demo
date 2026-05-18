// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::Serialize;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize)]
struct BuildInfo {
    version: String,
    build_id: String,
    commit_sha: String,
    run_id: String,
    run_attempt: String,
}

#[tauri::command]
fn get_build_info() -> BuildInfo {
    let version = option_env!("APP_VERSION")
        .map(|s| s.to_string())
        .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string());
    let build_id = option_env!("BUILD_ID").unwrap_or("local").to_string();
    let commit_sha = option_env!("COMMIT_SHA").unwrap_or("unknown").to_string();
    let run_id = option_env!("GITHUB_RUN_ID").unwrap_or("").to_string();
    let run_attempt = option_env!("GITHUB_RUN_ATTEMPT").unwrap_or("").to_string();

    BuildInfo {
        version,
        build_id,
        commit_sha,
        run_id,
        run_attempt,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_build_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
