/*
 * Addresses
 *
 * The Addresses API validates and corrects address information, eliminating errors, improving package delivery service and package pricing. This suite of APIs provides different utilities for addressing components. The ZIP Code&#8482; lookup finds valid ZIP Code&#8482; for a City and State.  The City/State lookup provides the valid cities and states for a provided ZIP Code&#8482;.  The Address Standardization API validates and standardizes USPS&#174; domestic addresses, city and state names, and ZIP Code&#8482; in accordance with USPS&#174; addressing standards.  The USPS&#174; address standard includes the ZIP + 4&#174;, signifying a USPS&#174; delivery point, given a street address, a city and a state. 
 *
 * The version of the OpenAPI document: 3.0.2
 * Contact: APISupport@usps.gov
 * Generated by: https://openapi-generator.tech
 */



#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    // TODO: take an oauth2 token source, similar to the go one
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}


impl Configuration {
    pub fn new() -> Configuration {
        Configuration::default()
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "https://api.usps.com/addresses/v3".to_owned(),
            user_agent: Some("OpenAPI-Generator/3.0.2/rust".to_owned()),
            client: reqwest::Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,

        }
    }
}
