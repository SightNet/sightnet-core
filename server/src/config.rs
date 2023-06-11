use lazy_static::lazy_static;
use serde::Deserialize;
use std::sync::Mutex;
use config::Config;

#[derive(Debug, Deserialize)]
pub struct Cfg {
    pub directory: String,
}

lazy_static! {
    pub static ref CFG: Mutex<Cfg> = Mutex::new(Config::builder()
        .add_source(config::File::with_name("Config.toml"))
        .build()
        .unwrap()
        .try_deserialize::<Cfg>()
        .unwrap());
}
