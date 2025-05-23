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
pub struct KindOneOf4 {
    #[serde(rename = "IdentityProviderAdmin")]
    pub identity_provider_admin: Box<models::IdentityProviderAdmin>,
}

impl KindOneOf4 {
    pub fn new(identity_provider_admin: models::IdentityProviderAdmin) -> KindOneOf4 {
        KindOneOf4 {
            identity_provider_admin: Box::new(identity_provider_admin),
        }
    }
}

