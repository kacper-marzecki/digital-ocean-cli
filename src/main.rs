#[macro_use]
extern crate envconfig_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
mod config;
mod api;
mod error;
mod util;
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

fn find_droplet_id_by_name(droplets: Droplets, name: &str) -> Option<u32> {
    droplets.droplets.into_iter().find_map(|droplet|{ 
        if droplet.name == name {
            Some(droplet.id)
        }  else {
            Option::None
        }})
}

fn delete_droplet(configuration: &Configuration, mut arguments: std::str::SplitWhitespace) -> Result<(), AppError> {
    let droplet_name = if let Some(name) = arguments.next() {
        Ok(name)
    } else {
        Err(AppError::CommandError("Please provide droplet name to delete".to_string()))
    }?;
    let droplets = list_droplets(&configuration)?;
    if let Some(droplet_id) = find_droplet_id_by_name(droplets, &droplet_name)  {
        api::call_do(&configuration, format!("droplet/{}", droplet_id), Method::DELETE)?;
        Ok(())
    } else {
        Err(AppError::LogicError(format!("droplet {} doesnt currently exist", droplet_name)))
    }
}

fn show_help() -> Result<(), AppError>{
    println!("Possible commands: 
        list -> lists currently running droplets  
        help -> shows this help text
        delete XXX -> deletes droplet with name == XXX 
        ");
    Ok(())
}

fn match_command(command: String, configuration: &Configuration ) -> Result<(), AppError>{
    let mut tokens = command.as_str().split_whitespace();
    let result = match &tokens.next() {
        Some("list") => show_droplets(&configuration),   
        Some("help") => show_help(),
        Some("delete") => delete_droplet(&configuration, tokens),
        _ => Err(AppError::CommandError("invalid command".to_string()))
    };
    if let Err(err) = result {
        match err {
            AppError::CommandError(reason) => eprint!("Command error: {:?}", reason),
            AppError::NetworkingError(reason) => eprint!("Networking error: {:?}", reason),
            AppError::LogicError(reason) => eprint!("Logic error: {:?}", reason),
            AppError::InteruptionError => eprint!("Interrupted"),
        }
    };
    Ok(())
 }

fn get_input() -> Result<String, AppError> {
    use_input(|line| Ok(line))
}

fn use_input<F, R>(mut f: F) -> Result<R, AppError> 
    where 
        F: FnMut(String) -> Result<R, AppError>
        {
        let mut rl = Editor::<()>::new();
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
               f(line)
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                Err(AppError::InteruptionError)
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                Err(AppError::InteruptionError)
            },
            Err(err) => {
                println!("Error: {:?}", err);
                Err(AppError::InteruptionError)
            }
        }
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
    loop {
        use_input(|line|  match_command(line, &configuration));
    }
}
