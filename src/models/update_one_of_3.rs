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
pub struct UpdateOneOf3 {
    #[serde(rename = "Transaction")]
    pub transaction: Box<models::Transaction>,
}

impl UpdateOneOf3 {
    pub fn new(transaction: models::Transaction) -> UpdateOneOf3 {
        UpdateOneOf3 {
            transaction: Box::new(transaction),
        }
    }
}

