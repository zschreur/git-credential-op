use config::Config;

#[derive(serde::Deserialize)]
pub struct Configuration {
    pub vault: String,
    pub id: String,
}

pub fn load_configuration() -> Result<Configuration, config::ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name(".op_git.toml"))
        .build()?;

    settings.try_deserialize::<Configuration>()
}
