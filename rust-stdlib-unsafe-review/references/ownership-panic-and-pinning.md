# Ownership, panic, and pinning review

Use this checklist when unsafe code changes initialization, ownership, drop
responsibility, or pinning, or when callbacks and unwinding can observe an
intermediate state.

## Explicit state machines

Model each allocation or slot with explicit states such as unallocated,
allocated-uninitialized, partially initialized, initialized-owned, moved-out,
and deallocated. Review every normal, error, panic, and drop edge between them.
Follow the full interval during which another operation depends on an invariant,
including after publication and before cleanup completes. Include cancellation
or early destruction where the abstraction permits them; reviewing only the
operation that first changes the state misses later exceptional exits.

For a partially initialized collection or array, establish:

- the exact initialized prefix or set after each operation;
- which guard or owner drops that set on unwind;
- that length and discriminant metadata never claim more initialization than
  exists;
- that cleanup drops each initialized value at most once and never drops an
  uninitialized slot;
- that a panic may leak resources but cannot expose invalid typed values,
  double-drop, or later use-after-free.

## Reads, writes, copies, and moves

Unsafe byte operations do not automatically perform Rust ownership transitions.
Trace their logical effects:

- A raw read of non-`Copy` `T` creates a second bitwise instance. The original
  storage must become logically uninitialized, be overwritten without dropping,
  or otherwise be prevented from later owned use.
- A raw write does not drop the old destination value. Establish whether the old
  value was uninitialized, moved out, deliberately leaked, or separately
  disposed of.
- A copy can duplicate bytes without duplicating logical ownership rights.
  Identify which copy, if any, becomes owned and which storage must no longer be
  treated as initialized `T`.
- `set_len`, `assume_init`, and equivalent internal metadata changes expose
  storage under stronger validity and drop contracts; prove the entire exposed
  region first.

Review postconditions in later code as carefully as preconditions at the unsafe
operation.

## Ownership transfer and `ManuallyDrop`

For manual extraction or reconstruction of raw parts, verify the ordering that
prevents two live automatic destructors from owning the same resource. A
consuming standard-library operation is easier to audit when available.

When an old owner is wrapped in `ManuallyDrop` or otherwise disabled, inspect:

- the last point at which the old value is valid to read or move;
- whether any field access after replacement owner creation uses an invalidated
  owner;
- panic points between disabling one owner and creating another;
- whether failure can leak safely without restoring two owners;
- allocator, capacity, and layout facts transferred to the new owner.

`ManuallyDrop<T>` suppresses automatic drop; it does not make arbitrary access
to a moved-out or invalid `T` sound.

## Destructors and leaks

Soundness cannot depend on the destructor of a value exposed to safe code
definitely running. Safe code can use `mem::forget`, leak a guard, create a
reference cycle, or abort before cleanup. A private local guard that cannot
escape may be part of an unwind proof; verify that safe code cannot forget or
leak it and that abort needs no continuing invariant. Review whether a missing
destructor merely leaks or instead leaves later safe access to invalid state.

Unknown `Drop` implementations are caller-controlled safe code. They may panic,
reenter the abstraction, mutate global or interior state, inspect other values,
or run at surprising points. Any invariant required while dropping `T` must hold
before invoking its destructor.

For custom `Drop`, `#[may_dangle]`, raw-pointer ownership, `MaybeUninit`, or
`ManuallyDrop`, also review:

- whether `PhantomData` accurately represents ownership and lifetimes to
  drop-check;
- whether fields that may be dropped remain live for the destructor;
- whether a value can observe another field after that field has been dropped;
- drop order across fields, arrays, tuples, and partial initialization;
- every cfg-specific destructor implementation.

## Panic and unwind boundaries

Review unknown safe code for every contract-permitted panic and reentrancy path.
This includes callbacks, closures, comparators, iterators, allocators, trait
methods, formatting, cloning, dropping, and code transitively invoked by a
reviewed component. An unsafe contract may constrain otherwise safe methods;
do not posit behavior that violates that contract. In particular, distinguish a
forbidden allocator unwind from an allowed allocation error and its subsequent
handling. A non-unwinding allocator method does not by itself prove that the
whole allocation path cannot unwind.

For unwinding at each call, establish one of:

- all externally observable invariants are restored;
- a guard owns enough state to restore or safely abandon the transition during
  unwind;
- the broken state is unreachable to safe code and cleanup can only leak;
- an applicable unsafe contract excludes the unwind, or a verified runtime
  boundary contains it before it can expose invalid state. A caller obligation
  need not be dynamically checked, but it must actually govern this invocation.

Poisoning is advisory state, not a universal unwind guard. If memory safety
depends on a lock-protected invariant, prove what happens when a panic occurs at
every point, including cases where poisoning is not set or is ignored.

## Reentrancy

Before calling unknown code, identify every capability it receives or can reach:
references, global handles, thread locals, callbacks, shared containers, and
interior-mutability paths. Review whether reentry can observe a temporarily
incorrect length, a moved-out value, an invalid reference, duplicate ownership,
or partially initialized storage.

A local mutable borrow does not prove non-reentrancy when unknown code can reach
the same logical object through another safe handle.

## Pinning

Pinning protects the pointee according to the applicable pin contract; it is not
limited to the lifetime of a temporary `Pin` wrapper. Identify:

- the exact pointee and why it is currently pinned;
- the event that ends the pinning obligation, commonly destruction of the
  pointee rather than dropping a borrowed `Pin<&mut T>` handle;
- whether projection exposes only fields that may be independently pinned or
  moved;
- whether replacement, swap, deallocation, or ownership reconstruction can move
  a `!Unpin` value;
- whether pinned drop code observes all structural invariants;
- whether panic or leak paths preserve the no-move guarantee for every live
  pinned value.

For `Pin::new_unchecked`, unchecked projection, or internal pin constructors,
extract the concrete pointer type's `Deref`, `DerefMut`, and `Drop` behavior.
An unsafe constructor may place obligations on those otherwise safe impls; prove
them for the actual closed implementation set rather than assuming arbitrary
safe trait implementations are honest.

## Review result

A confirmed finding identifies the exact invalid state transition and how safe
code reaches a read, drop, deallocation, move, or callback while that state is
observable. When the result depends on an unsettled panic, pinning, or validity
contract, keep it in the unresolved class and frame the missing proposition for
human review.

When supplied evidence uses an injected failure or a substitute harness, state
which transition it establishes and whether that transition is permitted and
reachable on the actual target. An injected trace alone does not establish a
real runtime path.
