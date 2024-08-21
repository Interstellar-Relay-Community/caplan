// SPDX-FileCopyrightText: 2024 Project Caplan contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::{Deserialize, Serialize};

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
