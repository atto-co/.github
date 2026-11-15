# Contributing to atto-co

## Mandatory Rust-first test/tooling convention

- Author **new** tests, test harnesses, property-based and mutation checks, fixture generation, validation scripts, release tooling, CI helpers and repeatable repository automation in Rust (`.rs`).
- Run `cargo test --locked` (and `cargo fmt --check`, `cargo clippy -- -D warnings` where configured). For std-only tiny tooling use `rustc --edition=2021 --test ...`; never replace real tests with static substring checks.
- Do not add `tests/**/*.py`, `scripts/**/*.py`, `pytest`, `unittest`, or GitHub Actions steps that invoke Python test drivers. Update existing Python test suites to Rust before adding new coverage.
- Source written in another language may remain if it is an actual product/runtime compatibility contract. Do not call Rust reimplementations parity proof without running behavioral tests against the real product implementation. Separate language-specific product code from test orchestration.
- Test negative/error inputs, boundary values, multi-tenant isolation, authorization and replay/overflow behavior; require exact SHA binding for mirrored private-source CI. Alternate runner use must not require cross-organization read secrets.
- Do not bypass branch protection or independent approvals. Any migration should be a draft PR until all required checks and reviews pass.

See [Rust-first testing policy](docs/RUST_FIRST_TESTING.md) for migration steps and how to use the guard. This organization-level policy is guidance until each repository integrates it in its own CI.
