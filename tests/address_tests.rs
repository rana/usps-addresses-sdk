use std::{env, io};
use usps_addresses_sdk::apis::configuration::Configuration as AdrConfiguration;
use usps_addresses_sdk::apis::resources_api::get_address;
use usps_oauth_sdk::apis::configuration::Configuration;
use usps_oauth_sdk::apis::default_api::post_token;
use usps_oauth_sdk::models::InlineResponse2001::ProviderAccessTokenResponse;

async fn fetch_address() -> Result<(), Box<dyn std::error::Error>> {
    let client_key = &env::var("USPS_CLIENT_KEY")?;
    let client_secret = &env::var("USPS_CLIENT_SECRET")?;

    // Get a token.
    let cfg = Configuration::new();
    match post_token(
        &cfg,
        Some("client_credentials"),
        None,
        Some(client_key),
        Some(client_secret),
        None,
        None,
        None,
    )
    .await
    {
        Ok(inline_res) => {
            if let ProviderAccessTokenResponse(res) = inline_res {
                let mut cfg_adr = AdrConfiguration::new();
                cfg_adr.oauth_access_token = Some(res.access_token);
                let street_address = "1600 Amphitheatre Pkwy";
                let city = "Mountain View";
                let state = "CA";
                let zip5 = "94043";
                // Get a modified address.
                match get_address(
                    &cfg_adr,
                    street_address,
                    state,
                    None,
                    Some(city),
                    None,
                    Some(zip5),
                    None,
                )
                .await
                {
                    Ok(adr) => {
                        eprintln!("adr:{adr:?}");
                        Ok(())
                    }
                    Err(err) => {
                        eprintln!("err adr:{err:?}");
                        Err(Box::new(err))
                    }
                }
            } else {
                Err(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    "unknown response",
                )))
            }
        }
        Err(err) => {
            eprintln!("err auth:{err:?}");
            Err(Box::new(err))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_address_success() {
        let result = fetch_address().await;
        assert!(result.is_ok(), "fetch_address should succeed");
    }
}
