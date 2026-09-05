# Caller Code and Traits

Use this checklist when an unsafe proof depends on generic code, a callback, a
trait implementation, sealing, coherence, variance, or an unsafe impl. Apply
only the sections relevant to the reviewed site.

## Caller-controlled safe code

Treat safe caller-provided code as adversarial within the type system and any
applicable unsafe contracts. Such contracts can constrain otherwise safe
methods; a safe trait law alone cannot. Within those bounds it may:

- panic or unwind at any permitted point;
- reenter the abstraction through callbacks, destructors, or global hooks;
- return any value allowed by its type;
- mutate through `Cell`, `RefCell`, atomics, locks, or other safe
  interior-mutability mechanisms;
- run surprising `Drop` implementations;
- violate the behavioral laws of a safe trait.

Identify every callback, iterator, future, closure, destructor, formatter,
allocator, comparator, hasher, or safe trait method invoked while an unsafe
premise is live. Check what state is exposed if that call panics or reenters and
whether the premise can become stale before the next unsafe operation.

Safe trait laws are ordinarily correctness contracts, not unchecked
memory-safety obligations. An unsafe implementation may not assume, merely
because a trait is safe, that:

- `size_hint` or `ExactSizeIterator::len` is accurate;
- `Ord`, `Eq`, or `Hash` implementations are consistent;
- `Clone` preserves identity, provenance, address, or hidden invariants;
- a callback is deterministic, non-panicking, non-reentrant, or free of side
  effects.

A behavioral property is usable in a soundness proof only when an unsafe
contract assigns responsibility for it, the implementation dynamically checks
it before relying on it, the type system enforces it, or the complete set of
possible implementations is closed and reviewed.

## Sealing and closed implementation sets

When a proof relies on outsiders being unable to implement a trait or construct
a witness, verify the mechanism rather than the intention.

Check whether downstream code can:

- name all required types, traits, associated items, or tokens;
- obtain a supposedly private witness through a public constructor, re-export,
  default, blanket impl, macro, derive, or associated-type projection;
- implement a public trait through a blanket or fundamental-type interaction;
- reach a cfg- or feature-specific implementation path;
- exploit an implementation supplied by another standard-library crate.

Then review every in-tree implementation and every way the set can grow. A
private module, a conventional `sealed` name, or the current absence of a
downstream impl is not by itself a proof.

If soundness relies on coherence or the orphan rules, state the exact theorem
being used and verify it for the actual trait, self type, type parameters, and
fundamental wrappers. Treat an absence established only by today's impl
inventory as revision-sensitive and require re-audit when the impl surface
changes.

## Unsafe traits and unsafe impls

For an unsafe trait, identify every obligation assigned to implementers and
every unsafe operation that later relies on it. Check that:

- the existing `# Safety` contract is sufficient for all current consumers;
- each unsafe impl establishes the obligation for every permitted value and
  configuration;
- safe trait methods do not provide a route to violate the invariant;
- default methods, associated constants and types, blanket impls, and
  supertraits preserve the contract;
- implementers are not required to infer hidden obligations from current
  implementation details.

For an `unsafe impl`, establish the actual memory-safety invariant being
asserted. The unusual impl or field type is only an inventory signal. For
example, a raw-pointer field or a manual `Send` or `Sync` impl is not itself
proof of a defect. Trace a safe path that can create invalid aliasing, a data
race, use-after-free, invalid access, or another established form of UB.

An implementation of an unsafe trait may impose stronger behavioral
requirements for its own safe methods only when the trait contract permits
that. One impl may not silently strengthen an unsafe trait method's caller
preconditions and then rely on those hidden requirements for soundness.

## Auto traits and concurrency exposure

`Send` and `Sync` are unsafe auto traits. `Unpin`, `UnwindSafe`, and
`RefUnwindSafe` are safe auto traits with different roles; unwind-safety markers
are not memory-safety guarantees. For `Send` and `Sync`, inspect the abstraction
rather than merely its fields.

Trace:

- which values, references, guards, handles, or pointers can cross threads;
- which safe methods can execute concurrently and what storage they touch;
- lifetime and ownership of pointees, handles, callbacks, and foreign state;
- whether mutation is synchronized for the exact accesses performed;
- whether destruction can race with access or callback execution;
- whether a negative impl or marker field is part of an intentional contract.

Use [Concurrency, FFI, and targets](concurrency-ffi-and-targets.md) for the
memory-model and synchronization proof, and
[Ownership, panic, and pinning](ownership-panic-and-pinning.md) for `Unpin` and
unwind-sensitive invariants.

## Variance, ownership markers, and drop checking

When a type contains raw pointers, `NonNull`, function pointers,
`PhantomData`, or erased lifetimes, determine its actual variance and ownership
semantics. Check that they agree with every safe constructor and conversion.

In particular, verify:

- covariance, contravariance, or invariance for each lifetime and type
  parameter;
- whether the type logically owns, borrows, or merely observes a `T`;
- whether drop checking must assume access to a `T` during destruction;
- whether auto-trait derivation matches the data that can actually be touched;
- whether lifetime erasure can let a safe reference or handle outlive its
  referent.

Do not infer these facts from a `PhantomData` spelling alone. Follow the current
language definition and the type's constructors, methods, and destructor.

## Safe APIs and internal helpers

A safe public API must not require downstream callers to uphold an unchecked
memory-safety condition. If the implementation depends on one, identify which
of these makes it safe:

- the type system prevents invalid inputs;
- the implementation checks the condition before the unsafe operation;
- a maintained invariant established by all constructors and mutators implies
  it;
- the obligation belongs to an earlier explicit unsafe boundary.

For a crate-visible or private safe helper used inside an unsafe proof, inspect
its complete call set and the property later consumers rely on. A hidden
precondition is an auditability concern; it establishes a soundness defect only
when an actual safe in-tree path can violate it and reach UB. The useful review
question is whether its contract and callers keep the property true, not
whether the helper bears a particular syntactic marker.

## Review result

Record separately:

- an established safe path to UB;
- a missing or false trait/auto-trait premise;
- a reliance on a safe behavioral law that cannot support memory safety;
- an unresolved coherence, variance, or language-semantics proposition;
- a maintainability risk whose correctness depends on a closed and evolving
  implementation set.

Do not promote an odd impl, a raw pointer, or a violated behavioral law into a
soundness finding without completing the path to an established unsafe
contract violation.
