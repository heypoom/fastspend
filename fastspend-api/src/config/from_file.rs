use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{Account, Config};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConfigFile {
    pub categories: HashMap<String, ConfigDefinition>,
    pub payees: HashMap<String, ConfigDefinition>,
    pub accounts: Vec<Account>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConfigDefinition {
    pub id: String,
    pub title: String,
}

pub const STATIC_CONFIG_JSON: &str = include_str!("config.example.json");

lazy_static! {
    pub static ref STATIC_CONFIG: Config =
        parse_config_file(serde_json::from_str::<ConfigFile>(&STATIC_CONFIG_JSON).unwrap());
}

pub fn parse_config_file(file: ConfigFile) -> Config {
    let mut config = Config::new();

    for (alias, data) in file.payees {
        config.register_payee(alias, Some(data.id), Some(data.title));
    }

    for (alias, data) in file.categories {
        config.register_category(alias, data.id, Some(data.title))
    }

    for account in file.accounts {
        config.accounts.push(account)
    }

    config
}
