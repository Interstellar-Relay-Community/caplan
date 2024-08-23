// SPDX-FileCopyrightText: 2024 Project Caplan contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::types::AppState;
use axum::Router;

pub async fn get_service() -> Router<AppState> {
    Router::new()
}
