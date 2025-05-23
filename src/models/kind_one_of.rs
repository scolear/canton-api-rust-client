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
pub struct KindOneOf {
    #[serde(rename = "CanActAs")]
    pub can_act_as: Box<models::CanActAs>,
}

impl KindOneOf {
    pub fn new(can_act_as: models::CanActAs) -> KindOneOf {
        KindOneOf {
            can_act_as: Box::new(can_act_as),
        }
    }
}

