# Leptos Trading view experiment

[![check](https://github.com/roberts-ivanovs/leptos-web-trading/actions/workflows/check.yaml/badge.svg)](https://github.com/roberts-ivanovs/leptos-web-trading/actions/workflows/check.yaml)
[![docs](https://github.com/roberts-ivanovs/leptos-web-trading/actions/workflows/doc.yaml/badge.svg)](https://github.com/roberts-ivanovs/leptos-web-trading/actions/workflows/doc.yaml)
[![msrv](https://github.com/roberts-ivanovs/leptos-web-trading/actions/workflows/msrv.yaml/badge.svg)](https://github.com/roberts-ivanovs/leptos-web-trading/actions/workflows/msrv.yaml)
[![test](https://github.com/roberts-ivanovs/leptos-web-trading/actions/workflows/test.yaml/badge.svg)](https://github.com/roberts-ivanovs/leptos-web-trading/actions/workflows/test.yaml)
[![unused-deps](https://github.com/roberts-ivanovs/leptos-web-trading/actions/workflows/unused-deps.yaml/badge.svg)](https://github.com/roberts-ivanovs/leptos-web-trading/actions/workflows/unused-deps.yaml)
[![deny](https://github.com/roberts-ivanovs/leptos-web-trading/actions/workflows/deny.yaml/badge.svg)](https://github.com/roberts-ivanovs/leptos-web-trading/actions/workflows/deny.yaml)
[![audit](https://github.com/roberts-ivanovs/leptos-web-trading/actions/workflows/audit.yaml/badge.svg)](https://github.com/roberts-ivanovs/leptos-web-trading/actions/workflows/audit.yaml)

## Development setup

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install cargo-make
cargo install cargo-make
cargo install --locked cargo-leptos
```

## Development commands

```bash
# Run the frontend
cargo make watch

# Build the server
cargo make build

# Test the code
cargo make test

# Format the code
cargo make format

# Check the code
cargo make check

# Perform all of the CI checks
cargo make local-ci
```
