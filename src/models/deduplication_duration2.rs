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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeduplicationDuration2 {
    #[serde(rename = "value")]
    pub value: Box<models::Duration>,
}

impl DeduplicationDuration2 {
    pub fn new(value: models::Duration) -> DeduplicationDuration2 {
        DeduplicationDuration2 {
            value: Box::new(value),
        }
    }
}

