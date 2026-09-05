# Remediation and migration

The selected repair must close the soundness hole for every input and
configuration covered by the existing contract. Compatibility, performance,
and flexibility decide among complete repairs; they cannot justify a partial
one. Compare repair designs privately and specify the evidence and decisions a
human author needs; implementation, project actions, and public material are
outside this advisory workflow.

## Disqualifying non-fixes

Reject a candidate that:

- handles only the supplied reproducer while an equivalent generic, cfg,
  target, panic, reentrant, or concurrent path remains;
- changes only documentation while a stable safe API can still reach UB;
- silently adds an unchecked precondition to a safe API;
- strengthens a stable unsafe API's caller contract without treating that as a
  contract-breaking change and providing migration;
- relies on `debug_assert!`, a test-only check, current optimizer luck, or a
  compiler behavior whose required semantics are not established;
- converts UB into a panic or error only on some affected paths;
- preserves benchmark numbers by retaining the invalid operation.

Making a safe API unsafe is a major source break and does not repair already
compiled callers. Treat it as a last-resort API redesign, not a small patch.

## Candidate order

Explore the least disruptive applicable shapes first:

1. **Internal contract-preserving repair.** Correct the implementation while
   preserving the stable signature, documented behavior, layout, and normal
   performance characteristics.
2. **Narrow checked path or fallback.** Preserve the common fast path, validate
   at a boundary where the invariant can be established once, and use defined
   conservative behavior only for the affected state. Account for new panics,
   errors, allocations, branches, and code size.
3. **Narrow source rejection.** If the defect originates in a compiler-accepted
   source pattern, reject only the unsound subset and keep unaffected patterns
   working. Prefer a machine-applicable migration when it is semantics-
   preserving, but final diagnostics and suggestions must be human-authored.
4. **Edition-gated contract correction.** If an existing safe operation has an
   unavoidable caller proof obligation that the language can express by
   edition, consider making it unsafe in a later edition with a compatibility
   lint and a safe alternative. This requires coordinated libs, language, and
   compiler approval. Automatic insertion of `unsafe` is only a migration aid;
   a human must audit the new obligation. The
   [Rust 2024 newly unsafe functions](https://doc.rust-lang.org/stable/edition-guide/rust-2024/newly-unsafe-functions.html)
   are precedent, not blanket authorization.
5. **Safe replacement with an explicit unchecked escape hatch.** If checking
   has a material measured cost or removes legitimate low-level flexibility,
   consider a sound checked API plus a clearly unsafe `_unchecked` operation
   whose preconditions are expressible and useful. This is a public API design
   requiring the applicable libs-api process. Do not move an invariant to
   downstream callers merely to avoid doing the implementation work.
6. **Deprecation and replacement.** When the stable contract cannot be
   implemented soundly, retain the item where compatibility requires it,
   provide a sound replacement, and define how the old entry point behaves
   without UB during migration.
7. **Immediate break or removal of capability.** Reserve for cases where no
   complete compatible repair exists and delay is worse than disruption. This
   requires explicit responsible-team judgment and may require Security
   Response coordination.

An internal unstable feature, intrinsic, or compiler/stdlib contract is
available to the standard library when justified. Public documentation is not
required for every intentional internal contract. State the required semantics,
evidence of that contract, concrete usage constraints, maintenance coupling,
and demonstrated performance or flexibility benefit. Observed code generation
alone does not establish the contract. If its meaning or intended guarantee is
unclear, make the candidate conditional on a decision by the relevant humans.
Do not replace a public compatibility problem with an unexamined soundness
premise.

## Comparison matrix

For every complete candidate, report:

| Dimension | Questions |
| --- | --- |
| Soundness | Which invariant changes, why are all paths closed, and what proof premise remains? |
| API contract | Are signature, safety contract, documented behavior, layout, ABI, traits, and const guarantees preserved? |
| Source and behavior | Which callers stop compiling, panic, return a different result, or observe different timing/order? |
| Ecosystem | How many validated root projects and important dependants need migration? |
| Performance | What changes on common and pathological paths in runtime, compile time, code size, allocation, and optimization? |
| Flexibility | Are genericity, target support, allocator choice, `no_std`, const use, or future implementation options lost? |
| Delivery | Can it be backported, tested on supported branches, deployed promptly, and reverted safely? |
| Auditability | Is the new invariant simpler and visible, or does it add fragile state and compiler coupling? |

Require soundness completeness, and weigh deployability against the response
timeline. Among complete candidates, strongly prefer preserving stable
contracts while comparing demonstrated compatibility costs with performance,
flexibility, and maintenance costs. This is not an absolute ordering: a severe
runtime regression and a narrow source break require an explicit tradeoff for
the responsible team. Patch size is secondary.

If the only complete repair has a material regression, say so plainly and
route the tradeoff to the responsible team. Explore moving checks to invariant-
establishing boundaries, caching validated state, narrowing slow paths, or
using justified internal mechanisms before accepting a broad hot-path cost.
Measurements must cover representative types, sizes, targets, and profiles;
microbenchmarks alone may miss code size or downstream effects.

When a performant candidate is complete in design but has narrower validation
than a simpler conservative repair, recommend it conditionally with explicit
proof, target, and benchmark gates. Name the complete conservative repair as
the fallback if those gates cannot be cleared in time for the needed release.
Do not turn “more testing needed” into an indefinite delay without a safe path
to ship.

## Containment and durable repair

Urgent response may need two complete fixes on different timelines:

- a narrow, easily reviewed and backported containment for affected release
  branches, which may conservatively disable an optimization or capability; and
- a durable repair that best preserves compatibility, performance,
  flexibility, and auditability on the main branch.

Containment must itself close the hole; it is not permission for a partial
guard. Record its temporary costs, supported branches, rebuild requirements,
and removal or replacement condition. Do not delay a deployable containment
for an ideal redesign, or fossilize a costly containment when a demonstrated
durable repair is ready.

## RFC 1589 rollout

For a compiler correction that makes accepted source fail, assess whether the
human rollout plan covers the following work:

1. Measure impact with Crater or an equivalent experiment and triage total
   affected projects.
2. Maintain a dedicated tracking issue focused on the change and migration.
3. Prefer a future-compatibility warning when the unsound pattern can be
   recognized precisely and the risk of temporary continued acceptance is
   tolerable.
4. Keep the warning in the wild for at least one release cycle before a hard
   error, then use the current team decision process.
5. If warning is infeasible, use a targeted human-authored diagnostic, migrate
   or notify known affected crates, and consider layering enforcement.
6. Treat direct error for fewer than 10 affected projects as a reasoned option,
   not an automatic outcome; retain tracking, migration, and outreach.

A warning does not close the soundness hole. Explicitly compare the migration
benefit against another release cycle of exposure. An urgent public defect may
justify immediate enforcement through the responsible team's exception process
even when an embargo is not involved.

If a complete runtime guard contains the defect while a compiler restriction
is warning-only, retain the guard. Remove it only after the hard restriction is
active on the relevant channel and proved to cover every formerly guarded path.

Under the current compiler approval guide, a compiler-feature future-
compatibility warning is proposed through an MCP and approved by FCP; a
language-feature warning follows the language team's process. Verify the live
[proposal guidance](https://forge.rust-lang.org/compiler/proposals-and-stabilization.html#lints-errors-and-warnings)
instead of treating RFC 1589's historical mechanics as current syntax.

For a runtime-only standard-library implementation repair, these goals still
matter but a compiler lint may be irrelevant. Focus on preserving the existing
contract, identifying affected releases, testing the runtime state, deciding
backports, and telling humans whether users must update and rebuild.

If the remediation adds or changes a stable public API or semantic contract,
route it to `T-libs-api` and the applicable ACP/FCP/RFC process as determined by
the team. An implementation-only change normally belongs with `T-libs`.
Language or memory-model changes require the corresponding language or
operational-semantics decision; a patch cannot decide semantics by accident.

## Migration design

Specify for human authors:

- the affected source or runtime pattern;
- a semantics-preserving destination state, not merely syntax that compiles;
- whether old and new toolchains accept the migrated form;
- diagnostic, lint, deprecation, or tooling feasibility;
- affected-crate outreach and ordering;
- warning duration, hard-error gate, and rollback signals where applicable;
- toolchain update, rebuild, redeploy, and backport requirements;
- what happens to binaries or libraries built before the fix.

For an urgent security issue, public warnings and pre-landing outreach may leak
the vulnerability. Security Response controls coordinated disclosure and may
choose a different sequence.

## Verification plan

Specify evidence proportionate to the change, separating available results and
bounded local checks from wider validation needed from human operators:

- a non-weaponized regression that fails before and is safe after;
- proof review of the repaired invariant and an audit for sibling patterns;
- Miri, sanitizers, fuzzing, or model tests where they exercise the mechanism;
- target/cfg/profile/panic/allocator/const coverage relevant to the bug;
- compile-pass, compile-fail, runtime, or codegen tests appropriate to the
  boundary;
- Crater or targeted ecosystem experiments for source/test fallout;
- before/after benchmarks for claimed performance and flexibility properties;
- supported-branch and rebuild verification for releases being repaired.

Tests and clean experiments support the proof; they do not replace it.

Use the [standard-library performance guide](https://std-dev-guide.rust-lang.org/development/perf-benchmarking.html)
for current benchmarking practice. `rustc-perf` is relevant only when the
compiler materially exercises the changed library path; it is not a universal
benchmark for standard-library runtime behavior.
