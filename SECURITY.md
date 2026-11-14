# Security policy for AttoCo projects

The `atto-co` repositories contain experimental commercial infrastructure. A repository or draft PR is not proof of deployed service readiness, production isolation, compliance, or completed security testing.

## Reporting a vulnerability

Please avoid posting exploit details, real credentials, or customer data in public issues or pull requests. Where GitHub private vulnerability reporting is enabled, use the **Security** tab of the affected repository to submit a private report to maintainers. If that option is unavailable, request a private reporting channel from the repository maintainers without disclosing exploit details publicly.

Include the affected repository and commit, a minimal reproduction with synthetic data, impact and the relevant tenant or trust boundary. Never attach live tokens, passwords, or sensitive tenant data.

## Review expectations

Security-relevant changes must state their threat model, data ownership, tenant separation, authentication and authorization, resource limits, replay/fencing model, failure recovery and audit evidence. Prefer fail-closed validation, bounded queues, immutable pinned inputs, exact build provenance, and negative adversarial tests.

Do not assume a response-time SLA or bug bounty exists unless the relevant project publishes one.
