# API and process evidence

Official sources checked 2026-09-05. Refresh relevant pages, current team ownership, and the actual proposal/bot state when making a live process judgment. Historical examples do not establish present approval. Some guide pages retain older team names; prefer current project decisions when they differ, and identify any unresolved discrepancy.

## Keep the decisions separate

The [feature lifecycle](https://std-dev-guide.rust-lang.org/development/feature-lifecycle.html) describes ACP acceptance as an early indication of suitability, not a stabilization promise. An ACP is not strictly required; a PR without one needs the appropriate library API review. Small unstable designs and significant changes to an accepted design need proportionate judgment, not a blanket “no ACP, no review” rule. Existing tracking issues can cover revisions of their unstable APIs. Stable trait impls skip an unstable period and deserve extra scrutiny.

The [reviewing guide](https://std-dev-guide.rust-lang.org/team/reviewing.html) connects approval to confidence in the code, compatibility and performance, respecting other reviewers, and completed FCP for changes to public stable promises. For an API/process concern, name the actual decision missing rather than using “needs FCP” as a generic objection. Inspect the proposal's disposition and resolution of concerns: an opened FCP, a checked box, an accepted ACP, and a completed merge FCP are different facts.

## Audit the proposed stable surface

For stabilization or partial stabilization, extract the exact current API, including trait impls, bounds, constness, re-exports and documentation commitments. Compare the current proposal, stabilization report, tracking discussion, and changed attributes. Follow feature references to catch items accidentally included or excluded. Do not treat every unresolved question about the broader experimental feature as a blocker for a narrower proposed subset; explain the dependency that makes it relevant.

The [stabilization guide](https://std-dev-guide.rust-lang.org/development/stabilization.html) asks reviewers to understand implementation history, the precise API and practical experience. Reports are strongly encouraged rather than universally mandatory. For partial stabilization, preserve the intended remaining experimental surface. Check stability/const-stability attributes and test/documentation gates, using the current version placeholder conventions. Const stabilization also needs the relevant const-evaluation input; exposing const intrinsics has additional language-team involvement. Check internal unstable dependencies rather than assuming changing one attribute proves the commitment viable.

## Compatibility beyond the signature

- [New trait implementations](https://std-dev-guide.rust-lang.org/breaking-changes/new-trait-impls.html): an additional impl can disrupt inference or deref coercion; fundamental types also affect downstream coherence. Identify concrete affected patterns and use ecosystem evidence proportionate to the risk. A crater run is an experiment requiring interpretation of failures, not just a status color.
- [Documentation guarantees](https://std-dev-guide.rust-lang.org/breaking-changes/doc-changes.html): promising a new behavior can be an API change even if implementation stays identical. A current-implementation note and a stable contract serve different purposes; do not use wording to disguise a real compatibility decision.
- [Writing documentation](https://std-dev-guide.rust-lang.org/development/how-to-write-documentation.html): examples should communicate behavior and be checked where practical; distinguish `no_run`, expected panic, and compile failure from blanket `ignore`.
- [Target-specific review](https://std-dev-guide.rust-lang.org/policy/target-code.html): first establish that the change is genuinely target-confined. Tier 1 requires full review. The documented tier 2/3 workflow emphasizes isolation, licensing and target-maintainer endorsement rather than requiring general Libs reviewers to establish all platform correctness. A user-requested deep platform review can still go further; be explicit about tested and unverified targets.
