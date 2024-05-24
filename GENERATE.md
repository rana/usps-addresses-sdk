# USPS Addresses SDK in Rust

See [USPS Addresses API documentation](https://developer.usps.com/api/93).

See [OpenAPI Generator Rust documentation](https://openapi-generator.tech/docs/generators/rust/).

Step 1. Update Open API yaml if newly downloaded.
- Revise enum descriptions for proper enum definitions. Enum definitions were preventing SDK generation.
```yaml
    DPVConfirmation:
      type: string
      description: >
        The DPV Confirmation Indicator is the primary method used by the USPS® to determine whether an address was considered deliverable or undeliverable. 

        Enum values:
        - Y: Address was DPV confirmed for both primary and (if present) secondary numbers.
        - D: Address was DPV confirmed for the primary number only, and the secondary number information was missing.
        - S: Address was DPV confirmed for the primary number only, and the secondary number information was present but not confirmed.
        - N: Both primary and (if present) secondary number information failed to DPV confirm.
      enum:
        - Y
        - D
        - S
        - N
    DPVCMRA:
      type: string
      description: >
        Indicates if the location is a [Commercial Mail Receiving Agency (CMRA)](https://faq.usps.com/s/article/Mail-Services-at-Non-Postal-Sites-CMRA).

        Enum values:
        - Y: Address was found in the CMRA table.
        - N: Address was not found in the CMRA table.
      enum:
        - Y
        - N
    business:
      type: string
      description: >
        Indicates whether this is a business address.

        Enum values:
        - Y: The address is a business address.
        - N: The address is not a business address.
      enum:
        - Y
        - N
    centralDeliveryPoint:
      type: string
      description: >
        Central Delivery is for all business office buildings, office complexes, and/or industrial/professional parks. This may include call windows, horizontal locked mail receptacles, cluster box units.

        Enum values:
        - Y: The address is a central delivery point.
        - N: The address is not a central delivery point.
      enum:
        - Y
        - N
    vacant:
      type: string
      description: >
        Indicates whether the location designated by the address is occupied.

        Enum values:
        - Y: The address is occupied.
        - N: The address is not occupied.
      enum:
        - Y
        - N
```

Step 2. Generate SDK.
```sh``
> pwd
/home/rana/prj/usps-addresses-sdk

curl -O https://developer.usps.com/sites/default/files/apidoc_specs/US_Postal_Service-addresses-3.0.2-resolved_1.yaml

openapi-generator-cli generate -i US_Postal_Service-addresses-3.0.2-resolved_1.yaml -g rust --additional-properties=packageName=usps-addresses-sdk,preferUnsignedInt=true
```

Step 3. Comment out duplicate generated code.
```rust
    // if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
    //     local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    // };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
```

Step 4. Manually update Cargo.toml.
- `reqwest`
    - Add `default-features = false`
    - Add `rustls-tls`
- Add [dev-dependencies] for tests.
```toml
reqwest = { version = "^0.12", default-features = false, features = [
    "json",
    "multipart",
    "rustls-tls",
] }

[dev-dependencies]
tokio = { version = "1.37.0", features = ["full"] }
usps-oauth-sdk = { git = "https://github.com/rana/usps-oauth-sdk.git" }
```