# confyg

[![][build-badge]][build]
[![][crate-badge]][crate]
[![][tag-badge]][tag]
[![][docs-badge]][docs]

[![][logo]][logo-large]

*A simple, TOML-based, ENV-enabled library that can find and merge configs*

## Usage

Be sure to see the [examples](./examples) (and the [Makefile](./Makefile), for how to run some of them with environment variables set).

Note that due to the fact that environment variables don't support the use of the `.` in variable names, the `toml` library's hierarchichal feature is not usable when you want to merge environment-based configs and string- or file-based configs with sections that have `.` in their names. You can get around this to a certain extent with underscores and switching up how you relate your Rust structs to sections (see the `./examples/env*` examples).

## License

Copyright © 2022, Oxur Group

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
