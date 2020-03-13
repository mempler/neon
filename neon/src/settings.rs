use std::env;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HttpSettings {
    pub address: Option<String>,
    pub port:    Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub connection_url: Option<String>,
    pub pool_size:      Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub http:     HttpSettings,
    pub database: DatabaseSettings,
}

lazy_static::lazy_static! {
   static ref SETTINGS: Settings = {
      let settings_path = match env::var("NEON_SETTINGS") {
         Ok(settings_path) => settings_path,
         _ => "./settings.toml".into(),
      };

      match std::fs::read_to_string(settings_path) {
         Ok(settings) => {
            match toml::from_str(&settings) {
               Ok(settings) => {
                  log::info!("Successfully parsed the settings file.");
                  settings
               },
               Err(error) => {
                  log::error!("Couldn't parse the settings file: {}, exiting.", error);
                  std::process::exit(1)
               }
            }
         },
         Err(error) => {
            log::error!("Couldn't read the settings file: {}, exiting.", error);
            std::process::exit(1)
         }
      }
   };
}

#[inline(always)]
pub fn get_settings<'a>() -> &'a Settings {
    &*SETTINGS
}
