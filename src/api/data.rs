use std::{collections::HashMap, fmt::Display};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiResponse<T> {
    Data(T),
    Error(ErrorResponse),
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    message: String,
    code: i32,
    data: Option<HashMap<String, String>>,
}
impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentData {
    pub account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: i32,
}
impl Display for AgentData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationData {
    pub system_symbol: String,
    pub symbol: String,
    pub r#type: String,
    pub x: i32,
    pub y: i32,
    pub orbitals: Vec<HashMap<String, String>>,
    pub traits: Vec<HashMap<String, String>>,
    pub chart: HashMap<String, String>,
    pub faction: HashMap<String, String>,
}
impl Display for LocationData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}
