mod api;

use crate::api::client::{ApiClient, TraderApis};

fn main() {
    println!("Getting agent data:");
    let api_client = ApiClient::default();
    let agent_data = api_client
        .get_agent_data()
        .expect("Error getting agent data!");
    println!("{agent_data}");

    println!("Getting location data:");
    let location_data = api_client
        .get_location("X1-DF55-20250Z")
        .expect("Error getting location data!");
    println!("{location_data}");
}
