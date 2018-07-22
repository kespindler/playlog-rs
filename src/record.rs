extern crate serde_json;

use log::Level;

use serde_json::Value;

pub type Args = Value;


pub struct Record {
    pub name: String,
    pub level: Level,
    pub message: String,
    pub data: Args,
}
