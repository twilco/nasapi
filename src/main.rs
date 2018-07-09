extern crate nasapi;

use std::env;

fn main() {
    let client = nasapi::NasapiClient::new(env::var("NASA_API_KEY").unwrap());
    client.apod().for_today(false);
}