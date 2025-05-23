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
pub struct JsCreated {
    #[serde(rename = "createdEvent")]
    pub created_event: Box<models::CreatedEvent>,
    /// The synchronizer which sequenced the creation of the contract Required
    #[serde(rename = "synchronizerId")]
    pub synchronizer_id: String,
}

impl JsCreated {
    pub fn new(created_event: models::CreatedEvent, synchronizer_id: String) -> JsCreated {
        JsCreated {
            created_event: Box::new(created_event),
            synchronizer_id,
        }
    }
}

