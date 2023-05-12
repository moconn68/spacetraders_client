use super::data::{AgentData, ApiResponse, ErrorResponse, LocationData};

use reqwest::{blocking::Client, header};
use std::{fs, path::PathBuf};

// API Routes
const ROOT_URL: &str = "https://api.spacetraders.io/v2";

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    Network(reqwest::Error),
    BadRequest(ErrorResponse),
}

trait HttpClient {
    fn get(&self, url: &str) -> Result<reqwest::blocking::Response, reqwest::Error>;
}

pub trait TraderApis {
    fn get_agent_data(&self) -> ApiResult<AgentData>;
    fn get_location(&self, waypoint: &str) -> ApiResult<LocationData>;
}

#[derive(Debug, Clone)]
pub struct ApiClient {
    http_client: Client,
    token: String,
}

impl Default for ApiClient {
    fn default() -> Self {
        let mut dotenv_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dotenv_path.push(".env");

        Self {
            http_client: Default::default(),
            token: fs::read_to_string(&dotenv_path)
                .expect(&format!(
                    "Could not read token from dotenv at {:?}",
                    &dotenv_path
                ))
                .trim()
                .to_owned(),
        }
    }
}

impl HttpClient for ApiClient {
    fn get(&self, url: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.http_client
            .get(url)
            .bearer_auth(&self.token)
            .header(header::CONTENT_TYPE, "application/json")
            .send()
    }
}

impl TraderApis for ApiClient {
    fn get_agent_data(&self) -> ApiResult<AgentData> {
        let url = format!("{ROOT_URL}/my/agent");

        let api_response: ApiResponse<AgentData> = self
            .get(&url)
            .map_err(|reqwest_err| ApiError::Network(reqwest_err))?
            .json()
            .expect("Error parsing API response JSON!");

        match api_response {
            ApiResponse::Data(agent_data) => Ok(agent_data),
            ApiResponse::Error(error) => Err(ApiError::BadRequest(error)),
        }
    }

    fn get_location(&self, waypoint: &str) -> ApiResult<LocationData> {
        let system = waypoint
            .split("-")
            .map(String::from)
            .collect::<Vec<String>>()[0..=1]
            .join("-");
        let url = format!("{ROOT_URL}/systems/{system}/waypoints/{waypoint}",);

        let api_response: ApiResponse<LocationData> = self
            .get(&url)
            .map_err(|reqwest_err| ApiError::Network(reqwest_err))?
            .json()
            .expect("Error parsing API response JSON!");

        match api_response {
            ApiResponse::Data(location_data) => Ok(location_data),
            ApiResponse::Error(error) => Err(ApiError::BadRequest(error)),
        }
    }
}
