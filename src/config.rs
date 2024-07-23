use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::fs;

/// Represents the configuration needed for the application.
///
/// # Fields
///
/// * `organization_id` - The ID of the organization.
/// * `yandex_client_id` - The client ID for Yandex.
/// * `yandex_client_secret` - The client secret for Yandex.
/// * `redirect_uri` - The redirect URI for the application.
#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub organization_id: String,
    pub yandex_client_id: String,
    pub yandex_client_secret: String,
    pub redirect_uri: String,
}

impl Config {
    /// Loads the configuration from a file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the configuration file.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Box<dyn std::error::Error>>` - The configuration object or an error.
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }

    /// Saves the configuration to a TOML file.
    ///
    /// This method serializes the `Config` instance to a TOML string and writes it to a file named `config_template.toml`.
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` - An empty result if successful, or an error if the operation fails.
    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = toml::to_string_pretty(self)?;
        fs::write("config_template.toml", config_str)?;
        Ok(())
    }

    /// Returns a global configuration instance.
    ///
    /// This method initializes the configuration from `config.toml` if it has not been initialized yet,
    /// and then returns a reference to the configuration.
    ///
    /// # Panics
    ///
    /// This method will panic if the configuration file cannot be loaded.
    pub fn global() -> &'static Self {
        static INSTANCE: OnceCell<Config> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            Config::from_file("config.toml").expect("Failed to load configuration from config.toml")
        })
    }
}

impl Default for Config {
    /// Creates a default `Config` instance.
    ///
    /// This implementation provides default values for all fields.
    /// The `organization_id`, `yandex_client_id`, and `yandex_client_secret` fields
    /// are set to placeholder strings with explanations, and the `redirect_uri`
    /// is set to "http://localhost:8000".
    ///
    /// # Returns
    ///
    /// A `Config` instance with default values.
    fn default() -> Self {
        Config {
            organization_id: "default_org_id: The ID of the organization".to_string(),
            yandex_client_id: "default_client_id: The client ID for Yandex".to_string(),
            yandex_client_secret: "default_client_secret: The client secret for Yandex".to_string(),
            redirect_uri: "http://localhost:8000: The redirect URI for the application".to_string(),
        }
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
