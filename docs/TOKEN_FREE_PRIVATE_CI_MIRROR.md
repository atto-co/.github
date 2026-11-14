# Token-free private GitHub Actions mirror — failure-contingent CI procedure

Use this runbook **only when a source repository's GitHub Actions runner cannot
execute** (for example, its organization's Actions minutes are exhausted).
Never label a queued/skipped job as passing. Source CI is preferred whenever
available.

## Requirements

- The source and carrier repositories are **private** and both authorized for
  the same task; verify carrier access and retention controls before any copy.
- The carrier organization has a demonstrably working, available Actions
  runner and appropriate budget. Do not guess based on organizational name.
- The exact source branch commit SHA is approved as a test candidate; no
  source branch or default branch is rewritten.
- Do **not** export production secrets, credentials, protected environment
  variables, private keys, customer datasets, or unapproved Git LFS objects.
- All file modes, file bytes, executable bits, symlink entries, empty trees,
  submodule references and tracked paths are reviewed. Abort if the connector
  cannot faithfully reproduce any tree entry; never silently omit binaries or
  submodules.

## Mirror the immutable Git tree

1. Fetch the source commit and its **tree SHA** using the GitHub connector.
   Record both SHAs and verify the source PR head has not changed.
2. Fetch the entire source Git tree recursively (pagination included). Transfer
   every exact Git blob into a temporary branch of the preauthorized private
   carrier repository using connector Git blob/tree/commit APIs. Preserve each
   entry's exact path, mode, object type and content (including binary blobs).
   Do **not** use a remote clone or checkout of the private source repository.
3. Verify the produced carrier tree SHA equals the recorded source tree SHA.
   Git object hashes prove exact tree identity; a matching list of filenames
   alone is insufficient. Independently compare byte sizes and per-file blob
   SHA values for the material copied. On any mismatch, abort without running
   or reporting tests.
4. Create an isolated **temporary carrier branch** pointing to a commit using
   that exact tree. The source and carrier commit SHAs may differ (commit
   parent, metadata or message) while their **tree SHAs must match**.
5. Trigger only a controlled existing carrier-local workflow. Its checkout
   uses the carrier branch's pinned commit SHA with `persist-credentials:
   false`, `permissions: contents: read` and no repository/organization
   secrets, upload credentials, untrusted actions, publishing or deployments.
   The workflow asserts the checked-out SHA and `HEAD^{tree}` equal the
   recorded carrier commit and source tree before invoking native tests.
6. Collect completed job conclusions and **executed** unit/integration-test
   counts from logs. Record: source commit SHA, source tree SHA, carrier
   repository/temporary branch, carrier commit SHA, carrier tree SHA,
   Actions run/job IDs, toolchain, command, output and resulting conclusion.
   `queued`, `pending`, `cancelled`, `skipped` and `neutral` are **not**
   successful test execution.
7. Re-read the source PR head; if it changed during testing, the evidence is
   stale for the new head. Repeat from its new immutable commit. Remove the
   temporary carrier branch when no longer needed, following retention rules.

## Threat and trust boundary

The carrier's own `GITHUB_TOKEN` cannot read a separate private source
organization's repository. This is intentional. Never pass a cross-org PAT,
machine-user token or source repo secret to the carrier. The GitHub connector
performs an explicitly authorized **data copy**, not a remote checkout. This
also means source code is disclosed to the carrier organization's Git
repository and runners: only use a carrier approved to hold the code.

A successful mirror test proves the exact **tree** ran under the recorded
carrier environment. It does not prove independent approval, correct
provenance of a separately built artifact, production safety, or branch
protection. Native tests plus independent review and normal merge governance
are still required.
