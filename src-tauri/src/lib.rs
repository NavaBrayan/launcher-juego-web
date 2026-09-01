#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .on_page_load(|window, _payload| {
            // Inyectar un script para forzar que los enlaces target="_blank" o popups de Google abran en el sistema si es necesario
            let _ = window.eval(r#"
                window.open = (function(open) {
                    return function(url, name, features) {
                        if (url && (url.includes('accounts.google.com') || url.includes('google.com/o/oauth2'))) {
                            if (window.__TAURI_OS_PLUGIN_OPENER__) {
                                window.__TAURI_OS_PLUGIN_OPENER__.openUrl(url);
                                return null;
                            }
                        }
                        return open.call(window, url, name, features);
                    };
                })(window.open);
            "#);
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}