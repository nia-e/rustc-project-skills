---
name: rust-stdlib-unsoundness-response
description: "Privately triage a suspected Rust core, alloc, or std soundness defect: validate its premises and assess severity, exploitability, exposure, or repair and rollout tradeoffs. Use after a concrete finding is proposed, not for broad discovery, exploit development, or public/source authoring."
---

# Rust Standard Library Unsoundness Response

## Purpose

Turn a potential standard-library unsoundness into a private, evidence-backed
response recommendation. Establish whether the defect is real, which releases
and configurations it affects, how severe and exploitable it is, how often its
trigger is likely to occur, and which complete repair causes the least ecosystem
disruption without needlessly sacrificing performance or flexibility.

This work begins after a concrete defect has been proposed. Validate the
finding far enough to support triage, but do not use this skill as a broad
unsafe-code audit. A logic error, panic, leak, incorrect result, or documented
unsafe caller violation is not a standard-library soundness defect merely
because unsafe code is nearby.

Match the depth to the request. A severity question needs finding validation
and risk calibration, not an automatic ecosystem study or complete rollout
plan. Read the references for the dimensions being assessed; name material
unknowns without manufacturing work or stalling the useful assessment.

The result is a decision aid for Rust humans. It is not a team decision, a
security classification, or merge approval.

## Private advisory scope

The deliverable is a private assessment for the user. Describe repair designs
through their invariants, algorithmic shape, tradeoffs, migration requirements,
and verification needs. A human independently verifies the finding and authors
the repair. Keep every continuation advisory: never offer or produce source
fixes, replacement safety notes or documentation, diagnostic wording, commit
messages, public-ready reviews or disclosures, or issue/PR text. Do not post,
contact others, execute bot actions, or launch shared CI or Crater work.

Read-only source and discussion research, existing tests, and bounded isolated
local checks can support the assessment. Keep verification artifacts private
and distinct from proposed source changes. Recommend any wider experiments,
coordination, or rollout decisions as work for the responsible humans. The
[Rust project LLM usage policy](https://forge.rust-lang.org/policies/llm-usage.html)
provides contribution-policy context; this skill's private scope is a personal
workflow boundary, not a summary of everything that policy permits.

## Confidentiality

Before a finding-specific network query, establish that its details are already
public. A public report does not make a newly found trigger, affected release,
or repair detail public. If disclosure status is unknown, continue locally and
use general policy or source queries that disclose none of the finding.

If a nonpublic report plausibly crosses a security boundary, keep its details
local. Do not send the trigger, affected symbols, patch shape, or reproducer to
public search engines or third-party services. The
[Rust security policy](https://rust-lang.org/policies/security/) covers the
Rust distribution and directs potential vulnerabilities to the Security
Response team. Identify that coordination need in the private assessment for
the responsible human.

A soundness issue is not automatically a security vulnerability. Assess
attacker control, trust boundaries, consequences, and deployment reality, then
leave the final security disposition, embargo, backports, and release timing to
the Security Response team. Prefer source/contract analysis and bounded
invariant checks; do not develop exploitation techniques or escalate testing
merely to obtain a more dramatic severity claim.

## Governing project sources

Use live project guidance for current operational details:

- The [current breaking-change procedure](https://rustc-dev-guide.rust-lang.org/bug-fix-procedure.html)
  is the maintained implementation guidance derived from
  [RFC 1589](https://github.com/rust-lang/rfcs/blob/master/text/1589-rustc-bug-fix-procedure.md).
- [RFC 1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)
  and the [standard-library maintenance guide](https://forge.rust-lang.org/libs/maintaining-std.html)
  govern library compatibility concerns.
- The [standard-library review policy](https://std-dev-guide.rust-lang.org/team/reviewing.html)
  requires a finished FCP for changes to public API or stable documented
  promises.
- The [compiler prioritization guide](https://forge.rust-lang.org/compiler/prioritization.html)
  defines compiler-team `P-*` labels. Those labels describe response urgency
  and resource allocation, not intrinsic technical severity.
- The [Rust security policy](https://rust-lang.org/policies/security/) controls
  confidential vulnerability handling.

RFC 1589 applies directly when a compiler correction will reject source that
previously compiled. Apply its principles—measure impact, track the change,
provide a clear migration, prefer staged warnings, and help affected crates—to
standard-library fixes where they fit. Do not invent a future-incompatibility
lint for a runtime-only implementation repair. The guide's “fewer than 10
affected projects” language is a rule of thumb for direct compiler errors, not
a severity threshold or automatic permission.

For soundness premises, use the Rust Reference and standard-library contracts,
applicable completed non-overridden Rust team FCPs, clearly recorded resolutions
of relevant [UCG issues](https://github.com/rust-lang/unsafe-code-guidelines/issues)
with terms from the [UCG glossary](https://rust-lang.github.io/unsafe-code-guidelines/glossary.html),
and intentional compiler/stdlib implementation contracts. Keep each source to
its exact scope. An applicable completed, non-overridden team FCP that explicitly
decides “we want X to be sound in our memory model” controls that proposition
over a conflicting UCG issue resolution. Verify completion, scope, and the
absence of a later override; a newer informal discussion does not override it.
Distinguish accepted intent from a published stable guarantee and
from proof that the current compiler implements it. Tests, Miri, compiler
source, and optimizer behavior are evidence, not independent semantic
authority. If these sources do not settle a proof-relevant proposition, mark it
unresolved and identify the decision needed from the relevant human team rather
than selecting the semantics most convenient for triage.

## Assessment workflow

### 1. Establish the finding

First classify reachability. Stable and tracked nightly APIs must uphold their
applicable contracts. Unstable items without tracking issues, commonly
`issue = "none"`, including internal lang items, are generally implementation
facilities: arbitrary downstream use of those gates alone does not establish
a stdlib defect. Confirm the intended use and manually assess concrete
compiler/in-tree callers. Retain findings exposed through ordinary safe APIs,
tracked features, or compliant unsafe calls even when an internal facility is
the failing lower layer. An absent tracking issue is not a blanket exemption.

Record:

- exact revision, local changes, and source location;
- verified affected channels and release bounds, separating the first observed
  failure from an established introduction or fix;
- targets, cfgs, features, panic strategies, optimization modes, allocators,
  and const/runtime contexts that matter;
- the published API contract and any applicable unsafe caller preconditions;
- the admissible safe API path, compliant unsafe API call, or concrete
  compiler/in-tree use whose implementation violates its contract;
- the exact UB mechanism and the first operation whose preconditions fail.

Use one of these statuses:

- **Confirmed:** the path and controlling semantic premises are established.
- **Conditional:** the path is valid if a precisely stated unresolved premise
  is accepted.
- **Unresolved:** the memory-model, compiler, platform, or API premise needed
  to decide the finding is not established.
- **Not established by this report:** evidence does not yet demonstrate the
  claimed path, reachability, or affected configuration. This is not a finding
  that the implementation is sound.
- **Not a stdlib soundness defect:** the reproducer violates an unsafe
  contract, demonstrates only a non-soundness bug, or the proposed path is
  disproved. State which claim was ruled out rather than clearing the whole API.

A complete source proof can establish the path without executing UB. For
supplied experiments, identify the source/sysroot actually tested, toolchain,
target, Miri model/flags where relevant, and final result. Distinguish an
unmodified target run from an extracted model or injected failure path. Verify
that test assumptions respect unsafe contracts and that the modeled event is
permitted in the real path. A failed test setup is not a negative bug result;
a passing model is not a universal soundness proof.

Do not assign a categorical severity to an unresolved semantic question. A
conditional risk analysis may explain the stakes while naming the premise.
If revisions, experiments, or deployment evidence are missing, continue with a
clearly provisional assessment, list the missing evidence, and say which
conclusions it could change. Do not fabricate precision or stop useful triage
merely because the initial report is incomplete.

### 2. Assess severity and exploitability

Read [Severity and exploitability](references/severity-and-exploitability.md).
Report technical consequence, safe reachability, practical exploitability,
affected scope, and confidence separately before giving an overall qualitative
severity. Never infer code execution merely from UB, or non-exploitability from
the absence of a known exploit.

Keep these judgments distinct:

- severity of the confirmed defect;
- likelihood and impact of practical exploitation;
- project response urgency or a provisional team priority;
- confidence in the available evidence.

### 3. Estimate real-world exposure and breakage

Read [Ecosystem impact](references/ecosystem-impact.md). Estimate how many
programs contain the affected API or pattern, how many meet the trigger
conditions, how often the runtime state occurs, and how many deployed artifacts
remain affected. Report a range or qualitative bound when a point estimate
would be fiction.

Separate exposure to the bug from disruption caused by the fix. Code search,
dependency counts, Crater, tests, issue reports, and telemetry answer different
questions; none is a universal prevalence measure.

### 4. Compare complete repairs

Read [Remediation and migration](references/remediation-and-migration.md).
Generate materially different candidates when the trade space warrants it.
For each candidate, establish that it closes every known safe path to UB, then
compare:

- stable API and contract compatibility;
- source, behavior, layout, ABI, trait, inference, const, target, and cfg
  breakage;
- migration clarity and overlap across toolchain versions;
- runtime and compile-time performance, allocations, code size, and hot-path
  effects;
- retained genericity, target support, `no_std` usability, and implementation
  flexibility;
- auditability, implementation risk, backport suitability, and time to deploy.

When urgency and long-term design point in different directions, compare a
complete, readily backportable containment with the durable least-breaking
repair instead of forcing one patch to serve both purposes.

“Minimally breaking” means the least disruptive complete repair, not the
smallest diff. A patch that fixes one reproducer while leaving an equivalent
path open is not a candidate. Documentation cannot retroactively transfer an
unchecked safety obligation to callers of a stable safe API.

Prefer an internal implementation repair that preserves the stable contract
and established behavior. If that is impossible, make the tradeoff explicit
and route stable contract or API changes to the relevant library humans. Do not
claim a performance or flexibility result without relevant measurements.

### 5. Recommend rollout and migration

Describe the work and decisions required from human owners, matching the
rollout to both risk and breakage:

- A security-relevant issue, whether public or private, may require coordinated
  fixes, backports, a point release, or immediate enforcement; Security
  Response owns that decision. Public disclosure removes the embargo question,
  not the urgency.
- A source-rejecting compiler correction normally needs a tracking issue,
  Crater evidence, a precise human-authored diagnostic and migration, and a
  future-compatibility warning for at least one release cycle when feasible and
  when leaving the hole temporarily open is acceptable. A warning is migration
  machinery, not remediation. Compare that delay with immediate enforcement
  and have the responsible team approve an exception when urgency wins. Follow
  the live compiler or language approval process for introducing the warning.
- A runtime-only implementation correction may need no source migration. It
  still needs affected-version identification, regression tests, release and
  backport judgment, and clear rebuild implications for already produced
  artifacts.
- If warnings are infeasible, assess targeted diagnostics, narrow rejection,
  affected-crate migration, and staged layering before recommending an
  immediate hard break.

When a runtime containment and a staged compiler restriction are paired, keep
the containment on every channel that still accepts the bad pattern. Remove it
only after the enforced restriction is proved complete for that channel.

Identify who must decide unresolved points: Security Response, `T-libs`,
`T-libs-api`, compiler, language, operational-semantics, release, or target
maintainers. The recommendation should give the user enough evidence to pursue
those decisions independently.

## Output

Produce a compact private assessment using the relevant parts of this outline:

1. **Finding status:** exact path to UB, semantic premises, affected scope, and
   confidence.
2. **Severity:** technical impact, safe reachability, exploitability,
   prevalence, overall qualitative severity, and provisional response urgency.
3. **Security handling:** public/private status and whether Security Response
   coordination is indicated.
4. **Ecosystem evidence:** validated affected uses, evidence limits, and likely
   fix-induced breakage.
5. **Fix candidates:** a comparison in which incomplete repairs are rejected,
   not merely scored poorly.
6. **Recommendation:** the least disruptive complete repair, why it wins, and
   what human decision remains.
7. **Migration and rollout:** warnings or diagnostics where applicable,
   tracking, affected-crate work, release/backport/rebuild implications, and
   rollback criteria.
8. **Verification:** soundness regression coverage, cfg/target checks,
   ecosystem experiments, and performance/flexibility measurements still
   required.

Do not force all eight sections onto a narrow question. Label facts as
demonstrated, inferred, or unknown, separating source proof from measured
behavior and stating confidence per conclusion. Cite exact revisions,
experiment reports, and current project decisions. Do not disguise missing
evidence with a numeric score or present a provisional label as team consensus.
Identify the missing evidence that could change the recommendation. Confidence
in a soundness proof need not imply confidence in attacker control, historical
release range, or deployment prevalence.
