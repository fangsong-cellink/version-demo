fn main() {
    // Forward CI-provided variables into the compiled binary so they are
    // available at runtime via `option_env!`.
    if let Ok(run_id) = std::env::var("GITHUB_RUN_ID") {
        let attempt = std::env::var("GITHUB_RUN_ATTEMPT").unwrap_or_else(|_| "1".into());
        println!("cargo:rustc-env=BUILD_ID={}-{}", run_id, attempt);
        println!("cargo:rustc-env=GITHUB_RUN_ID={}", run_id);
        println!("cargo:rustc-env=GITHUB_RUN_ATTEMPT={}", attempt);
    } else if let Ok(build_id) = std::env::var("BUILD_ID") {
        println!("cargo:rustc-env=BUILD_ID={}", build_id);
    }

    if let Ok(ver) = std::env::var("VERSION") {
        println!("cargo:rustc-env=APP_VERSION={}", ver);
    }

    if let Ok(sha) = std::env::var("COMMIT_SHA") {
        println!("cargo:rustc-env=COMMIT_SHA={}", sha);
    }

    tauri_build::build()
}
