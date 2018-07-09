#[macro_use] extern crate serde_derive;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate failure;
extern crate url;

use apod::Apod;

use serde::de::DeserializeOwned;
use reqwest::Client;
use url::Url;

pub mod apod;

pub struct NasapiClient {
    api_key: String
}

impl NasapiClient {
    pub fn new(api_key: String) -> NasapiClient {
        NasapiClient { api_key }
    }

    pub fn apod(&self) -> Apod {
        Apod { nasapi: &self }
    }

    fn send_request<T>(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<T, failure::Error>
        where T: DeserializeOwned
    {
        let url = Url::parse_with_params(&("https://api.nasa.gov/".to_owned() + endpoint), params);
        let client = Client::new();
        let mut request = client.get(url.unwrap().as_str());
        let mut response = request.send()?;
        Ok(response.json()?)
    }
}
