# Rust-first testing and automation policy

Effective 2026-11-15 for **new** atto-co work.

Use Rust for test programs, smoke and conformance suites, CI validation helpers, fixture generation, standalone scripts and security regressions. Do **not** add new Python test scripts or script-based testing automation. Prefer a normal `Cargo.toml` with `#[test]` integration tests; tiny dependency-free scripts may use `rustc`. Keep exact source Git SHA verification in CI and use native/Wasm compiler differential tests where relevant.

## Migration of existing repositories

1. Inventory existing `tests/*.py`, `scripts/*.py` and workflow `python -m unittest` / `pytest` execution.
2. Move genuine tests into `tests/*.rs`; keep behavioral boundary, negative and failure-atomicity coverage. Do not simply port assertions to an unrelated Rust mock: test the actual Rust implementation or use an honest cross-language differential harness.
3. Update workflows to install a pinned Rust toolchain and run `cargo test --locked` (or `rustc --test` for std-only validation).
4. Remove retired Python *test drivers* and stale documentation. Do not delete legitimate Python product/runtime source files without a replacement and compatibility evidence.
5. Check clean failures, exact-head SHA, required independent reviews and branch protection before merging. Unverified or skipped tests are not pass results.

## Standalone Rust gate

```sh
rustc --edition=2021 --test tools/rust_first_guard.rs -o /tmp/rust_first_guard_tests
/tmp/rust_first_guard_tests
rustc --edition=2021 tools/rust_first_guard.rs -o /tmp/rust_first_guard
/tmp/rust_first_guard .
```

The gate scans `tests/`, `test/`, `scripts/`, and `tools/` for Python-script files, and scans `.github/workflows/` for common Python testing invocations. It intentionally does **not** reject existing language-specific product modules (e.g. `contracts/reference.py`) or Python source at unrelated product paths. This guard is a convenience and not a full YAML parser; reviewers must still inspect workflow scripts.

The gate runs on `atto-co/.github` itself. For other repositories, copy/vendor the guard into each repo or explicitly add a trusted shared workflow reference with appropriate repository permissions. An org .github README alone **does not enforce** a CI policy across every repo.
