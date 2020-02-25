#[macro_use]
extern crate envconfig_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
mod config;
mod api;
mod error;

extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::string::String;
use envconfig::Envconfig;
use config::Configuration;
use reqwest::blocking::Response;
use reqwest::{Error as ReqError, Method};
use api::{Droplets, Droplet};
use error::AppError;


fn list_droplets(configuration: &Configuration)-> Result<Droplets, AppError>  {
    let droplets = api::call_do(&configuration, "droplets".to_string(), Method::GET )?    
    .json::<Droplets>()?;
    Ok(droplets)
}

fn show_droplets(configuration: &Configuration) -> Result<(), AppError> {
    let droplets = list_droplets(&configuration)?;
    println!("{:#?}", droplets.droplets);
    Ok(())
}

fn show_help() -> Result<(), AppError>{
    println!("Possible commands: 
        list -> lists currently running droplets  
        help -> shows this help text
        ");
    Ok(())
}

fn match_command(command: String, configuration: &Configuration ){
    let result = match command.as_str() {
        "list" => show_droplets(&configuration),   
        "help" => show_help(),
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

    show_help();
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
