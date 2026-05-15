# Canton Ledger API Rust Client

Rust client for the Canton JSON Ledger API, generated from the OpenAPI spec using [openapi-generator](https://openapi-generator.tech) with manual adjustments.

- Canton API version: 3.6.0
- Minimum Canton version: 3.6.0
- Package version: 3.6.0-0.1.0

## Installation

Add to your `Cargo.toml`:

```toml
canton-api-client = "3.6.0-0.1.0"
```

Or use a path dependency:

```toml
canton-api-client = { path = "../canton-api-client" }
```

## Regenerating

The client is generated from `openapi-spec.yaml`. To regenerate:

```bash
openapi-generator generate -i openapi-spec.yaml -g rust -o . --package-name canton-api-client
```

After regenerating, reapply manual changes:

1. **Remove `post_v2_packages`** - Remove the `PostV2PackagesError` enum and `post_v2_packages()` function from `src/apis/default_api.rs` (deprecated endpoint, use `/v2/dars` instead)
2. **Update User-Agent** - In `src/apis/configuration.rs`, change the default `user_agent` to `"CantonAPI/rust"`
3. **Update `Cargo.toml`** - Set version to `<api-version>-0.1.x`, author to `robert@dlc.link`, description to `"Canton Ledger API rust client"`

## Documentation

```bash
cargo doc --open
```
