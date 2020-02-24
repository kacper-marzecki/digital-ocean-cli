

extern crate serde_derive;

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