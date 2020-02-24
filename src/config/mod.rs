
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