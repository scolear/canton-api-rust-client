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

/// UpdatePartyDetailsRequest : Required authorization: ``HasRight(ParticipantAdmin) OR IsAuthenticatedIdentityProviderAdmin(party_details.identity_provider_id)``
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatePartyDetailsRequest {
    #[serde(rename = "partyDetails", skip_serializing_if = "Option::is_none")]
    pub party_details: Option<Box<models::PartyDetails>>,
    #[serde(rename = "updateMask", skip_serializing_if = "Option::is_none")]
    pub update_mask: Option<Box<models::FieldMask>>,
}

impl UpdatePartyDetailsRequest {
    /// Required authorization: ``HasRight(ParticipantAdmin) OR IsAuthenticatedIdentityProviderAdmin(party_details.identity_provider_id)``
    pub fn new() -> UpdatePartyDetailsRequest {
        UpdatePartyDetailsRequest {
            party_details: None,
            update_mask: None,
        }
    }
}

