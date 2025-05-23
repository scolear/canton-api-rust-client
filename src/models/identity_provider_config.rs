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
pub struct IdentityProviderConfig {
    /// The identity provider identifier Must be a valid LedgerString (as describe in ``value.proto``). Required
    #[serde(rename = "identityProviderId")]
    pub identity_provider_id: String,
    /// When set, the callers using JWT tokens issued by this identity provider are denied all access to the Ledger API. Optional, Modifiable
    #[serde(rename = "isDeactivated")]
    pub is_deactivated: bool,
    /// Specifies the issuer of the JWT token. The issuer value is a case sensitive URL using the https scheme that contains scheme, host, and optionally, port number and path components and no query or fragment components. Required Modifiable
    #[serde(rename = "issuer")]
    pub issuer: String,
    /// The JWKS (JSON Web Key Set) URL. The Ledger API uses JWKs (JSON Web Keys) from the provided URL to verify that the JWT has been signed with the loaded JWK. Only RS256 (RSA Signature with SHA-256) signing algorithm is supported. Required Modifiable
    #[serde(rename = "jwksUrl")]
    pub jwks_url: String,
    /// Specifies the audience of the JWT token. When set, the callers using JWT tokens issued by this identity provider are allowed to get an access only if the \"aud\" claim includes the string specified here Optional, Modifiable
    #[serde(rename = "audience")]
    pub audience: String,
}

impl IdentityProviderConfig {
    pub fn new(identity_provider_id: String, is_deactivated: bool, issuer: String, jwks_url: String, audience: String) -> IdentityProviderConfig {
        IdentityProviderConfig {
            identity_provider_id,
            is_deactivated,
            issuer,
            jwks_url,
            audience,
        }
    }
}

