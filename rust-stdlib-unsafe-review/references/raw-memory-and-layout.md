# Raw memory and layout review

Use this checklist when an unsafe proof touches pointers, references, slices,
initialization, allocation, representation, provenance, transmute, or const-eval
layout. Apply the authority rules from the parent skill to every model-sensitive
claim.

## Pointer access

For each load, store, copy, dereference, or reference construction, establish the
operation-specific facts that matter:

- nullness and alignment for the accessed type;
- a live allocation covering the exact byte range;
- whether the range must remain within one allocation;
- read permission, write permission, or both;
- initialization and validity of the value-bearing bytes required by the
  operation and accessed type; require initialized padding only when the
  operation actually observes it under a contract that says so;
- validity of the resulting Rust value, including discriminants, references,
  `bool`, `char`, `NonZero*`, and enum invariants;
- provenance or allocation origin where the governing contract requires it;
- aliasing, exclusivity, and mutation permissions for the full temporal scope;
- checked element-to-byte arithmetic, `isize::MAX` constraints, and absence of
  address-space wraparound.

"The pointer is valid" is not a complete premise. Validity is relative to an
operation, byte range, access kind, type, and duration.

Zero length and zero-sized types do not erase documented pointer requirements.
Slice constructors and reference-like values may still require a non-null,
properly aligned pointer. Check the exact API contract instead of inferring from
the lack of data bytes.

## Pointer arithmetic

For `add`, `sub`, `offset`, `offset_from`, or equivalent internal operations,
review:

- the original pointer's allocation and provenance;
- whether intermediate and resulting addresses must remain in bounds or may be
  one-past;
- the mathematical byte offset and its fit in `isize`;
- multiplication/addition overflow and address-space wraparound;
- the contract difference between wrapping and in-bounds operations;
- whether the result is later dereferenced under a stronger contract.

A bounds check on an integer index proves only the arithmetic fact it actually
checks; it does not establish allocation identity, initialization, alignment,
or aliasing.

## Reference-like values

Creating `&T`, `&mut T`, slices, `str`, `CStr`, `Box<T>`, pinned references,
trait-object references, or another reference-like owner from raw parts is
itself the safety assertion. Review validity at creation, not only at the later
load or store.

Establish:

- pointee liveness and type validity for the complete produced extent;
- correct metadata for slices, strings, and trait objects;
- a lifetime no longer than the actual storage and permissions;
- the applicable shared-reference immutability or mutable-reference exclusivity
  rules from current accepted semantics;
- that the new value does not overstate ownership, pinning, mutation, or
  deallocation rights.

Prefer narrow, late, short-lived references as an auditability property, while
still proving that even the narrow reference is valid. `UnsafeCell` permits
interior mutation only according to its contract; its presence is not a blanket
aliasing exemption. If the relevant aliasing rule is unsettled, produce a
precise unresolved question rather than choosing a candidate model silently.

## Initialization and value validity

Track storage and value state separately. Memory can be allocated but
uninitialized, initialized bytes can be invalid for a type, and write-valid
storage may not yet be read-valid as `T`.

For `MaybeUninit`, raw buffers, unions, or partially initialized arrays, verify:

- exactly which elements or fields are initialized along every control-flow
  path;
- whether typed reads, references, drops, or length updates expose only the
  initialized prefix;
- behavior on panic, early return, and partial failure;
- that padding is not mistaken for initialized value bytes;
- that a union field read is valid for the selected interpretation.

## Allocation and ownership reconstruction

For `Vec::from_raw_parts`, `Box::from_raw`, `Rc::from_raw`, `Arc::from_raw`,
allocator APIs, or internal equivalents, inspect:

- original allocator identity and exact allocation layout;
- size and alignment equality required by the deallocator, not mere
  compatibility;
- capacity, length, and the initialized element range;
- special ZST or zero-capacity rules;
- the single owner responsible for drop and deallocation;
- all old aliases, handles, and owners after the transfer;
- panic and early-return points between dismantling the old owner and creating
  the new one.

A statement that a pointer "came from a Vec" does not establish the current
capacity, allocator, ownership state, or absence of an old destructor.

## Layout, niches, padding, and transmute

For every layout assumption, identify its source: `repr`, published layout
guarantee, completed decision, or intentional compiler/stdlib implementation
contract. A measured size, current LLVM lowering, or successful transmute test
is only evidence.

Review transmute-like operations for:

- source and destination size;
- source validity and destination validity for the concrete value;
- alignment, ABI, provenance, and pointer/integer restrictions;
- ownership and destructor obligations on both sides;
- enclosing-type layout, not only the layout of an inner field;
- a simpler conversion that avoids the extra representation claim.

Inner layout equivalence does not automatically compose through `Option`,
enums, or other wrappers. `UnsafeCell<T>` and `MaybeUninit<T>` can change or
suppress niches in an enclosing type even when their own size and alignment
match `T`.

Padding belongs to the representation contract of a type. Typed moves may leave
padding uninitialized or fail to preserve its bytes. Raw hashing, comparison,
serialization, or copying of an entire object representation must establish that
every observed byte may be read and that no result relies on padding being
stable unless an authority explicitly guarantees it.

For packed fields, verify that forming a raw pointer does not create an
intermediate misaligned reference. Review the actual expression that takes the
address, not only the later unaligned read.

## Operation state transitions

Trace the postcondition of operations such as:

- `ptr::read`: a bitwise copy was produced; for non-`Copy` values, the original
  location cannot later be treated as a second owned initialized value without
  an appropriate overwrite/state transition;
- `ptr::write`: the destination now holds a new value and any previous value was
  not dropped by this operation;
- `copy` and `copy_nonoverlapping`: initialization and ownership may now exist in
  multiple byte locations even when only one logical owner is permitted;
- `assume_init`: storage is now exposed under `T`'s validity contract;
- `set_len`: the vector will treat the entire new prefix as initialized owned
  elements;
- raw ownership constructors: deallocation and drop responsibility transferred.

The existing safety note should make any proof-relevant transition visible to
later reviewers.

## Contracts to extract exactly

Consult the reviewed toolchain's documentation or intentional internal contract
for, at minimum, any use of:

- `slice::from_raw_parts` and `from_raw_parts_mut`;
- raw pointer reads, writes, copies, arithmetic, and `offset_from`;
- `read_unaligned`, volatile operations, and atomics over raw storage;
- `mem::transmute` and `transmute_unchecked`;
- `MaybeUninit::assume_init` and related array helpers;
- `Vec::set_len` and raw-parts constructors;
- `Box`, `Rc`, `Arc`, `CString`, or allocator raw constructors;
- `NonNull::new_unchecked`, unchecked indexing, and UTF-8 unchecked APIs;
- layout-affecting `rustc_*` attributes, intrinsics, and lang items.

## Const evaluation

When unsafe code is const-evaluable, review both run-time and compile-time
semantics. Establish any additional determinism, provenance, allocation, or
interpreter contract used by CTFE, and check that compile-time and run-time
results correspond as required. An internal const intrinsic or
`allow_internal_unstable` path needs the same explicit implementation-contract
and human-escalation treatment as its run-time counterpart.
