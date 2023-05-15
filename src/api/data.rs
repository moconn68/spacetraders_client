use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Implement a standard pretty-print Display trait for a struct based on Debug.
///
/// This is a bare-bones macro only intended for use with these specific data classes only.
/// The implementation is brittle and will only work if $structname already derives/implements Debug!
macro_rules! impl_pretty_disp {
    ($structname: ident) => {
        impl Display for $structname {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, "{self:#?}")
            }
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiResponse<T> {
    Data(T),
    Error(ErrorResponse),
}

/// Shape of errors that come from the SpaceTraders API - see https://docs.spacetraders.io/api-guide/response-errors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub code: i32,
    pub data: Option<HashMap<String, serde_json::Value>>,
}

/// Data that is returned when a new agent is created in the game.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationData {
    pub token: String,
    pub agent: AgentData,
    pub contract: ContractData,
    pub faction: FactionData,
    pub ship: ShipData,
}

/// Basic information about a given player agent.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentData {
    pub account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: i64,
}
impl_pretty_disp!(AgentData);

/// Information about contracts AKA missions.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractData {
    pub id: String,
    pub faction_symbol: String,
    pub r#type: String,
    pub terms: ContractTerms,
    pub accepted: bool,
    pub fulfilled: bool,
    pub expiration: String,
}

/// Metadata about contracts AKA missions.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractTerms {
    pub deadline: String,
    pub payment: PaymentInfo,
    pub deliver: Vec<DeliveryInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInfo {
    pub on_accepted: i64,
    pub on_fulfilled: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryInfo {
    pub trade_symbol: String,
    pub destination_symbol: String,
    pub units_required: i64,
    pub units_fulfilled: i64,
}

/// Names of the various factions currently in the game.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Factions {
    Cosmic,
    Void,
    Galactic,
    Quantum,
    Dominion,
    Astro,
    Corsairs,
}
impl Display for Factions {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self:?}")
    }
}

/// Metadata pertaining to each faction.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactionData {
    pub symbol: Factions,
    pub name: String,
    pub description: String,
    pub headquarters: String,
    pub traits: Vec<TraitData>,
}

/// General characteristics, currently used for factions and locations.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraitData {
    pub symbol: String,
    pub name: String,
    pub description: String,
}

/// Metadata associated with a given ship.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipData {
    pub symbol: String,
    pub nav: NavInfo,
    pub crew: CrewInfo,
    pub fuel: FuelInfo,
    pub frame: FrameInfo,
    pub reactor: ReactorInfo,
    pub engine: EngineInfo,
    pub modules: Vec<ModuleInfo>,
    pub mounts: Vec<MountInfo>,
    pub registration: ShipRegistration,
    pub cargo: CargoInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavInfo {
    pub system_symbol: String,
    pub waypoint_symbol: String,
    pub route: Route,
    pub status: String,
    pub flight_mode: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub departure: LocationData,
    pub destination: LocationData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrewInfo {
    pub current: u16,
    pub capacity: u16,
    pub required: u16,
    pub rotation: String,
    pub morale: u8,
    pub wages: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuelInfo {
    pub current: u32,
    pub capacity: u32,
    pub consumed: ConsumedFuel,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsumedFuel {
    pub amount: u32,
    pub timestamp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentInfo {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub condition: Option<u8>,
    pub requirements: ComponentRequirements,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentRequirements {
    pub crew: Option<u8>,
    pub power: Option<u8>,
    pub slots: Option<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameInfo {
    #[serde(flatten)]
    pub component_info: ComponentInfo,
    pub module_slots: u8,
    pub mounting_points: u8,
    pub fuel_capacity: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReactorInfo {
    #[serde(flatten)]
    pub component_info: ComponentInfo,
    pub power_output: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineInfo {
    #[serde(flatten)]
    pub component_info: ComponentInfo,
    pub speed: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleInfo {
    #[serde(flatten)]
    pub component_info: ComponentInfo,
    pub capacity: Option<u16>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MountInfo {
    #[serde(flatten)]
    pub component_info: ComponentInfo,
    pub strength: u8,
    pub deposits: Option<Vec<String>>,
}

/// To what agent a given ship is registered to.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipRegistration {
    pub name: String,
    pub faction_symbol: Factions,
    pub role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CargoInfo {
    pub capacity: u32,
    pub units: u32,
    pub inventory: Vec<CargoItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CargoItem {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub units: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationData {
    pub system_symbol: String,
    pub symbol: String,
    pub r#type: String,
    #[serde(flatten)]
    pub coords: Coords,
    pub orbitals: Option<Vec<HashMap<String, String>>>,
    pub traits: Option<Vec<TraitData>>,
    pub chart: Option<HashMap<String, String>>,
    pub faction: Option<HashMap<String, String>>,
}
impl_pretty_disp!(LocationData);
