# USPS Addresses SDK in Rust

See [USPS Addresses API documentation](https://developer.usps.com/api/93).

See [OpenAPI Generator Rust documentation](https://openapi-generator.tech/docs/generators/rust/).


Step 1. Generate SDK.
```sh``
> pwd
/home/rana/prj/usps-addresses-sdk

curl -O https://developer.usps.com/sites/default/files/apidoc_specs/US_Postal_Service-addresses-3.0.2-resolved_1.yaml

openapi-generator-cli generate -i US_Postal_Service-addresses-3.0.2-resolved_1.yaml -g rust --additional-properties=packageName=usps-addresses-sdk,preferUnsignedInt=true
```

Step 2. Manually update Cargo.toml.
- `reqwest`
    - Add `default-features = false`
    - Add `rustls-tls`
```toml
reqwest = { version = "^0.12", default-features = false, features = [
    "json",
    "multipart",
    "rustls-tls",
] }
```