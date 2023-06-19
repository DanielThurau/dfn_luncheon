use ic_canister_serve::serve_logs;
use ic_cdk::api::management_canister::http_request::{CanisterHttpRequestArgument, HttpResponse};
use luncheon::{
    c_log,
    logs::{ERROR, INFO},
    types::{Restaurant, RollingLunchOrderEntry},
};
use luncheon_interface::{
    AddRestaurantRequest, AddRestaurantResponse, GetRollingLunchOrderResponse,
    GetRollingLunchOrdersRequest,
};
use std::{cell::RefCell, collections::BTreeMap};

thread_local! {
    // This is represents as a btreemap because it has deterministic ordering
    // and can be iterated like a list
    static ROLLING_LUNCH_ORDERS: RefCell<BTreeMap<u64, RollingLunchOrderEntry>> = RefCell::new(Default::default());

    static RESTAURANTS: RefCell<BTreeMap<String, Restaurant>> = RefCell::new(Default::default());
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    c_log!(INFO, "Hello, {}", name);
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn get_rolling_lunch_order_v1(
    _request: GetRollingLunchOrdersRequest,
) -> GetRollingLunchOrderResponse {
    c_log!(INFO, "get_rolling_lunch_order_v1");
    todo!()
}

#[ic_cdk::update]
fn add_restaurant_v1(request: AddRestaurantRequest) -> AddRestaurantResponse {
    RESTAURANTS.with(|restaurants| {
        luncheon::add_restaurant(request, ic_cdk::caller(), &mut restaurants.borrow_mut())
    })
}

#[ic_cdk::query]
fn http_request(request: CanisterHttpRequestArgument) -> HttpResponse {
    let path = match request.url.find('?') {
        None => &request.url[..],
        Some(index) => &request.url[..index],
    };

    match path {
        "/logs" => serve_logs(request, &INFO, &ERROR),
        _ => HttpResponse {
            status: 404.into(),
            body: "not_found".into(),
            ..Default::default()
        },
    }
}

fn main() {}
