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
pub struct Update1OneOf1 {
    #[serde(rename = "Reassignment")]
    pub reassignment: Box<models::Reassignment1>,
}

impl Update1OneOf1 {
    pub fn new(reassignment: models::Reassignment1) -> Update1OneOf1 {
        Update1OneOf1 {
            reassignment: Box::new(reassignment),
        }
    }
}

