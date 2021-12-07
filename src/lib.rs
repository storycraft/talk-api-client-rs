/*
 * Created on Mon Dec 06 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

pub mod agent;
pub mod auth;
pub mod credential;
pub mod response;

use std::{ops::Deref, str::FromStr};

use reqwest::Url;

/// Wrapped [reqwest::Url]
#[derive(Debug, Clone)]
pub struct ApiURL(Url);

impl ApiURL {
    pub fn new(scheme: &str, host: &str) -> Option<Self> {
        Some(Self(Url::from_str(&format!("{}://{}", scheme, host)).ok()?))
    }
}

impl Deref for ApiURL {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
