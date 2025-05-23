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

/// Reassignment : Complete view of an on-ledger reassignment.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reassignment {
    #[serde(rename = "value")]
    pub value: Box<models::JsReassignment>,
}

impl Reassignment {
    /// Complete view of an on-ledger reassignment.
    pub fn new(value: models::JsReassignment) -> Reassignment {
        Reassignment {
            value: Box::new(value),
        }
    }
}

