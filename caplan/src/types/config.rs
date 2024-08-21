// SPDX-FileCopyrightText: 2024 Project Caplan contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Database {
    // TBD
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActivityPub {
    // TBD
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub activity_pub: ActivityPub,
}

pub(crate) fn parse_toml(tomlstr: &str) -> Config {
    let cfg: Config = toml::from_str(tomlstr).expect("Invalid config file");

    // TODO: Sanitize the value?

    cfg
}

pub(crate) async fn read_config_file(cfgpath: &str) -> Config {
    let cfg_str = fs::read_to_string(cfgpath)
        .await
        .expect("failed to read config file");

    // TODO: Postprocess parsed config file
    parse_toml(&cfg_str)
}
