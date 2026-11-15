# Rust-first test migration register

Recorded 2026-11-15. The links below are **draft work**, not merged organization policy or completed org-wide enforcement.

| Repository | Draft PR | Rust test/tooling change |
| --- | --- | --- |
| `atto-co/.github` | [#3](https://github.com/atto-co/.github/pull/3) | Org guidance, Rust-only policy guard and CI test |
| `atto-co/atto-observability.rs` | [#2](https://github.com/atto-co/atto-observability.rs/pull/2) | Native Rust telemetry reference/tests; old Python test files deleted |
| `atto-co/atto-marketplace` | [#2](https://github.com/atto-co/atto-marketplace/pull/2) | Native Rust listing reference/tests; old Python test files deleted |
| `atto-co/atto-infra` | [#2](https://github.com/atto-co/atto-infra/pull/2) | Native Rust manifest validator/tests; old Python test files deleted |
| `atto-co/atto-stack-cli` | [#2](https://github.com/atto-co/atto-stack-cli/pull/2) | Rust-authored integration tests run the **actual product CLI**; old Python test files deleted |
| `atto-co/atto-durable-objects.atto` | [#17](https://github.com/atto-co/atto-durable-objects.atto/pull/17) | Rust lease-window mutation/boundary matrix and guard |
| `atto-co/atto-runtime-host.rs` | [#22](https://github.com/atto-co/atto-runtime-host.rs/pull/22) | Rust generation-fencing regressions and guard |

## Remaining migration and release gates

- **`atto-co/atto-integrations` [#2](https://github.com/atto-co/atto-integrations/pull/2):** still has Python-authored tests for the webhook replay reference. Replace with a Rust harness that exercises the real HMAC implementation (avoid an independent mock or hand-rolled crypto). Do not delete the existing tests before comparable behavior can be checked.
- Audit all other repositories and older feature branches for test drivers, as well as any CI workflows under `.github/workflows`; use Rust in new test/check scripts. This register is **not** a complete org-wide audit.
- Where a Python product-language reference coexists with a new Rust implementation (marketplace/infra/observability), native Rust unit tests do **not** prove Python/Rust behavioral equivalence. Add differential checks before removing compatibility code.
- The `atto-co/.github` CI policy gate operates in that repository; each consuming repo must integrate the guard or trusted reusable workflow explicitly. Documentation alone is not a required GitHub status check elsewhere.
- Run exact-head, review-backed CI. If source-org Actions cannot run, verify Git blob/tree SHA in a token-free alternate private runner. Never count skipped or pending jobs as passed.
- Do not merge draft features or security-sensitive parent PRs without independent approval, successful checks and branch-protection compliance.
