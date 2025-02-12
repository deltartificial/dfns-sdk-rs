# Dfns SDK Rust

![Build status](https://img.shields.io/badge/build-passing-brightgreen?style=flat) ![License](https://img.shields.io/badge/license-MIT-blue) ![Implementation](https://img.shields.io/badge/implemented-100%25-green)

**Modular, extensible, and easy-to-use Rust SDK for the Dfns API.**

![](./assets/sdk-rs.png)

> **Warning**: This SDK is currently under development and not ready for production use. The API is subject to change. This SDK is not fully implemented and missing tests.

- [Dfns Website](https://www.dfns.co)
- [Dfns API Docs](https://docs.dfns.co)

## Documentation

### Generating Documentation

To generate the documentation locally, you'll need the nightly Rust toolchain. Here's how to set it up:

```bash
# Install and switch to nightly Rust
rustup install nightly
rustup default nightly

# Generate the documentation
RUSTDOCFLAGS="--enable-index-page -Zunstable-options" cargo doc --no-deps --document-private-items --target-dir ./docs
```

### Viewing Documentation

To view the documentation in your browser:

```bash
# Start a local server (Python required)
python3 -m http.server 8000 --directory ./docs/doc
```

Then open [http://localhost:8000/dfns_sdk_rs/](http://localhost:8000/dfns_sdk_rs/) in your browser.
