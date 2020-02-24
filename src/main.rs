#[macro_use]
extern crate envconfig_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
mod config;
mod api;

extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::string::String;
use envconfig::Envconfig;
use config::Configuration;
use reqwest::blocking::Response;
// use reqwest::Response;
use reqwest::{Error as ReqError};
use api::{Droplets, Droplet};

enum AppError {
    CommandError(String),
    NetworkingError(String)
}

impl std::convert::From<ReqError> for AppError {
    fn from(err: ReqError)-> Self {
        AppError::NetworkingError(format!("{}", err))
    }
}


fn make_api_call(configuration: &Configuration, api_path: String) -> Result<Response, ReqError> {
    let url = format!("https://api.digitalocean.com/v2/{}", api_path);
    let client = reqwest::blocking::Client::new();
    let response = client.get(&url)
    .bearer_auth(configuration.do_token.as_str())
    .header("Content-Type", "application/json")
    .send();
    response
}

fn list_droplets(configuration: &Configuration)-> Result<Droplets, AppError>  {
    let droplets = make_api_call(&configuration, "droplets".to_string())?    
    .json::<Droplets>()?;
    println!("{:#?}", droplets.droplets);
    Ok(droplets)
}

fn match_command(command: String, configuration: &Configuration ){
    let result = match command.as_str() {
        "list" => list_droplets(&configuration),    
        _ => Err(AppError::CommandError("invalid command".to_string()))
    };
    if let Err(err) = result {
        match err {
            AppError::CommandError(reason) => eprint!("Command error: {:?}", reason),
            AppError::NetworkingError(reason) => eprint!("Networking error: {:?}", reason),
        }
    };
 }

fn main() {

    let configuration = match Configuration::init(){
            Ok(config) => config,
            Err(err) => {
                eprint!("cannot initialize application {:?}", err); 
                return ;
            }
    };
    println!("MIESZKANIA DEPLOYMENT TOOL INITIALIZING!");

    println!("Possible commands: 
        list -> lists currently running droplets  
        ");
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match_command(line, &configuration);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}
