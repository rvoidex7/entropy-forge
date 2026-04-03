pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::use_tab::encrypt_decrypt,
            commands::test_tab::run_quality_tests,
            commands::bench_tab::run_benchmark,
            commands::learn_tab::get_xor_steps,
            commands::learn_tab::get_entropy_steps,
            commands::learn_tab::get_nist_steps,
            commands::learn_tab::get_nist_steps_random
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
