/*
 * Created on Tue Dec 07 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};

/// OAuth credential data for internal talk service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalkAuthCredential {
    pub access_token: String,
    pub refresh_token: String,
}
