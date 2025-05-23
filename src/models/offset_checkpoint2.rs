/*
 * JSON Ledger API HTTP endpoints
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.3.0-SNAPSHOT
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// OffsetCheckpoint2 : OffsetCheckpoints may be used to:  - detect time out of commands. - provide an offset which can be used to restart consumption.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OffsetCheckpoint2 {
    #[serde(rename = "value")]
    pub value: Box<models::OffsetCheckpoint1>,
}

impl OffsetCheckpoint2 {
    /// OffsetCheckpoints may be used to:  - detect time out of commands. - provide an offset which can be used to restart consumption.
    pub fn new(value: models::OffsetCheckpoint1) -> OffsetCheckpoint2 {
        OffsetCheckpoint2 {
            value: Box::new(value),
        }
    }
}

