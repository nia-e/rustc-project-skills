---
name: rust-libs-pr-review
description: Privately review Rust standard-library pull requests for correctness, API and stabilization scope, compatibility, documentation, tests, performance, and maintainability. Use for core, alloc, std, and related library changes; route detailed unsafe proofs and soundness triage to the dedicated stdlib skills.
---

# Rust library PR review

Provide private, evidence-backed advice that helps a human reviewer distinguish defects, unanswered questions, API decisions, and preferences. The deliverable is an analysis for the user; all follow-up stays within private review. Do not produce or offer public-ready comments, PR text, source changes, or external actions such as posting, assigning, running shared CI, approving, or merging.

Inspect source and discussions and use bounded local checks while preserving the reviewed tree and uncommitted work. The human reviewer independently verifies findings and owns final decisions and communication. The [Rust LLM policy](https://forge.rust-lang.org/policies/llm-usage.html) provides project context; this skill's private-only scope does not depend on whether that policy permits other uses.

## Establish what is being reviewed

Record the PR URL, current head and base commits, local revision and modifications, and retrieval date. Distinguish the PR diff from unrelated checkout changes. Read the description and linked tracking issue/ACP/FCP. Normally read previous substantive reviews and their replies too; when the user asks for an independent pass, first form an initial judgment from source and governing contracts, then reconcile prior reviews and their resolutions. For re-review, find what changed since the relevant review as well as the final cumulative diff.

Resolve the user-visible purpose before reviewing implementation choices. Classify the change as a bug fix, refactor, optimization, unstable addition, stable trait implementation, documentation guarantee, or stabilization; a PR can contain several. Follow its affected public entry points into shared helpers, callers, and target implementations instead of stopping at changed lines.

Use current source and current discussion over memory, old PR descriptions, and historical reviewer conclusions. A force-push or revised stabilization proposal can invalidate an otherwise sound earlier assessment. If the head moves during review, state which commit the evidence covers and recheck affected conclusions.

## Review the contract and implementation together

Adapt the following to the changed behavior. Do not turn every item into a mandatory full audit.

- **Semantics:** identify the observable before/after behavior and the invariant connecting the implementation to it. Inspect boundary values, state transitions, errors, cancellation or unwind behavior where relevant. For iterators, account for partial consumption, mixed front/back calls, exhaustion, and specialized methods when touched. An analogous API is a comparison point, not proof that all its semantics transfer.
- **API scope:** enumerate the actual exposed surface, including methods, impls, bounds, re-exports, `const` capabilities, and new promises in documentation. Compare that surface with the current proposal and decisions. Separate suitability for std, permission to experiment on nightly, and permission to stabilize. Read [API and process evidence](references/api-and-process.md) for API additions, stabilization, trait impls, or changed guarantees.
- **Compatibility:** include type inference, method resolution, coherence, auto traits, diagnostics and extension-trait users where relevant. An unstable item can affect stable callers through shared implementation or name-collision warnings. A new trait impl is not harmless merely because it adds code. Trace actual exposure rather than labeling the entire feature unstable.
- **Platforms:** compare Rust's promised behavior with each affected platform's documented API, error conventions, fallback, supported versions, and representation. Inspect the `cfg` boundary first: does a purportedly local change touch shared paths? Check target policy and maintainer involvement for the actual scope. Host tests or a target's API resemblance do not establish compatibility elsewhere.
- **Documentation:** ask what a caller will believe after reading it. Separate guarantees from current implementation details; examine panic/error behavior, examples, units and indexing, target caveats, links and feature gates. An example should distinguish plausible interpretations instead of accidentally producing the same result under both.
- **Maintainability:** prefer an invariant a future reviewer can explain. Reuse established parsing or state machinery when that removes duplicate semantic rules. Identify missing rationale for non-obvious constants, fallback conditions, or ownership transitions; comments naming an algorithm are not its argument. Passing tests alone does not make an opaque implementation adequately reviewable.

For detailed unsafe-code proofs, safety comments, or contract questions, use `rust-stdlib-unsafe-review`; for a concrete proposed soundness defect and its exposure/response, use `rust-stdlib-unsoundness-response`. Keep the surrounding API, platform, and process review here, distinguishing documentation debt and compatibility concerns from established soundness defects.

## Choose evidence that can change the conclusion

Start with existing tests and the repository's current test layout/build configuration. Match the check to the claim: semantic/unit tests, compile/UI tests for inference and diagnostics, doctests, codegen tests for a claimed optimization, or target-specific execution. Record the actual command, revision, relevant configuration, and final result. Record the tested compiler, sysroot and build artifact separately from the source revision: an installed toolchain or stale build may not exercise the reviewed library. Distinguish a test that passed from a command still running, skipped coverage, or infrastructure failure.

Prefer independent oracles over two calls reading the same state or two copies of the same algorithm. Useful ordinary correctness checks include comparison with established behavior, a simple specification, or agreement between independently implemented operations. For stateful APIs, a few carefully chosen sequences can be more informative than many happy-path assertions. Run useful temporary correctness checks in isolated scratch space, keeping the reviewed source unchanged. Honor explicit read-only/no-test constraints and distinguish extracted experiments from the actual library build. When a check is unavailable, describe the evidence it would provide and the conclusion left open.

For performance changes, establish the workload, baseline/head, target, library and consumer build profiles, and metric before accepting a result. Ask for the relevant input distribution and regressions, not only the winning microbenchmark. Ensure the benchmark uses the changed library. A clean rustc-perf run measures compiler workloads and can miss the API's runtime cost; codegen improvement, runtime speed, compile time, memory, and binary size are separate evidence. See the [benchmarking guide](https://std-dev-guide.rust-lang.org/development/perf-benchmarking.html). Run local checks warranted by a material claim or unresolved risk. Assess existing shared perf/Crater reports; identify additional measurements a human would need without offering to submit jobs.

## Deliver a calibrated review

Lead with the useful conclusion and scope: material concerns, a specific decision still needed, or no material findings in the reviewed surface. Keep supporting notes proportional to the patch. For each actionable finding provide:

- the current source location and affected behavior;
- the violated contract or concrete compatibility risk, with evidence;
- the condition under which it matters;
- the smallest decision or verification that would resolve it.

Mark uncertainty explicitly and separate merge/stabilization concerns from follow-up work and nonblocking preferences. Do not demand an unrelated redesign to land a focused improvement. Conversely, do not let an approved proposal, prior reviewer, resolved GitHub thread, or green CI substitute for evaluating the current patch. “No material findings” is a valid outcome; it is not a human approval or a proof of soundness.

When ownership or expertise matters, identify the specific missing perspective and why it could change the conclusion. Preserve existing substantive reviewer concerns and note whether the current changes actually address them. End with material test coverage and limitations, not an exhaustive checklist of everything theoretically reviewable.

## Supporting material

- [API and process evidence](references/api-and-process.md): use for scope, compatibility, stabilization, platform policy, and source refresh.
- [Observed review techniques](references/review-examples.md): read when a concrete precedent helps shape a review or explain this skill's design; these are historical examples, not permanent rulings.
- [Behavioral checks](references/behavioral-checks.md): use when evaluating or changing this skill, not during every PR review.
