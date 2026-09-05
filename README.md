# Rust Project skills

Skills for Rust Project participation, private review, and creative collaboration.

| Skill | Use it for |
| --- | --- |
| [rust-libs-pr-review](rust-libs-pr-review/SKILL.md) | An independent review of a library PR: behavior, API scope, compatibility, tests, documentation, platforms, and performance. |
| [rust-stdlib-unsafe-review](rust-stdlib-unsafe-review/SKILL.md) | Soundness proofs and safety-contract review in `core`, `alloc`, and `std`. |
| [rust-stdlib-unsoundness-response](rust-stdlib-unsoundness-response/SKILL.md) | Assessing a concrete soundness finding and its severity, exposure, and repair tradeoffs. |
| [rust-libs-meeting](rust-libs-meeting/SKILL.md) | Selecting FCPs worth synchronous discussion, or checking whether meeting decisions reached the people who need them. |
| [rust-libs-review-queue](rust-libs-review-queue/SKILL.md) | Finding what needs your attention next, with rough effort and current blockers. |
| [prose-review](prose-review/SKILL.md) | Reviewing prose for coherence, reader context, justified qualifications, and consequential editorial issues. |
| [creative-inquiry](creative-inquiry/SKILL.md) | Exploring a task's purpose, design possibilities, and unexpected connections, then carrying it through. |

For example:

- “Use $rust-libs-pr-review for an independent private pass on this PR.”
- “Use $rust-libs-meeting to suggest FCPs for next week's meeting.”
- “Use $rust-libs-meeting to check follow-through from the last two completed meetings.”
- “Use $rust-libs-review-queue to find what I can unblock in a short session.”
- “Use $prose-review to assess whether this document stands on its own for its intended readers.”
- “Use $creative-inquiry to explore what this idea could become and build a small working version.”

The PR and meeting skills keep their research and detailed guidance in linked
references. Historical examples explain the method; live PRs, decisions, bot
receipts, and repository state determine the answer to a new task. The meeting
workflow tracks consensus delivery: a present author or assignee need not leave
a redundant acknowledgment comment.

The review, queue, and meeting workflows end in private analysis for the user.
These skills can identify human next steps, but never offer public-ready prose, patches, publishing,
external actions, or decisions on the team's behalf. Read-only research and
bounded local verification support the analysis.

Creative inquiry supports exploration and execution within the underlying
task's scope. It can guide authorized building or writing, and preserves the
private advisory boundaries when used alongside a review skill.

## Local use and maintenance

Each top-level skill directory is a complete skill package. Copy a chosen
directory into the user skill location configured for your Codex installation,
or symlink it there to use this checkout directly. Use one installation per
skill name to avoid duplicate discovery. The current documented user location
is `~/.agents/skills`; this machine also loads existing skills from
`~/.codex/skills`. See [Codex skill discovery](https://learn.chatgpt.com/docs/build-skills#where-codex-loads-local-skills).

Maintain the skill packages here and synchronize installed copies after an
update. Compare both trees first so a newer local change is not silently
overwritten; symlinked installations follow the repository directly.

Validate a changed package with the installed skill-creator's
`scripts/quick_validate.py`, then exercise a realistic request. Format validation
does not establish review quality. Keep source examples in references and avoid
turning one historical review decision into a universal rule.
