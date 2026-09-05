# Ecosystem impact

Estimate two different populations:

1. programs exposed to the defect; and
2. programs disrupted by each proposed fix.

They often differ. An implementation-only repair can affect many deployed
programs while breaking no source. A compiler rejection can break many builds
for a pattern that rarely reaches UB at runtime.

## Exposure funnel

Build the estimate as a funnel rather than a raw hit count:

1. **Surface adoption:** projects or artifacts using the relevant API, trait,
   type, intrinsic, or behavior.
2. **Semantic pattern:** uses that satisfy the structural preconditions for the
   bug, after eliminating examples, tests, dead code, vendored copies,
   duplicates, and unrelated symbols.
3. **Configuration:** affected versions, targets, cfgs, profiles, features,
   panic strategies, allocators, and monomorphizations.
4. **Runtime trigger:** programs that can reach the required state, input,
   timing, callback, layout, or concurrency.
5. **Deployment:** affected artifacts actually built and deployed with a
   vulnerable toolchain or standard library.

Give a lower bound, upper bound, or qualitative range when the sample cannot
support a point estimate. Private code and underrepresented targets mean that
zero public hits is a lower bound, not proof of zero use.

Separate release bounds established by source history or inspected artifacts
from those inferred from similar code. The first failing tested toolchain need
not be the first affected release, and the first passing one need not contain a
complete repair. Trace introduction and repair changes, branch backports,
target-specific implementations, and relevant compiler dependencies before
claiming a contiguous affected range. Record unexamined releases as unknown or
inferred rather than silently including or excluding them.

## Evidence sources and limits

### Code search

Use exact API and semantic-pattern searches, then inspect and deduplicate the
results. Textual matches are an inventory. They do not prove compilation,
reachability, affected versions, or runtime triggering. Search generated code,
macros, reexports, wrapper APIs, and equivalent spellings when material.

Keep nonpublic finding-specific terms out of network queries. Search existing
public facts without disclosing private extensions to a known report.

### Crater

[Crater](https://github.com/rust-lang/crater) compares custom toolchains across
crates.io and public GitHub projects by building and testing them. It is strong
evidence for source, diagnostic, and test-suite fallout from a proposed change.
It normally does not measure data-dependent runtime prevalence, production
traffic, private code, or platforms/configurations absent from the experiment.
Record the actual experiment host, targets, modes, features, and configuration
before generalizing results to Windows, embedded, wasm, or other target-heavy
bugs.

For the current breaking-change guide's “fewer than 10” rule of thumb, count
total validated affected projects, not root-error groups or duplicate and
cascading diagnostics. Triage regressions and identify whether each project is
actually affected by the change. The threshold is an option for considering a
direct compiler error, not a risk score, and it does not remove the tracking,
diagnostic, migration, or outreach obligations.

Analyze supplied or published Crater reports. If new evidence is needed,
describe the experiment and its decision value for human operators; do not
request or launch it. Crater executes untrusted third-party code and requires
appropriate isolation.

### Dependency and popularity data

Reverse-dependency counts, downloads, repository stars, and use by foundational
crates can identify potentially important downstream reach. They do not provide
a count of vulnerable users, deployed binaries, or runtime triggers. Weight a
validated use in a widely depended-on crate more heavily than an isolated leaf,
while avoiding double-counting its dependants as independently observed
triggers.

### Reports, tests, and measurements

Existing bug reports, downstream workarounds, fuzzing corpora, telemetry, crash
signatures, and production evidence can validate real execution. State their
sampling and privacy limits. A lack of reports is weak evidence for UB because
optimization, allocator behavior, and concurrency can hide or misattribute it.

## Fix-induced breakage

For each repair, inspect at least:

- source compilation and inference;
- documented behavior, panic behavior, and error values;
- stable signatures, trait impls, auto traits, variance, coherence, and drop
  behavior;
- public layout, ABI, niches, alignment, and `repr` guarantees;
- const evaluation and const-stability behavior;
- target, cfg, feature, allocator, and `no_std` coverage;
- runtime performance, compile time, code size, and allocations;
- already-built binaries and whether users must upgrade, recompile, or change
  source.

Report the migration unit: a call site, crate, reverse dependency, binary,
toolchain, target image, or deployed service. Source-compatible does not mean
deployment-compatible, and a toolchain update does not retroactively repair an
old binary.

## Evidence summary

Conclude with:

- total validated affected projects and important dependants;
- likely but unvalidated uses;
- important blind spots, especially private or target-specific code;
- expected source/test fallout for each fix candidate;
- confidence in both bug exposure and migration cost.

Do not collapse inventory, demonstrated use, and inferred deployment into one
number.
