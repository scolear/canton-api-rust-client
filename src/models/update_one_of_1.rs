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
pub struct UpdateOneOf1 {
    #[serde(rename = "Reassignment")]
    pub reassignment: Box<models::Reassignment>,
}

impl UpdateOneOf1 {
    pub fn new(reassignment: models::Reassignment) -> UpdateOneOf1 {
        UpdateOneOf1 {
            reassignment: Box::new(reassignment),
        }
    }
}

