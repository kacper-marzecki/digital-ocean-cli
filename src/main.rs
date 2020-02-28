#[macro_use]
extern crate envconfig_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
mod api;
mod config;
mod core;
mod error;
mod util;
extern crate rustyline;

use config::Configuration;
use envconfig::Envconfig;
use error::AppError;
use std::string::String;

fn show_help() -> Result<(), AppError> {
    println!(
        "Possible commands: 
        list -> lists currently running droplets  
        help -> shows this help text
        delete XXX -> deletes droplet with name == XXX 
        create  -> enters new droplet creation 'wizard'
        "
    );
    Ok(())
}

fn match_command(command: String, configuration: &Configuration) -> Result<(), AppError> {
    let mut tokens = command.as_str().split_whitespace();
    let result = match &tokens.next() {
        Some("list") => core::show_droplets(&configuration),
        Some("help") => show_help(),
        Some("delete") => core::delete_droplet(&configuration, tokens),
        Some("create") => core::create_droplet(&configuration),
        _ => Err(AppError::CommandError("invalid command".to_string())),
    };
    if let Err(err) = result {
        match err {
            AppError::CommandError(reason) => Ok(eprint!("Command error: {:?}", reason)),
            AppError::NetworkingError(reason) => Ok(eprint!("Networking error: {:?}", reason)),
            AppError::LogicError(reason) => Ok(eprint!("Logic error: {:?}", reason)),
            AppError::InteruptionError => Err(AppError::InteruptionError),
            AppError::InputError => Ok(eprint!("Unprocessable input")),
        }
    } else {
        Ok(())
    }
}

fn main() {
    let configuration = match Configuration::init() {
        Ok(config) => config,
        Err(err) => {
            eprint!("cannot initialize application {:?}", err);
            return;
        }
    };
    println!("MIESZKANIA DEPLOYMENT TOOL INITIALIZING!");

    show_help();
    loop {
        match util::use_input(None, |line| match_command(line, &configuration)) {
            Err(_) => break,
            _ => (),
        };
    }
}
