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

/// User :  Users and rights /////////////////  Users are used to dynamically manage the rights given to Daml applications.  They are stored and managed per participant node.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// The user identifier, which must be a non-empty string of at most 128 characters that are either alphanumeric ASCII characters or one of the symbols \"@^$.!`-#+'~_|:\". Required
    #[serde(rename = "id")]
    pub id: String,
    /// The primary party as which this user reads and acts by default on the ledger *provided* it has the corresponding ``CanReadAs(primary_party)`` or ``CanActAs(primary_party)`` rights. Ledger API clients SHOULD set this field to a non-empty value for all users to enable the users to act on the ledger using their own Daml party. Users for participant administrators MAY have an associated primary party. Optional, Modifiable
    #[serde(rename = "primaryParty")]
    pub primary_party: String,
    /// When set, then the user is denied all access to the Ledger API. Otherwise, the user has access to the Ledger API as per the user's rights. Optional, Modifiable
    #[serde(rename = "isDeactivated")]
    pub is_deactivated: bool,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::ObjectMeta>>,
    /// The ID of the identity provider configured by ``Identity Provider Config`` Optional, if not set, assume the user is managed by the default identity provider.
    #[serde(rename = "identityProviderId")]
    pub identity_provider_id: String,
}

impl User {
    ///  Users and rights /////////////////  Users are used to dynamically manage the rights given to Daml applications.  They are stored and managed per participant node.
    pub fn new(id: String, primary_party: String, is_deactivated: bool, identity_provider_id: String) -> User {
        User {
            id,
            primary_party,
            is_deactivated,
            metadata: None,
            identity_provider_id,
        }
    }
}

