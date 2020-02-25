extern crate envconfig;

use envconfig::Envconfig;

#[allow(dead_code)]
#[derive(Envconfig, Debug, Clone)]
pub struct Configuration {
    #[envconfig(from = "DB_PASSWORD")]
    pub database_password: Option<String>,
    #[envconfig(from = "DB_USER")]
    pub database_user: Option<String>,
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: Option<String>,
    #[envconfig(from = "DOTOKEN")]
    pub do_token: String,
    #[envconfig(from = "FRONTEND_ORIGIN")]
    pub frontend_url: Option<String>,
}

enum DropletImage {
    Ubuntu,
    CoreOS,
}

impl DropletImage {
    pub fn slug(&self) -> String {
        match self {
            Ubuntu => "ubuntu-18-04-x64",
            CoreOS => "coreos-stable",
        }
        .to_string()
    }
}

enum DropletSize {
    S1Cpu1Gb,
}

impl DropletSize {
    fn slug(&self) -> String {
        match self {
            S1Cpu1Gb => "s-1vcpu-1gb",
        }
        .to_string()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize)]
pub struct DropletConfiguration {
    pub name: String,
    pub region: String,
    pub size: String,
    pub image: String,
    pub backups: bool,
    pub ipv6: bool,
    pub private_networking: bool,
    pub tags: Vec<String>,
}
