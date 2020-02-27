extern crate serde_derive;
use crate::config::Configuration;
use reqwest::blocking::Response;
use reqwest::{Error as ReqError, Method};

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
    pub droplets: Vec<Droplet>,
}

pub fn call_do(
    configuration: &Configuration,
    api_path: String,
    method: Method,
    body: Option<String>,
) -> Result<Response, ReqError> {
    let url = format!("https://api.digitalocean.com/v2/{}", api_path);
    let client = reqwest::blocking::Client::new();
    if let Some(b) = body {
        println! {"{}", b};
        client
            .request(method, &url)
            .bearer_auth(configuration.do_token.as_str())
            .header("Content-Type", "application/json")
            .body(b)
            .send()
    } else {
        client
            .request(method, &url)
            .bearer_auth(configuration.do_token.as_str())
            .header("Content-Type", "application/json")
            .send()
    }
}
