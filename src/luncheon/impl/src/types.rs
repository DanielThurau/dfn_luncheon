use time::Date;
use url::Url;

pub struct Restaurant {
    pub name: String,
    pub url: Url,
    pub stats: Stats,
}

pub struct RollingLunchOrderEntry {
    pub restaurant: Restaurant,
    pub date: Date,
}

pub struct Stats {}
