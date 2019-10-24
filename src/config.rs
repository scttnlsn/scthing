use serde_derive::Deserialize;
use std::fs;
use std::io;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub devices: Devices,
    pub menus: Vec<Menu>,
}

#[derive(Deserialize, Debug)]
pub struct Devices {
    pub framebuffer: String,
    pub encoder: String,
    pub button: String,
}

#[derive(Deserialize, Debug)]
pub struct Menu {
    pub name: String,
    pub patches: Vec<Patch>,
}

#[derive(Deserialize, Debug)]
pub struct Patch {
    pub name: String,
    pub params: Vec<Param>,
}

#[derive(Deserialize, Debug)]
pub struct Param {
    pub name: String,
    pub value: f32,
    pub step: f32,
    pub min: f32,
    pub max: f32,
}

pub fn parse(path: &str) -> Result<Config, io::Error> {
    let config_toml = fs::read_to_string(path)?;

    let config: Config = toml::from_str(&config_toml)?;
    Ok(config)
}
