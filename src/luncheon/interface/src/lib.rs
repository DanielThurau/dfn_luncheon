use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct GetRollingLunchOrdersRequest {

}

#[derive(CandidType, Deserialize)]
pub struct GetRollingLunchOrderResponse {
}

#[derive(CandidType, Deserialize)]
pub struct AddRestaurantRequest {
    pub name: String,
    pub url: String,
}

#[derive(CandidType, Deserialize)]
pub struct AddRestaurantError {
    pub message: String,
}

#[derive(CandidType, Deserialize)]
pub enum AddRestaurantResult {
    Ok(String),
    Err(AddRestaurantError),
}

#[derive(CandidType, Deserialize)]
pub struct AddRestaurantResponse {
    pub result: AddRestaurantResult,
}