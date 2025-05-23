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

/// Command : A command can either create a new contract or exercise a choice on an existing contract.
/// A command can either create a new contract or exercise a choice on an existing contract.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Command {
    CommandOneOf(Box<models::CommandOneOf>),
    CommandOneOf1(Box<models::CommandOneOf1>),
    CommandOneOf2(Box<models::CommandOneOf2>),
    CommandOneOf3(Box<models::CommandOneOf3>),
}

impl Default for Command {
    fn default() -> Self {
        Self::CommandOneOf(Default::default())
    }
}

