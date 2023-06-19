use crate::types::{Restaurant, RollingLunchOrderEntry, Stats};
use luncheon_interface::{
    AddRestaurantError, AddRestaurantRequest, AddRestaurantResponse, AddRestaurantResult,
};
use std::{cell::RefCell, collections::BTreeMap};
use url::{ParseError, Url};

pub mod logs;
pub mod types;

pub fn add_restaurant(
    request: AddRestaurantRequest,
    _caller: candid::Principal, // TODO authorization
    restaurants: &mut BTreeMap<String, Restaurant>,
) -> AddRestaurantResponse {
    let parsed_url = match Url::parse(&request.url) {
        Ok(url) => url,
        Err(err) => {
            return AddRestaurantResponse {
                result: AddRestaurantResult::Err(AddRestaurantError {
                    message: err.to_string(),
                }),
            }
        }
    };

    restaurants.insert(
        request.name.clone(),
        Restaurant {
            name: request.name.clone(),
            url: parsed_url,
            stats: Stats {},
        },
    );

    AddRestaurantResponse {
        result: AddRestaurantResult::Ok("".to_string()),
    }
}
