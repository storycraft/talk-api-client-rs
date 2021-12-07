/*
 * Created on Tue Dec 07 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalkStatusResponse<T> {
    pub status: i32,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<T>
}
