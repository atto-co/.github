# .github

Canonical atto-co organization repository for organization profile, default community health files, and shared GitHub configuration.

## Rust-first tests and scripts

**Organization convention: write new test suites, CI test runners, validation tools, fixture generators and automation scripts in Rust, not Python.** Use `cargo test --locked` for Cargo projects and standalone `rustc --test` for small std-only checks. Do not introduce new Python test scripts, Python-based test drivers, `pytest`, or `unittest` in CI. See [CONTRIBUTING.md](CONTRIBUTING.md) and [docs/RUST_FIRST_TESTING.md](docs/RUST_FIRST_TESTING.md).

Existing Python *product/runtime* reference implementations may be retained until deliberately migrated; **do not delete working production code just to satisfy a testing policy.** New testing and automation must use Rust. A Rust check in this repository can be copied into consuming repositories or invoked locally; the policy is not automatically enforced in other repositories until their CI integrates the gate.
