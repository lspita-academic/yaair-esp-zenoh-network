pub mod wifi;

pub struct InitOptions {
    pub enable_logger: bool,
}

impl Default for InitOptions {
    fn default() -> Self {
        Self {
            enable_logger: true,
        }
    }
}

pub fn init(options: InitOptions) {
    esp_idf_svc::sys::link_patches();
    if options.enable_logger {
        esp_idf_svc::log::init_from_env();
    }
}
