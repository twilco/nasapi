use NasapiClient;
use apod::models::*;

pub mod models;

pub struct Apod<'a> {
    pub nasapi: &'a NasapiClient
}

impl<'a> Apod<'a> {
    pub fn for_date(&self, year: u16, month: u8, day: u8, hd: bool) {
    }

    pub fn for_today(&self, hd: bool) -> Result<Pic, ::failure::Error> {
        let pic = self.nasapi.send_request("planetary/apod", &[("api_key", self.nasapi.api_key.as_str())]);
        println!("pic: {:?}", pic);
        pic
    }
}