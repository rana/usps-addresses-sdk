/*
 * Addresses
 *
 * The Addresses API validates and corrects address information, eliminating errors, improving package delivery service and package pricing. This suite of APIs provides different utilities for addressing components. The ZIP Code&#8482; lookup finds valid ZIP Code&#8482; for a City and State.  The City/State lookup provides the valid cities and states for a provided ZIP Code&#8482;.  The Address Standardization API validates and standardizes USPS&#174; domestic addresses, city and state names, and ZIP Code&#8482; in accordance with USPS&#174; addressing standards.  The USPS&#174; address standard includes the ZIP + 4&#174;, signifying a USPS&#174; delivery point, given a street address, a city and a state. 
 *
 * The version of the OpenAPI document: 3.0.2
 * Contact: APISupport@usps.gov
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AddressResponse : Standardizes street addresses including city and street abbreviations, and provides missing information such as ZIP Code&#8482; and ZIP + 4&#174;.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressResponse {
    /// This is the business or firm name at the address.
    #[serde(rename = "firm", skip_serializing_if = "Option::is_none")]
    pub firm: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Box<models::DomesticAddress>>,
    #[serde(rename = "additionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<Box<models::AddressAdditionalInfo>>,
    /// The corrections made to the requested address.
    #[serde(rename = "corrections", skip_serializing_if = "Option::is_none")]
    pub corrections: Option<Vec<models::AddressCorrectionsInner>>,
    /// Matches made to the requested address.
    #[serde(rename = "matches", skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<models::AddressMatchesInner>>,
}

impl AddressResponse {
    /// Standardizes street addresses including city and street abbreviations, and provides missing information such as ZIP Code&#8482; and ZIP + 4&#174;.
    pub fn new() -> AddressResponse {
        AddressResponse {
            firm: None,
            address: None,
            additional_info: None,
            corrections: None,
            matches: None,
        }
    }
}

