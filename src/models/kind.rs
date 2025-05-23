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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Kind {
    KindOneOf(Box<models::KindOneOf>),
    KindOneOf1(Box<models::KindOneOf1>),
    KindOneOf2(Box<models::KindOneOf2>),
    KindOneOf3(Box<models::KindOneOf3>),
    KindOneOf4(Box<models::KindOneOf4>),
    KindOneOf5(Box<models::KindOneOf5>),
}

impl Default for Kind {
    fn default() -> Self {
        Self::KindOneOf(Default::default())
    }
}

