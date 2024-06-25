use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub organization_id: String,
    pub yandex_client_id: String,
    pub yandex_client_secret: String,
    pub redirect_uri: String,
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }

    pub fn global() -> &'static Self {
        static INSTANCE: OnceCell<Config> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            Config::from_file("config.toml").expect("Failed to load configuration from config.toml")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_file() {
        // Создадим временный конфигурационный файл для теста
        let config_content = r#"
        organization_id = "test_org_id"
        yandex_client_id = "test_client_id"
        yandex_client_secret = "test_client_secret"
        redirect_uri = "http://localhost:8080/redirect"
        "#;

        let file_path = "test_config.toml";
        std::fs::write(file_path, config_content).expect("Failed to write test config file");

        let config = Config::from_file(file_path).expect("Failed to load configuration from file");

        assert_eq!(config.organization_id, "test_org_id");
        assert_eq!(config.yandex_client_id, "test_client_id");
        assert_eq!(config.yandex_client_secret, "test_client_secret");
        assert_eq!(config.redirect_uri, "http://localhost:8080/redirect");

        // Удалим временный файл после теста
        std::fs::remove_file(file_path).expect("Failed to delete test config file");
    }
}