#[macro_use]
extern crate envconfig_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
mod api;
mod config;
mod error;
mod util;
extern crate rustyline;

use api::{Droplet, Droplets};
use config::{Configuration, DropletConfiguration};
use envconfig::Envconfig;
use error::AppError;
use reqwest::blocking::Response;
use reqwest::{Error as ReqError, Method};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::string::String;

fn list_droplets(configuration: &Configuration) -> Result<Droplets, AppError> {
    let droplets = api::call_do(&configuration, "droplets".to_string(), Method::GET, None)?
        .json::<Droplets>()?;
    Ok(droplets)
}

fn show_droplets(configuration: &Configuration) -> Result<(), AppError> {
    let droplets = list_droplets(&configuration)?;
    println!("{:#?}", droplets.droplets);
    Ok(())
}

fn find_droplet_id_by_name(droplets: Droplets, name: &str) -> Option<u32> {
    droplets.droplets.into_iter().find_map(|droplet| {
        if droplet.name == name {
            Some(droplet.id)
        } else {
            Option::None
        }
    })
}

fn delete_droplet(
    configuration: &Configuration,
    mut arguments: std::str::SplitWhitespace,
) -> Result<(), AppError> {
    let droplet_name = if let Some(name) = arguments.next() {
        Ok(name)
    } else {
        Err(AppError::CommandError(
            "Please provide droplet name to delete".to_string(),
        ))
    }?;
    let droplets = list_droplets(&configuration)?;
    if let Some(droplet_id) = find_droplet_id_by_name(droplets, &droplet_name) {
        let response = api::call_do(
            &configuration,
            format!("droplets/{}", droplet_id),
            Method::DELETE,
            None,
        )?;
        Ok(())
    } else {
        Err(AppError::LogicError(format!(
            "droplet {} doesnt currently exist",
            droplet_name
        )))
    }
}

fn create_preset_droplet(configuration: &Configuration) -> Result<(), AppError> {
    let droplet_config = DropletConfiguration {
        name: get_input("Enter droplet name")?,
        region: "lon1".to_string(),
        size: "s-1vcpu-1gb".to_string(),
        image: "ubuntu-16-04-x64".to_string(),
        backups: false,
        ipv6: false,
        private_networking: false,
        tags: get_input("Enter tags")?
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    };
    let serialised = serde_json::to_string(&droplet_config)?;
    let response = api::call_do(
        &configuration,
        "droplets".to_string(),
        Method::POST,
        Some(serialised),
    )?;
    println!("{:?}", response);

    Ok(())
}

fn create_custom_droplet(configuration: &Configuration) -> Result<(), AppError> {
    let droplet_config = DropletConfiguration {
        name: get_input("Enter droplet name")?,
        region: get_input("Enter droplet name")?,
        size: get_input("Enter droplet name")?,
        image: get_input("Enter droplet name")?,
        backups: false,
        ipv6: false,
        private_networking: false,
        tags: vec![get_input("Enter droplet name")?],
    };
    let serialised = serde_json::to_string(&droplet_config)?;
    let response = api::call_do(
        &configuration,
        "droplets".to_string(),
        Method::POST,
        Some(serialised),
    )?;
    println!("{:?}", response);
    Ok(())
}

fn create_droplet(configuration: &Configuration) -> Result<(), AppError> {
    // get if from preset or custom
    let is_custom = get_bool_input("Custom Droplet ? N/y")?;
    if is_custom {
        create_custom_droplet(&configuration)
    } else {
        create_preset_droplet(&configuration)
    }
}

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
        Some("list") => show_droplets(&configuration),
        Some("help") => show_help(),
        Some("delete") => delete_droplet(&configuration, tokens),
        Some("create") => create_droplet(&configuration),
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

fn get_bool_input(prompt: &str) -> Result<bool, AppError> {
    use_input(
        Some(format!("{} y/n", prompt).as_str()),
        |line| match line.as_str() {
            "y" => Ok(true),
            "n" => Ok(false),
            _ => Err(AppError::InputError),
        },
    )
}

fn get_input(prompt: &str) -> Result<String, AppError> {
    use_input(Some(prompt), |line| Ok(line))
}

fn use_input<F, R>(prompt: Option<&str>, mut f: F) -> Result<R, AppError>
where
    F: FnMut(String) -> Result<R, AppError>,
{
    let mut rl = Editor::<()>::new();
    if let Some(text) = prompt {
        println!("{}", text);
    }
    let readline = rl.readline(">> ");
    match readline {
        Ok(line) => f(line),
        Err(ReadlineError::Interrupted) => {
            println!("CTRL-C");
            Err(AppError::InteruptionError)
        }
        Err(ReadlineError::Eof) => {
            println!("CTRL-D");
            Err(AppError::InteruptionError)
        }
        Err(err) => {
            println!("Error: {:?}", err);
            Err(AppError::InteruptionError)
        }
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
        match use_input(None, |line| match_command(line, &configuration)) {
            Err(_) => break,
            _ => (),
        };
    }
}
