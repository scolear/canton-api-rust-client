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
pub struct EventOneOf2 {
    #[serde(rename = "ExercisedEvent")]
    pub exercised_event: Box<models::ExercisedEvent>,
}

impl EventOneOf2 {
    pub fn new(exercised_event: models::ExercisedEvent) -> EventOneOf2 {
        EventOneOf2 {
            exercised_event: Box::new(exercised_event),
        }
    }
}

