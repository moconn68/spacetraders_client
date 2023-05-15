mod api;
mod utils;

/// Used for quick & dirty prototyping and validation; won't be kept around long-term.
pub mod proto {
    use crate::{
        api::{
            client::{ApiClient, TraderApis},
            data::Factions,
        },
        utils::{self, config::ConfigData},
    };

    pub fn test_create_api_client() -> ApiClient {
        let api_client = match ApiClient::init() {
            Ok(client) => {
                println!("Successfully initialized client: {client:#?}");
                client
            }
            Err(e) => {
                println!("Error initializing API client: {e}");
                ApiClient::new("TEST_AGENT", Factions::Astro)
                    .expect("Error creating new Api client")
            }
        };

        println!("API Client: {api_client:#?}");
        api_client
    }

    pub fn test_agent_data(api_client: &ApiClient) {
        println!("Getting agent data:");
        let agent_data = api_client
            .get_agent_data()
            .expect("Error getting agent data!");
        println!("{agent_data}");
    }

    pub fn test_location_data(api_client: &ApiClient) {
        println!("Getting location data:");
        let location_data = api_client
            .get_waypoint_location_data("X1-DF55-20250Z")
            .expect("Error getting location data!");
        println!("{location_data}");
    }

    pub fn test_read_config() {
        let token = utils::config::read_default_config_file();
        println!("Token from config: {token:?}");
    }

    pub fn test_write_config(token: impl std::fmt::Display) {
        println!("Config before: ");
        test_read_config();

        println!("Writing token {token}");
        utils::config::write_default_config_file(ConfigData {
            token: token.to_string(),
        })
        .unwrap();

        println!("Config after: ");
        test_read_config();
    }
}
