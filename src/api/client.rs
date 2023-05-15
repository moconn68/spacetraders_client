use super::data::{
    AgentData, ApiResponse, ErrorResponse, Factions, LocationData, RegistrationData,
};
use crate::utils::{self, config::ConfigData};

use reqwest::{blocking::Client, header};
use std::collections::HashMap;

// API Routes
const ROOT_URL: &str = "https://api.spacetraders.io/v2";

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    Network(reqwest::Error),
    MissingToken,
    BadRequest(ErrorResponse),
}
impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Network(e) => write!(
                f,
                "There was a networking error when trying to access the SpaceTraders API: {e:#?}"
            ),
            ApiError::MissingToken => {
                write!(f, "Cannot access the SpaceTraders API: missing auth token.")
            }
            ApiError::BadRequest(e) => {
                write!(f, "The SpaceTraders API rejected the request: {e:#?}")
            }
        }
    }
}

/// Encapsulates basic HTTP methods used by the API client under the hood.
trait HttpClient {
    /// Convenience method for HTTP GET with some SpaceTraders-specific defaults prefilled.
    ///
    /// * `url` - URL for the given HTTP endpoint.
    ///
    /// Returns the [`Response`](reqwest::blocking::Response) for a successful request, or the [`Error`](reqwest::Error).
    fn get(&self, url: &str) -> reqwest::Result<reqwest::blocking::Response>;

    /// Convenience method for HTTP POST with some SpaceTraders-specific defaults prefilled.
    ///
    /// * `request_body` - [`Body`](`reqwest::blocking::Body`) content of request.
    /// * `url` - URL for the given HTTP endpoint.
    ///
    /// Returns the [`Response`](reqwest::blocking::Response) for a successful request, or the [`Error`](reqwest::Error).
    fn post(
        &self,
        request_body: impl Into<reqwest::blocking::Body>,
        url: &str,
    ) -> reqwest::Result<reqwest::blocking::Response>;
}

/// All of the relevant methods for high-level interactions with the SpaceTrader API.
pub trait TraderApis {
    /// Register a new SpaceTraders agent and save its config data.
    ///
    /// * `agent_name` - desired name of new agent.
    /// * `faction_name` - [`Faction`](`Factions`) of new agent.
    ///
    /// Returns [`RegistrationData`] for new agent, or [`ApiError`] failure reason.
    fn register_new_agent(
        &self,
        agent_name: &str,
        faction_name: Factions,
    ) -> ApiResult<RegistrationData>;

    /// Gets data for the currently registered agent.
    ///
    /// Returns [`AgentData`] for the agent, or the [`ApiError`] reason for failure.
    fn get_agent_data(&self) -> ApiResult<AgentData>;

    /// Gets location data for a given waypoint.
    ///
    /// * `waypoint` - string representation of the given waypoint. This is expected
    /// to be in the format of "XX-YYYY-ZZZZZZ" where Xs constitute the sector and 'XX-YYYY' is the system.
    ///
    /// Returns [`LocationData`] for the waypoint, or the [`ApiError`] reason for failure.
    fn get_waypoint_location_data(&self, waypoint: &str) -> ApiResult<LocationData>;
}

/// Client interface for the SpaceTraders API. Uses HTTP requests under the hood to make these transactions.
#[derive(Debug, Clone)]
pub struct ApiClient {
    /// Underlying client for executing HTTP requests.
    http_client: Client,
    /// API token for the user's player agent necessary for authenticating API requests.
    token: String,
}

impl ApiClient {
    /// Initializes an [`ApiClient`] based on existing config data.
    ///
    /// Returns an [`ApiClient`] for your agent, or the [`ApiError`] reason for failure.
    pub fn init() -> ApiResult<Self> {
        let api_config_data =
            utils::config::read_default_config_file().ok_or(ApiError::MissingToken)?;
        Ok(Self {
            http_client: Default::default(),
            token: api_config_data.token,
        })
    }

    /// Creates a new [`ApiClient`] along with registering a new agent.
    ///
    /// This function should only be used to create a new agent - to get an instance of
    /// [`ApiClient`] for an existing config, use `init` instead.
    ///
    /// * `agent_name` - name of the agent you want to create.
    /// * `faction` - [`Faction`](`Factions`) you want your new agent to be in.
    ///
    /// Returns an [`ApiClient`] registered to your new agent, or the [`ApiError`] reason for failure.
    pub fn new(agent_name: &str, faction: Factions) -> ApiResult<Self> {
        let mut api_client = Self {
            http_client: Default::default(),
            token: Default::default(),
        };

        let registration_data = api_client.register_new_agent(agent_name, faction)?;
        api_client.token = registration_data.token;
        Ok(api_client)
    }
}

impl HttpClient for ApiClient {
    fn get(&self, url: &str) -> reqwest::Result<reqwest::blocking::Response> {
        self.http_client
            .get(url)
            .bearer_auth(&self.token)
            .header(header::CONTENT_TYPE, "application/json")
            .send()
    }

    fn post(
        &self,
        request_body: impl Into<reqwest::blocking::Body>,
        url: &str,
    ) -> reqwest::Result<reqwest::blocking::Response> {
        self.http_client
            .post(url)
            .header(header::CONTENT_TYPE, "application/json")
            .body(request_body)
            .send()
    }
}

impl TraderApis for ApiClient {
    fn register_new_agent(
        &self,
        agent_name: &str,
        faction_name: Factions,
    ) -> ApiResult<RegistrationData> {
        let url = format!("{ROOT_URL}/register");
        let request_body = format!(
            "{:?}",
            HashMap::from([
                ("symbol", agent_name),
                ("faction", &faction_name.to_string().to_uppercase())
            ])
        );

        let api_response: ApiResponse<RegistrationData> = self
            .post(request_body, &url)
            .map_err(|reqwest_err| ApiError::Network(reqwest_err))?
            .json::<ApiResponse<RegistrationData>>()
            .expect("Error parsing API JSON response");

        match api_response {
            ApiResponse::Data(registrion_data) => {
                utils::config::write_default_config_file(ConfigData {
                    token: registrion_data.token.clone(),
                })
                .expect("Error writing to config file!");
                Ok(registrion_data)
            }
            ApiResponse::Error(api_error) => Err(ApiError::BadRequest(api_error)),
        }
    }

    fn get_agent_data(&self) -> ApiResult<AgentData> {
        let url = format!("{ROOT_URL}/my/agent");

        let api_response: ApiResponse<AgentData> = self
            .get(&url)
            .map_err(|reqwest_err| ApiError::Network(reqwest_err))?
            .json()
            .expect("Error parsing API response JSON!");

        match api_response {
            ApiResponse::Data(agent_data) => Ok(agent_data),
            ApiResponse::Error(api_error) => Err(ApiError::BadRequest(api_error)),
        }
    }

    fn get_waypoint_location_data(&self, waypoint: &str) -> ApiResult<LocationData> {
        let system = waypoint
            .split("-")
            .map(String::from)
            .collect::<Vec<String>>()[0..=1]
            .join("-");
        let url = format!("{ROOT_URL}/systems/{system}/waypoints/{waypoint}");

        let api_response: ApiResponse<LocationData> = self
            .get(&url)
            .map_err(|reqwest_err| ApiError::Network(reqwest_err))?
            .json()
            .expect("Error parsing API response JSON!");

        match api_response {
            ApiResponse::Data(location_data) => Ok(location_data),
            ApiResponse::Error(api_error) => Err(ApiError::BadRequest(api_error)),
        }
    }
}
