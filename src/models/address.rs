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

/// Address : Address fields standard to all locations.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Address {
    /// The number of a building along with the name of the road or street on which it is located.
    #[serde(rename = "streetAddress", skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    /// This is the abbreviation of street address line for the address.
    #[serde(rename = "streetAddressAbbreviation", skip_serializing_if = "Option::is_none")]
    pub street_address_abbreviation: Option<String>,
    /// The secondary unit designator, such as apartment(APT) or suite(STE) number, defining the exact location of the address within a building.  For more information please see [Postal Explorer](https://pe.usps.com/text/pub28/28c2_003.htm).
    #[serde(rename = "secondaryAddress", skip_serializing_if = "Option::is_none")]
    pub secondary_address: Option<String>,
    /// This is the abbreviation of city name for the address.
    #[serde(rename = "cityAbbreviation", skip_serializing_if = "Option::is_none")]
    pub city_abbreviation: Option<String>,
}

impl Address {
    /// Address fields standard to all locations.
    pub fn new() -> Address {
        Address {
            street_address: None,
            street_address_abbreviation: None,
            secondary_address: None,
            city_abbreviation: None,
        }
    }
}

