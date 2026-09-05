# Rust Standard Library Unsafe Review Skill - Background & Attributions

This review-only skill audits existing unsafe code in Rust's `core`, `alloc`,
and `std` crates as explicit proof obligations. It covers Rust-project
safety-note policy, compiler/standard-library internal contracts, unresolved
memory-model questions, and the authority of completed team FCPs and resolved
UCG issues. Its output and continuations stay within private advisory analysis,
supported by read-only research and bounded local verification. Source changes,
public material, and project actions remain human work. This personal workflow
boundary is distinct from the Rust project's contribution policy.

The methodology is derived from `unsafe_rust_review`.

## Authorship & Development

*   **Primary Development**: Developed by @joshlf working in collaboration
    with an AI agent.
*   **Extensions & Refinements**: @manishearth added additional rules,
    policy alignment, and operational guidance based on extensive experiences
    reviewing unsafe Rust code internally at Google.

## Theoretical & Practical Foundations

The rules and verification criteria within this skill are grounded in:

*   **Language & Standard Semantics**:
    *   [The Rust Reference](https://doc.rust-lang.org/reference/)
    *   The Rust standard library documentation
    *   [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
    *   [The Unsafe Code Guidelines](https://rust-lang.github.io/unsafe-code-guidelines/)
    *   [UCG issue tracker](https://github.com/rust-lang/unsafe-code-guidelines/issues)
    *   [Rust standard-library safety comment policy](https://std-dev-guide.rust-lang.org/policy/safety-comments.html)
    *   [Rust project LLM usage policy](https://forge.rust-lang.org/policies/llm-usage.html)
    *   Rust RFCs
    *   [Clippy documentation](https://rust-lang.github.io/rust-clippy/master/index.html)
*   **Real-world Findings**:
    *   Incorporates bug patterns and safety findings documented in the
        open-source
        [google/rust-crate-audits](https://github.com/google/rust-crate-audits)
        repository.
