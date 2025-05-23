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
pub struct TraceContext {
    /// https://www.w3.org/TR/trace-context/
    #[serde(rename = "traceparent", skip_serializing_if = "Option::is_none")]
    pub traceparent: Option<String>,
    /// 
    #[serde(rename = "tracestate", skip_serializing_if = "Option::is_none")]
    pub tracestate: Option<String>,
}

impl TraceContext {
    pub fn new() -> TraceContext {
        TraceContext {
            traceparent: None,
            tracestate: None,
        }
    }
}

