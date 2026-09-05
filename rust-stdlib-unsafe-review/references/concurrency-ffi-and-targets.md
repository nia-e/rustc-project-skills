# Concurrency, FFI, and Targets

Use this checklist when an unsafe proof crosses threads, global state, a foreign
boundary, inline assembly, target features, platform APIs, or configuration-
specific code. Apply only the sections relevant to the reviewed site.

## Concurrency and the memory model

For a suspected data race or ordering defect, name the concrete conflicting
accesses. Establish:

- the memory locations and byte ranges accessed;
- whether each access is atomic or non-atomic and whether at least one writes;
- which threads, signal handlers, callbacks, or reentrant invocations perform
  them;
- whether their lifetimes can overlap;
- the exact synchronization or happens-before edges claimed to order them;
- the authority defining the relevant atomic, aliasing, and race semantics.

Do not treat the presence of an atomic, mutex, or lock guard as a proof. Check
that the same synchronization discipline covers every access, initialization,
publication, replacement, and destruction path. Account for relaxed ordering,
failed compare-exchange orderings, mixed atomic and non-atomic access,
overlapping or mixed-size atomics, and target-specific atomic support.

For lock-free code, reconstruct the state machine and linearization points.
Trace pointer provenance, object lifetime, reclamation, ABA defenses, reference
counts, and every ordering edge on each success and failure path. A test that
observes the intended interleaving is evidence, not a memory-model proof.

Lock poisoning, panic suppression, or a process-wide convention is not a
memory-safety boundary unless an accepted contract says so. Determine what
state is visible after panic and whether safe code can continue using it.

## Global and process state

Assume mutable global, thread-local, environment, locale, signal, allocator,
panic-hook, and runtime state may be accessed concurrently or reentrantly unless
an explicit contract excludes that behavior.

Check:

- initialization and publication before first use;
- mutation, replacement, teardown, and process-exit paths;
- callbacks or destructors invoked while locks or partial state are live;
- signal- or cancellation-safety where applicable;
- fork, dynamic loading, and runtime shutdown behavior where supported;
- whether the proof accidentally relies on one test process or current startup
  order.

## Foreign-function boundaries

At each FFI boundary, identify the contract on both sides. Verify:

- ABI and calling convention, including variadics and target-specific ABI
  variants;
- representation, alignment, padding, discriminants, niches, and value
  validity for every exchanged type;
- pointer nullability, provenance, alignment, initialization, aliasing, and
  accessible byte range;
- ownership transfer, allocation domain, deallocation API, and who may retain
  each pointer or handle;
- lifetimes of borrowed buffers, strings, callbacks, contexts, and error data;
- thread affinity, synchronization, and callback concurrency;
- unwind behavior in both directions;
- error signaling, partial success, output initialization, and cleanup;
- foreign mutation of memory Rust currently references.

`repr(C)` or `repr(transparent)` answers only the layout questions it actually
guarantees. It does not prove foreign validity, ownership, lifetime, or semantic
requirements. Likewise, a bindgen declaration shows an interface shape; match
it against the supported version of the external header, ABI, and library
contract.

If foreign code calls back into Rust, review the callback as an independent
entry point. Establish what Rust state exists at that moment, which thread may
invoke it, whether it can reenter, how panics are contained, and how long the
foreign side may retain its function and context pointers.

## Platform and runtime contracts

For an OS, libc, allocator, loader, unwinder, or runtime dependency, record:

- the exact supported platform and version scope;
- the primary specification or in-tree platform contract relied upon;
- initialization, thread, signal, and teardown restrictions;
- ownership and lifetime of returned handles and memory;
- errno or last-error preservation rules when proof-relevant;
- behavior on partial failure, interruption, cancellation, or reentrancy.

Do not generalize behavior observed on one platform to another cfg branch. An
undocumented platform or compiler behavior needed for soundness is an explicit
human-resolution item.

## Target features, SIMD, and inline assembly

For a target-feature-sensitive operation, verify both compilation and runtime
requirements. Determine:

- which feature set the function body is compiled for;
- how every call site establishes that the executing CPU supports it;
- whether inlining, monomorphization, function pointers, or cross-crate codegen
  can move the operation across a feature boundary;
- which fallback handles unsupported hardware;
- whether the ABI changes when vector or target-specific types cross a call
  boundary.

For SIMD and architecture intrinsics, check lane count and type, alignment,
immediate-operand restrictions, masked-lane behavior, and the exact machine
instruction semantics on every supported target.

For inline assembly, audit:

- register classes, operand widths, subregister use, and initialized inputs;
- all clobbered registers, flags, memory, and architectural state;
- pointer validity and aliasing for memory accessed by the assembly;
- stack alignment, red zones, calling convention, and reserved registers;
- `options(...)` claims such as `nomem`, `readonly`, `pure`, `nostack`, and
  `preserves_flags`;
- control flow, labels, divergence, unwinding, and exception behavior;
- compiler reordering permitted by the declared operands and options.

Assembly inspection and successful execution on one CPU establish current
codegen evidence, not the full Rust or architecture contract.

## Configuration matrix

List the configurations that can change the proof, including:

- target architecture, OS, environment, pointer width, endianness, and atomic
  capabilities;
- debug and optimized builds, overflow checks, panic strategy, LTO, and codegen
  units;
- crate features, bootstrap/compiler stages, sanitizer and Miri cfgs, tests,
  and internal `cfg` flags;
- allocator choice, global hooks, dynamic versus static linking, and platform
  library versions;
- generated code and macro expansion differences.

Review every supported branch or record it as an exclusion. A branch being
unbuilt in the local environment does not make it irrelevant. Use cross-target
checking, codegen inspection, emulation, or hardware tests as evidence where
available, while keeping semantic proof obligations separate.

## Review result

Record separately:

- an established data race, invalid foreign exchange, target-feature violation,
  or inline-assembly contract breach;
- a missing synchronization, ABI, ownership, unwind, or configuration premise;
- a compiler, platform, or operational-semantics proposition that requires a
  human decision;
- configurations examined, evidence run, and material branches not reviewed.

Passing concurrency tests, Miri, sanitizers, a platform smoke test, or codegen
inspection can support a conclusion but cannot replace the contract-level
argument.
