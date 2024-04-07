
#[derive(serde::Deserialize)]
pub struct Settings{
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings{
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,self.password,self.host,self.port,self.database_name
        )
    } 

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username,self.password,self.host,self.port
        )
    } 
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;

    settings.try_deserialize()
}


#[cfg(test)]
mod test {
    use super::get_configuration;

    #[test]
    fn should_get_configuration() {
        let cfg = get_configuration().expect("failed to get cfg");
        assert_eq!("postgres://postgres:password@127.0.0.1:5432/newsletter", cfg.database.connection_string());
    }
}