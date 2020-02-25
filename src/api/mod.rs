

extern crate serde_derive;
use reqwest::{Error as ReqError, Method};
use reqwest::blocking::Response;
use crate::config::Configuration;

#[derive(Deserialize, Debug)]
pub struct Droplet {
    pub id: u32,
    pub name: String,
    pub size_slug: String,
    pub disk: u8,
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct Droplets {
    pub droplets: Vec<Droplet>
}

pub fn call_do(configuration: &Configuration, api_path: String, method: Method) -> Result<Response, ReqError> {
    let url = format!("https://api.digitalocean.com/v2/{}", api_path);
    let client = reqwest::blocking::Client::new();
    let response = client.request(method, &url)
    .bearer_auth(configuration.do_token.as_str())
    .header("Content-Type", "application/json")
    .send();
    response
}