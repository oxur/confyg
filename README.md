# confyg

[![][build-badge]][build]
[![][crate-badge]][crate]
[![][tag-badge]][tag]
[![][docs-badge]][docs]

[![][logo]][logo-large]

*A simple, TOML-based, ENV-enabled library for configuration management*

## Overview

`confyg` provides a flexible way to build application configurations by:
- Loading from TOML files with path searching
- Scanning environment variables with prefix mapping
- Merging multiple configuration sources with override semantics
- Deserializing directly into strongly-typed Rust structs

## Features

- **Multiple Sources**: Combine TOML files, environment variables, strings, and Rust structs
- **Path Search**: Automatically search multiple directories for configuration files
- **Environment Variable Mapping**: Map prefixed environment variables to TOML sections
- **Deep Merging**: Intelligently merge configurations with clear priority rules
- **Type Safety**: Deserialize directly into your Rust types using serde
- **Error Handling**: Comprehensive error types with context preservation
- **Zero Unsafe**: Pure safe Rust with no unsafe code

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
confyg = "0.3"
serde = { version = "1.0", features = ["derive"] }
```

Basic example:

```rust
use confyg::{Confygery, Result};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    env: String,
    database: Database,
}

#[derive(Debug, Deserialize)]
struct Database {
    host: String,
    port: u16,
}

fn main() -> Result<()> {
    let config: Config = Confygery::new()?
        .add_file("defaults.toml")?
        .add_file("production.toml")?
        .build()?;

    println!("Environment: {}", config.env);
    println!("Database: {}:{}", config.database.host, config.database.port);
    Ok(())
}
```

## Usage Examples

### Loading from Files

```rust
use confyg::{Confygery, conf, Result};

let mut opts = conf::Options::default();
opts.add_path("./config")
    .add_path("/etc/myapp")
    .add_path("/usr/local/etc/myapp");

let config: MyConfig = Confygery::new()?
    .with_opts(opts)?
    .add_file("defaults.toml")?
    .add_file("production.toml")?
    .build()?;
```

### Environment Variables

Environment variables follow the pattern: `TOPLEVEL_SECTION_KEY=value`

```bash
# Set environment variables
export MYAPP_ENV=production
export MYAPP_DATABASE_HOST=db.example.com
export MYAPP_DATABASE_PORT=5432
```

```rust
use confyg::{Confygery, env, Result};

let mut env_opts = env::Options::with_top_level("myapp");
env_opts.add_section("database");

let config: MyConfig = Confygery::new()?
    .add_file("defaults.toml")?
    .add_env(env_opts)?  // Environment overrides file config
    .build()?;
```

### Configuration Priority

Sources added later override earlier sources:

```rust
Confygery::new()?
    .add_str(defaults_toml)?       // Priority: 1 (lowest)
    .add_file("defaults.toml")?    // Priority: 2
    .add_file("production.toml")?  // Priority: 3
    .add_env(env_opts)?            // Priority: 4 (highest)
    .build()?
```

### Inline Configuration

```rust
let config: MyConfig = Confygery::new()?
    .add_str(r#"
        env = "development"

        [database]
        host = "localhost"
        port = 5432
    "#)?
    .build()?;
```

### From Rust Structs

```rust
use serde_derive::Serialize;

#[derive(Serialize)]
struct Defaults {
    timeout: u32,
    retries: u8,
}

let defaults = Defaults {
    timeout: 30,
    retries: 3,
};

let config: MyConfig = Confygery::new()?
    .add_struct(&defaults)?
    .add_file("config.toml")?
    .build()?;
```

## Environment Variable Naming

Environment variables are converted to lowercase TOML keys:

| Environment Variable | TOML Equivalent |
|---------------------|-----------------|
| `MYAPP_ENV` | `env = "..."` |
| `MYAPP_LOG_LEVEL` | `log_level = "..."` |
| `MYAPP_DATABASE_HOST` | `[database]`<br>`host = "..."` |
| `MYAPP_DATABASE_PORT` | `[database]`<br>`port = "..."` |

**Note**: Due to environment variable naming limitations:
- Use underscores instead of dots in section names
- Hyphens in section names become underscores (e.g., `my-app` → `MY_APP`)
- All keys are converted to lowercase

## Examples

See the [examples](./examples) directory for complete working examples:

- [`env.rs`](./examples/env.rs) - Environment variable scanning
- [`env_str.rs`](./examples/env_str.rs) - Combining strings and environment variables
- [`file_search.rs`](./examples/file_search.rs) - File path searching
- [`full.rs`](./examples/full.rs) - Complete example with all features

Run examples:

```bash
make demos   # Run all examples with environment variables set
```

## API Documentation

Full API documentation is available on [docs.rs](https://docs.rs/confyg/).

## Error Handling

All operations return `Result<T, ConfigError>` with descriptive error messages:

```rust
match Confygery::new()?.add_file("config.toml")?.build::<MyConfig>() {
    Ok(config) => println!("Loaded config: {:?}", config),
    Err(e) => eprintln!("Configuration error: {}", e),
}
```

Error types include:
- `FileRead` - Failed to read a file
- `TomlParse` - Invalid TOML syntax
- `TomlSerialize` - Failed to serialize to TOML
- `FileNotFound` - File not found in search paths
- `NoConfigs` - No configuration sources added
- `InvalidState` - Invalid configuration state
- `InvalidPath` - Path contains invalid UTF-8

## Requirements

- Rust 1.75 or later
- `serde` with `derive` feature for deserialization

## License

Copyright © 2022-2026, Oxur Group

Apache License, Version 2.0

[//]: ---Named-Links---

[logo]: resources/images/logo-v1-small.png
[logo-large]: resources/images/logo-v1.png
[build]: https://github.com/oxur/confyg/actions/workflows/cicd.yml
[build-badge]: https://github.com/oxur/confyg/actions/workflows/cicd.yml/badge.svg
[crate]: https://crates.io/crates/confyg
[crate-badge]: https://img.shields.io/crates/v/confyg.svg
[docs]: https://docs.rs/confyg/
[docs-badge]: https://img.shields.io/badge/rust-documentation-blue.svg
[tag-badge]: https://img.shields.io/github/tag/oxur/confyg.svg
[tag]: https://github.com/oxur/confyg/tags
