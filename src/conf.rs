use config::Config;

#[derive(serde::Deserialize)]
pub struct Configuration {
    pub vault: String,
    pub id: String,
}

pub fn find_configuration_file() -> Option<String> {
    let mut current_dir = std::env::current_dir().ok()?;
    loop {
        let config_file = current_dir.join(".op_git.toml");
        if config_file.exists() {
            return Some(config_file.to_str()?.to_string());
        }

        if !current_dir.pop() {
            break;
        }
    }

    None
}

pub fn load_configuration() -> Result<Configuration, config::ConfigError> {
    let config_file = find_configuration_file()
        .ok_or(config::ConfigError::NotFound(".op_git.toml".to_string()))?;
    let settings = Config::builder()
        .add_source(config::File::with_name(config_file.as_str()))
        .build()?;

    settings.try_deserialize::<Configuration>()
}
