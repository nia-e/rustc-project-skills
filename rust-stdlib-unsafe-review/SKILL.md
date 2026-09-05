---
name: rust-stdlib-unsafe-review
description: Privately review unsafe code, safety contracts, and safety notes in Rust's core, alloc, and std. Use for stdlib soundness audits or patch review, including compiler internals and Rust team/UCG decisions. Gives proof obligations and human follow-up, not replacement source or public review prose.
---

# Rust Standard Library Unsafe Review

## Purpose

Review existing unsafe code in Rust's standard-library implementation as a set
of proof obligations. An unsafe API or trait declares obligations for callers or
implementers. An unsafe block, impl, extern boundary, or attribute asserts that
the corresponding obligations hold at that program point.

The review target is soundness: safe, type-correct downstream code must not be
able to trigger undefined behavior without violating an unsafe contract. Logic
errors, panics, leaks, or incorrect results are separate unless they expose a
path to UB or invalidate a safety premise.

A complete review makes every material premise traceable to an accepted Rust
semantic decision, a published contract, an intentional compiler/library
contract, a maintained invariant, a type-system fact, or a local fact. Missing
semantic knowledge becomes an explicit unresolved question for humans, not an
invented rule.

## Scope

Use this skill for source or patches under `library/core`, `library/alloc`, and
`library/std`, including:

- unsafe functions, traits, methods, impls, blocks, extern boundaries, and
  unsafe attributes;
- raw memory, layout, initialization, ownership, pinning, concurrency, FFI,
  inline assembly, SIMD, and target-feature-sensitive code;
- safe APIs, callbacks, trait implementations, macros, generated code, and
  cfg-specific paths that can establish or break an unsafe invariant;
- unstable features, intrinsics, lang items, `rustc_*` attributes, internal
  ABIs, and other compiler/stdlib implementation contracts.

## Private review scope

The deliverable is private analysis for the user: established findings,
contracts, proof gaps, coverage limits, and questions or repair directions for
human judgment. Keep every continuation within that scope. Never offer or
produce source fixes, replacement safety notes or documentation, diagnostic
wording, commit messages, public-ready review or disclosure prose, or issue/PR
text. Do not post, contact others, execute bot actions, or launch shared CI or
Crater work; identify any necessary human follow-up in the advisory.

Read-only source and discussion research, existing tests, and bounded isolated
local checks can support the review. Keep verification artifacts private and
distinct from proposed source changes. A human independently verifies the
analysis, authors changes, and makes project decisions. The
[Rust project LLM usage policy](https://forge.rust-lang.org/policies/llm-usage.html)
provides contribution-policy context; this skill's private scope is a personal
workflow boundary, not a summary of everything that policy permits.

## Review setup

1. Record the exact revision, paths, targets, and feature/cfg scope under
   review, including relevant uncommitted changes. Distinguish the source under
   review from the compiler/sysroot actually used by any experiment.
2. Read repository-local instructions and the live crate attributes, lints, and
   relevant policy before interpreting automated results.
3. Inventory every unsafe surface in scope, including undocumented unsafe
   functions and each `ignore-tidy-undocumented-unsafe` exemption. Include
   macro expansions, generated code, target modules, and cfg branches. For
   broad audits, track reviewed, unresolved, and unreviewed proof families;
   sharing an invariant does not make uninspected sites covered. For patch
   reviews, follow changed invariants into their callers and consumers.
4. Trace the surrounding safe API far enough to determine whether adversarial
   but type-correct safe code can reach the operation or violate an invariant.
5. Read the topic references routed from [Proof audit](#proof-audit) before
   finalizing findings.

Keep potentially security-relevant nonpublic findings out of finding-specific
network queries. General policy lookups can continue without disclosing the
finding. If concrete unsoundness needs severity or rollout assessment, use
`rust-stdlib-unsoundness-response` when available for that private assessment.
Do not expand a narrow audit into an unsolicited ecosystem-wide investigation.

## Standard-library policy

### Safety notes and block size

Under the [standard-library safety-comment
policy](https://std-dev-guide.rust-lang.org/policy/safety-comments.html), unsafe
blocks need safety notes. Require specific notes in `core` and `alloc`; check
the applicable policy for the reviewed parts of `std`. Determine mechanical
coverage, exclusions, and recognized marker spelling from the reviewed
checkout's [tidy check](https://github.com/rust-lang/rust/blob/main/src/tools/tidy/src/style.rs)
and crate lints. Do not freeze today's rollout into a permanent core-only rule.
A differently styled note can contain a useful proof while failing the exact
`// SAFETY:` marker check. Missing enforcement does not waive the review
requirement; distinguish policy, tooling, and proof quality.

A marker is not a proof. Review the note for the exact operation, complete
premises, and their continued validity at the operation. An empty or generic
marker does not satisfy the review. When annotation work is intentionally
deferred, `ignore-tidy-undocumented-unsafe` is lint triage: inspect its scope and
rationale, and keep the unresolved proof obligation visible.

Treat an `unsafe impl` as a separate proof assertion. Require nearby existing
rationale that establishes the unsafe trait's implementer obligations, while
keeping that review criterion distinct from the mechanical coverage found
above. Apply the same distinction to other unsafe declarations under a local
policy: missing proof prose may be a documentation defect even when the current
mechanical check does not cover it.

Keep unsafe blocks small. A block should contain only the tightly coupled
operations covered by its adjacent proof. Separate operations with different
contracts or intervening safe calls, callbacks, or state transitions. If one
block must contain several unsafe operations, the existing note must identify
and discharge each one.

Inspect the crate's `unsafe_op_in_unsafe_fn` setting. The outer unsafe
function's caller contract is only one possible premise for each inner
operation; it does not discharge unrelated obligations.

### Unstable and compiler-internal mechanisms

The standard library may use unstable language features, intrinsics, lang
items, `rustc_*` attributes, target details, and other implementation contracts.
Public stability and downstream availability are not requirements for their
in-tree use.

Classify the reachable API before judging general applicability:

- Stable APIs and tracked nightly APIs must uphold their applicable contracts.
  Nightly availability alone is no exemption from soundness.
- Unstable items without a tracking issue, commonly `issue = "none"`, including
  internal lang items, are generally compiler/stdlib implementation facilities.
  Arbitrary downstream use that enables these internal features is not, by
  itself, a stdlib soundness finding. Confirm that classification from the
  item's gate, visibility, intended callers, and compiler use; a missing issue
  is a signal, not a blanket waiver.
- Manually review every concrete in-tree/compiler use in scope, including
  generated calls and generic instantiations, against the actual internal
  preconditions. An internal gate does not excuse UB exposed through a stable
  or tracked safe API, or a contract-compliant unsafe API call. Record excluded
  arbitrary uses separately from concrete uses whose proof remains open.

For each proof-relevant internal mechanism, establish:

- the precise behavior relied upon, including targets, configurations,
  compiler components, and temporal scope;
- evidence that this is an intentional compiler/library contract rather than
  accidental current behavior;
- a concrete performance or flexibility rationale that justifies the coupling
  instead of a public, documented alternative;
- how the dependency remains visible for coordination and re-audit if the
  compiler changes.

Classify absent performance or flexibility justification as a design and
maintainability concern, not evidence of UB. Existing design rationale can
justify a review premise; measure claims when comparing new alternatives, not
as a ritual benchmark requirement for every established internal mechanism.
Compiler source and tests show current behavior but do
not alone establish a durable contract. Scope internal premises to the
compiler-built `core`, `alloc`, or `std` artifacts; they are not downstream Rust
guarantees.

When the semantics are undocumented, disputed, or too subtle to establish from
accepted decisions and source, state the exact proposition that remains open
and route it to the relevant library, language, compiler, or operational-
semantics humans.

## Soundness authorities and evidence

Use each source only for the propositions it actually establishes.

### Published semantics and public contracts

The [Rust Reference](https://doc.rust-lang.org/reference/) and
[standard-library documentation](https://doc.rust-lang.org/stable/std/) define
published language semantics and public API contracts. The documentation of
the implementation under review defines what must be proved; it does not prove
that the implementation satisfies its own promise. Bottom that proof out in
lower-level operations and independent premises.

### Rust team FCP decisions

A [completed Rust team
FCP](https://github.com/rust-lang/opsem-team/blob/main/fcps.md) that explicitly
declares "we want X to be sound in our memory model" controls that proposition
over a conflicting UCG issue resolution. Verify the exact proposal, completed
disposition, applicable scope, and absence of a later override. An API approval,
stabilization decision, pending FCP, or individual comment does not imply a
memory-model decision. A newer informal discussion does not override a completed
applicable FCP; look for an actual superseding decision.

An FCP records accepted project intent. Also verify that the reviewed compiler
implements that intent and distinguish it from a published stable guarantee. A
conflict between accepted intent, current implementation, and published text is
a human-resolution item rather than an invitation to silently pick one.

### Resolved UCG issues and terminology

For an odd edge case not controlled by an applicable non-overridden FCP, the recorded
resolution of an issue in the
[UCG repository](https://github.com/rust-lang/unsafe-code-guidelines/issues) may
serve as the source of truth. Use terms as defined by the
[UCG glossary](https://rust-lang.github.io/unsafe-code-guidelines/glossary.html).

A label, issue number, open discussion, isolated expert comment, Miri result, or
closed status alone is not a resolution. Identify a clear recorded conclusion,
such as a closing summary or linked accepted decision, and keep its exact scope.
An unresolved issue yields an unresolved finding only if deciding the reviewed
operation actually depends on that open proposition. Use glossary terminology
without promoting its historical examples or draft model statements into new
guarantees.

### Implementation contracts and component lemmas

An intentional compiler/stdlib coupling is an `IMPLEMENTATION CONTRACT`. Name
the exact feature, intrinsic, attribute, ABI, target assumption, or compiler
component and tie revision-sensitive evidence to the reviewed checkout.

Documented behavior of a reviewed lower layer is a `COMPONENT LEMMA`: `alloc`
may rely on `core`, `std` may rely on `core` and `alloc`, and platform modules
may rely on explicit OS, ABI, or runtime contracts. Tie external contracts to
the reviewed version. Component documentation cannot define Rust UB, validity,
layout, aliasing, or provenance outside the Rust authorities above.

### Invariants and local proof facts

An `INVARIANT` is usable only when it is explicit and every constructor,
mutator, destructor, trait impl, FFI boundary, panic path, and relevant cfg
preserves it. Field privacy does not prove an invariant.

A proof may also use an upstream unsafe `PRECONDITION`, a compiler-enforced
`TYPE FACT`, a checked `LOCAL FACT`, or the proved `POSTCONDITION` of an earlier
operation. A fact checked earlier must survive all intervening mutation,
reallocation, aliasing, callback, panic, drop, and interior-mutability effects.

Proof-relevant behavior of a safe helper is a lemma to verify, not a consequence
of the helper being safe. Identify the exact property, inspect its implementation
and callers, and confirm that unknown code cannot invalidate it.

### Advisory evidence

Miri, sanitizers, fuzzing, tests, codegen inspection, current LLVM behavior,
blogs, papers, the Nomicon, draft model text, and unresolved discussions are
useful for discovering bugs and gathering evidence. They do not independently
settle a proof obligation.

Every material premise needs an adequate source. A missing explanation with an
independently established proof is documentation debt; a missing semantic
premise that changes the conclusion is `Soundness unresolved`.

When using experiments, record the tested source/sysroot, toolchain, target,
model/flags, and final result. Separate an actual target execution from an
extracted model or injected failure path. First check that the harness respects
all unsafe contracts and models a permitted event. A Miri model rejection is
evidence to explain against the controlling contract; passing one configuration
does not close other configurations or exceptional paths. Static proof can
establish a defect without a runnable reproduction. Prefer bounded checks of
the disputed invariant; do not build exploits to strengthen a review finding.

## Proof audit

For every unsafe site, answer these questions from the existing code and prose:

1. **Operation:** What exact unsafe operation or boundary is asserted? If a
   block contains several, enumerate all of them.
2. **Contract:** What exact obligations does that operation impose, and which
   authority or implementation contract defines them?
3. **Premises:** Which team decision, UCG resolution, published contract,
   implementation contract, component lemma, precondition, invariant, type
   fact, local fact, or prior postcondition discharges each obligation?
4. **Freshness:** Why does every premise still hold here after intervening code,
   including caller-controlled code, unwinding, reentrancy, mutation, and cfg-
   specific behavior?
5. **State transition:** What ownership, initialization, aliasing, lifetime,
   pinning, synchronization, or drop obligation changes after the operation?
6. **Safe boundary:** Can safe, type-correct downstream use, or an actual safe
   in-tree call path, reach UB without violating an unsafe contract?

Read the references that match the site's proof surface:

- For raw pointers, references, slices, initialization, allocation, layout,
  provenance, transmute, or const-eval representation, read
  [Raw memory and layout](references/raw-memory-and-layout.md).
- For ownership transfer, partial initialization, drop, unwind safety,
  callbacks during broken invariants, or pinning, read
  [Ownership, panic, and pinning](references/ownership-panic-and-pinning.md).
- For generics, safe callbacks, trait laws, sealing, coherence, auto traits,
  variance, or unsafe impls, read
  [Caller code and traits](references/caller-code-and-traits.md).
- For atomics, data races, locks, global state, FFI, inline assembly, target
  features, platform contracts, or cfg matrices, read
  [Concurrency, FFI, and targets](references/concurrency-ffi-and-targets.md).

### Existing `# Safety` contracts

Check that the contract:

- names the responsible caller, implementer, linker, host, or foreign party;
- states every unchecked memory-safety precondition with its temporal scope;
- separates validity, initialization, aliasing, ownership, and semantic
  requirements rather than calling a pointer merely "valid";
- does not place hidden unchecked safety obligations on callers of a safe API;
- does not strengthen an unsafe trait method's caller contract in one impl;
- states proof-relevant postconditions on which later unsafe code may rely.

### Existing safety notes

Check that the note:

- identifies every operation inside the block and its contract;
- maps each obligation to a precise, still-valid premise;
- names any invariant or internal contract rather than gesturing at it;
- accounts for proof-relevant state changes after the operation;
- remains local enough for a maintainer to audit without reconstructing the
  entire module from folklore.

Caller-provided safe code is adversarial but type-correct within all applicable
contracts, including obligations explicitly assigned by unsafe APIs or traits.
Where those contracts permit it, it may panic,
reenter, return any value allowed by its type, violate safe trait laws, mutate
through safe interior-mutability paths, and run surprising destructors. A
reviewed lower layer remains responsible for its documented behavior, but that
trust does not extend to callbacks or trait implementations supplied by callers.

## Findings and output

Use four distinct finding classes:

- **Soundness defect:** an admissible execution reaches UB under an established
  contract. Identify whether it is reachable through a safe API, a compliant
  unsafe API call, or a concrete compiler/in-tree use. A direct unsafe
  precondition violation still needs a reachable state under the applicable
  outer contract; an invented private-helper call is not that evidence.
- **Soundness unresolved:** the conclusion depends on an unsettled,
  undocumented, or conflicting language/compiler contract. State the exact
  proposition requiring human resolution.
- **Policy/documentation defect:** a required note or `# Safety` contract is
  missing, noncanonical for an enforced lint, vague, stale, too remote, or
  unable to cover an unnecessarily broad block.
- **Design/maintainability concern:** an unstable or internal mechanism lacks a
  demonstrated performance or flexibility justification, or its coupling is
  too hidden to re-audit reliably.

Report ordinary compile errors, test failures, integration defects, and stale
attributes separately as build or compatibility results. Do not force them into
the soundness taxonomy merely because they occur near unsafe code. Treat a
published-documentation/implementation mismatch as a documentation defect
unless it leaves a material soundness premise unresolved; then report both the
mismatch and the exact unresolved proposition.

Lead with findings in that order. Each finding includes:

- path, line, and exact unsafe operation or boundary;
- applicable contract and source;
- the false, missing, stale, or unresolved premise;
- for a soundness defect, an admissible path to the first established unsafe
  contract violation and its public, tracked-nightly, or concrete internal scope;
- affected targets/configurations and the smallest useful human follow-up.

Report review coverage and material exclusions. A present marker, passing tidy,
or passing Miri is not a soundness conclusion. A raw-pointer field, unusual
internal attribute, or manual `Sync` impl is not a defect without the missing
obligation or safe UB path. Keep confirmed bugs, unresolved semantics, policy
failures, and design concerns separate.

Scale the report to the requested scope. Report what was proved, what remains
conditional, and what was not reviewed; do not imply a complete audit from an
inventory or from a few representative sites. Technical severity and project
priority require their own evidence, not a fixed label attached to every UB.

The output is a private advisory analysis. Give humans the operations, evidence,
and proof gaps they need to resolve; leave final source and public review prose
to them.
