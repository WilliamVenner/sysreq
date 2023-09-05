[![crates.io](https://img.shields.io/crates/v/sysreq.svg)](https://crates.io/crates/sysreq)
[![docs.rs](https://docs.rs/sysreq/badge.svg)](https://docs.rs/sysreq/)
[![license](https://img.shields.io/crates/l/sysreq)](https://github.com/WilliamVenner/sysreq/blob/master/LICENSE)

# sysreq

Simple, virtually-zero-dependencies HTTP client wrapping a system client. Make HTTP requests without breaking the bank!

"Virtually-zero" means no unnecessary runtime dependencies. The only runtime dependency, other than `std`, is URL validation, which is required for security reasons.

## Supported Backends

* wget
* cURL
* PowerShell (`Invoke-WebRequest`)

# Usage

In your `Cargo.toml`:

```toml
[dependencies]
sysreq = "0.1"
```

In your code:

```rust
let html = sysreq::get("https://www.rust-lang.org/").unwrap();
println!("{}", String::from_utf8_lossy(&html));
```