use crate::api;
use crate::api::{Droplet, DropletNetwork, DropletNetworks, Droplets};
use crate::config::{Configuration, DropletConfiguration};
use crate::error::AppError;
use crate::util;
use reqwest::{Error as ReqError, Method};

pub fn list_droplets(configuration: &Configuration) -> Result<Droplets, AppError> {
    let droplets = api::call_do(&configuration, "droplets".to_string(), Method::GET, None)?
        .json::<Droplets>()?;
    Ok(droplets)
}

pub fn show_droplets(configuration: &Configuration) -> Result<(), AppError> {
    let droplets = list_droplets(&configuration)?;
    println!("{:#?}", droplets.droplets);
    Ok(())
}

pub fn create_droplet(configuration: &Configuration) -> Result<(), AppError> {
    let is_custom = util::get_bool_input("Custom Droplet ?")?;
    if is_custom {
        create_custom_droplet(&configuration)
    } else {
        create_preset_droplet(&configuration)
    }
}

pub fn delete_droplet(
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
    if let Some(droplet) = find_droplet_id_by_name(droplets, &droplet_name) {
        let response = api::call_do(
            &configuration,
            format!("droplets/{}", droplet.id),
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

fn find_droplet_id_by_name(droplets: Droplets, name: &str) -> Option<Droplet> {
    droplets.droplets.into_iter().find_map(|droplet| {
        if droplet.name == name {
            Some(droplet)
        } else {
            Option::None
        }
    })
}

fn create_preset_droplet(configuration: &Configuration) -> Result<(), AppError> {
    let droplet_config = DropletConfiguration {
        name: util::get_input("Enter droplet name")?,
        region: "lon1".to_string(),
        size: "s-1vcpu-1gb".to_string(),
        image: "ubuntu-16-04-x64".to_string(),
        backups: false,
        ipv6: false,
        private_networking: false,
        tags: util::get_input("Enter tags")?
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    };
    let serialised = serde_json::to_string(&droplet_config)?;
    api::call_do(
        &configuration,
        "droplets".to_string(),
        Method::POST,
        Some(serialised),
    )?;
    Ok(())
}

fn create_custom_droplet(configuration: &Configuration) -> Result<(), AppError> {
    let droplet_config = DropletConfiguration {
        name: util::get_input("Enter droplet name")?,
        region: util::get_input("Enter droplet region")?,
        size: util::get_input("Enter droplet size")?,
        image: util::get_input("Enter droplet image")?,
        backups: false,
        ipv6: false,
        private_networking: true,
        tags: util::get_input("Enter tags")?
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    };
    let serialised = serde_json::to_string(&droplet_config)?;
    println!("Creating droplet: {:#?}", droplet_config);
    api::call_do(
        &configuration,
        "droplets".to_string(),
        Method::POST,
        Some(serialised),
    )?;
    Ok(())
}

fn ssh_authenticate(
    configuration: &Configuration,
    mut arguments: std::str::SplitWhitespace,
) -> Result<ssh2::Session, AppError> {
    use ssh2::Session;
    use std::net::TcpStream;
    let droplet_name = if let Some(name) = arguments.next() {
        Ok(name)
    } else {
        Err(AppError::CommandError(
            "Please provide droplet name to connect to".to_string(),
        ))
    }?;
    let droplets = list_droplets(&configuration)?;
    let droplet = if let Some(droplet) = find_droplet_id_by_name(droplets, droplet_name) {
        Ok(droplet)
    } else {
        Err(AppError::LogicError(format!(
            "droplet {} doesnt currently exist",
            droplet_name
        )))
    }?;
    let network = if let Some(network) = droplet.networks.v4.get(0) {
        Ok(network)
    } else {
        Err(AppError::LogicError(format!(
            "droplet {} doesnt have a network",
            droplet_name
        )))
    }?;
    let tcp = TcpStream::connect(format!("{}:22", network.ip_address))?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;
    session.userauth_agent("root")?;
    if session.authenticated() {
        Ok(session)
    } else {
        Err(AppError::LogicError("Cannot authenticate".to_string()))
    }
}
