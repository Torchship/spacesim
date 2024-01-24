use env_logger::Env;

pub fn init() {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info") // Default log level
        .write_style_or("LOG_STYLE", "always");

    env_logger::Builder::from_env(env)
        .format_timestamp_secs()
        .init();
}