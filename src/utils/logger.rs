use std::sync::Once;

static INIT: Once = Once::new();

pub fn init_logger() {
    INIT.call_once(|| {
        #[cfg(target_arch = "wasm32")]
        {
            // Configuración para la consola del Navegador
            wasm_logger::init(wasm_logger::Config::default());
            log::info!("Logger inicializado para Web (WASM)");
        }

        #[cfg(target_os = "android")]
        {
            // Configuración para Android Logcat
            android_logger::init_once(
                android_logger::Config::default()
                    .with_tag("RUST_RETAIL") // Etiqueta para filtrar en Logcat
                    .with_max_level(log::LevelFilter::Debug),
            );
            log::info!("Logger inicializado para Android");
        }

        #[cfg(target_os = "ios")]
        {
            // Configuración para iOS (Xcode Console)
            let mut config = oslog::OsLogger::new("com.tuempresa.rustretail");
            config.level_filter(log::LevelFilter::Debug).init().ok();
            log::info!("Logger inicializado para iOS");
        }

        #[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
        {
            // Fallback para escritorio (PC/Mac/Linux) durante desarrollo local
            //env_logger::init();
        }
    });
}