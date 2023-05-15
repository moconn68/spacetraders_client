/// Utilities related to the application's configuration.
pub mod config {
    use serde::{Deserialize, Serialize};
    use std::{fs, path::PathBuf};

    const CONFIG_FILE_NAME: &str = "config.json";

    pub type ConfigResult<T> = Result<T, ConfigError>;

    #[derive(Clone, Copy, Debug)]
    pub enum ConfigError {
        FileWrite,
    }

    /// The shape of the data contained in the config file.
    #[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
    pub struct ConfigData {
        /// User's auth token for the SpaceTraders API.
        pub token: String,
    }

    /// Get the default config file path based on the application root directory and default config file name.
    ///
    /// Returns the [`PathBuf`] default path to the config file.
    fn get_default_config_file_path() -> PathBuf {
        let mut config_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        config_file_path.push(CONFIG_FILE_NAME);
        config_file_path
    }

    /// Middleware function for reading config data from the config file.
    ///
    /// * `config_file_path` - [`PathBuf`] path to the config file.
    ///
    /// Returns client [`ConfigData`], or [`Option::None`] if the file cannot be read or contains no data.
    fn read_config_file(config_file_path: PathBuf) -> Option<ConfigData> {
        let config_data_str = fs::read_to_string(config_file_path).ok()?;
        serde_json::from_str::<ConfigData>(&config_data_str).ok()
    }

    /// Public convenience wrapper for [`read_config_file`] using the default config file path.
    ///
    /// Returns client [`ConfigData`], or [`Option::None`] if the file cannot be read or contains no data.
    pub fn read_default_config_file() -> Option<ConfigData> {
        read_config_file(get_default_config_file_path())
    }

    /// Middleware function for writing config data to the config file.
    ///
    /// * `config_data` - [`ConfigData`] to be written.
    /// * `config_file_path` - [`PathBuf`] path to the config file.
    ///
    /// Returns [`ConfigResult`] containing unit on success, or [`ConfigError::FileWrite`] if the operation fails.
    fn write_config_file(config_data: ConfigData, config_file_path: PathBuf) -> ConfigResult<()> {
        let config_file = fs::File::create(config_file_path).map_err(|_| ConfigError::FileWrite)?;
        serde_json::to_writer_pretty(config_file, &config_data).map_err(|_| ConfigError::FileWrite)
    }

    /// Public convenience wrapper for [`write_config_file`] using the default config file path.
    ///
    /// * `config_data` - [`ConfigData`] to be written.
    ///
    /// Returns [`ConfigResult`] containing unit on success, or [`ConfigError::FileWrite`] if the operation fails.
    pub fn write_default_config_file(config_data: ConfigData) -> ConfigResult<()> {
        write_config_file(config_data, get_default_config_file_path())
    }

    #[cfg(test)]
    mod tests {
        use super::{read_config_file, write_config_file, ConfigData};

        use tempfile;

        #[test]
        fn verify_read_config_file() {
            // Setup
            let expected_config_data = ConfigData {
                token: String::from("TEST_READ_TOKEN"),
            };

            let tmp_config_file = tempfile::NamedTempFile::new().unwrap();
            serde_json::to_writer_pretty(&tmp_config_file, &expected_config_data).unwrap();

            // Test
            let actual_config_data = read_config_file(tmp_config_file.path().to_path_buf());

            // Verify
            assert!(actual_config_data.is_some());
            assert_eq!(actual_config_data.unwrap(), expected_config_data);
        }

        #[test]
        fn verify_write_config_file() {
            // Setup
            let tmp_config_file = tempfile::NamedTempFile::new().unwrap();
            let test_config_data = ConfigData {
                token: String::from("TEST_WRITE_TOKEN"),
            };

            // Test
            write_config_file(
                test_config_data.clone(),
                tmp_config_file.path().to_path_buf(),
            )
            .unwrap();

            // Verify
            let final_cfg: Option<ConfigData> = serde_json::from_reader(&tmp_config_file).unwrap();
            assert!(final_cfg.is_some());
            assert_eq!(test_config_data, final_cfg.unwrap());
        }
    }
}
