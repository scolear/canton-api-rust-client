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
pub struct ParticipantPermissionOneOf {
    #[serde(rename = "PARTICIPANT_PERMISSION_CONFIRMATION")]
    pub participant_permission_confirmation: serde_json::Value,
}

impl ParticipantPermissionOneOf {
    pub fn new(participant_permission_confirmation: serde_json::Value) -> ParticipantPermissionOneOf {
        ParticipantPermissionOneOf {
            participant_permission_confirmation,
        }
    }
}

