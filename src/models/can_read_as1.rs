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
pub struct CanReadAs1 {
    #[serde(rename = "party")]
    pub party: String,
}

impl CanReadAs1 {
    pub fn new(party: String) -> CanReadAs1 {
        CanReadAs1 {
            party,
        }
    }
}

