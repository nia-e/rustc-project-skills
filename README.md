# Rust Project skills

A collection of [Codex skills](https://learn.chatgpt.com/docs/build-skills) for Rust Project work: reviewing standard-library changes, working through soundness questions, preparing meetings, and managing a review queue. There are also skills for editorial review and creative work.

Each skill is a set of instructions and reference material that Codex uses for a particular kind of task. The Rust review skills draw on project guidance and examples of substantive reviews, with an emphasis on checking current source and explaining the evidence behind a finding.

## What's included

| Skill | What it helps with |
| --- | --- |
| [rust-libs-pr-review](rust-libs-pr-review/SKILL.md) | Reviewing library PRs for correctness, API scope, compatibility, tests, documentation, and performance. |
| [rust-stdlib-unsafe-review](rust-stdlib-unsafe-review/SKILL.md) | Reviewing unsafe code and safety contracts in `core`, `alloc`, and `std`. |
| [rust-stdlib-unsoundness-response](rust-stdlib-unsoundness-response/SKILL.md) | Assessing a reported soundness defect, its severity and exposure, and possible repairs. |
| [rust-libs-meeting](rust-libs-meeting/SKILL.md) | Preparing Libraries team discussions and following up on meeting decisions. |
| [rust-libs-review-queue](rust-libs-review-queue/SKILL.md) | Finding PRs and issues that need your attention, with an estimate of the work involved. |
| [prose-review](prose-review/SKILL.md) | Reviewing existing writing for structure, clarity, and the context its readers need. |
| [creative-inquiry](creative-inquiry/SKILL.md) | Exploring an idea's purpose and possibilities, then developing it into concrete work. |

The review, queue, and meeting skills produce private assessments. Public writing, patches, and team decisions stay with the human doing the work. Creative inquiry also supports hands-on writing and building.

## Getting started

Each top-level skill directory is a complete package. Copy the skills you want into your Codex user skills directory. For example, from a checkout of this repository, install PR review and the two soundness skills with:

```sh
mkdir -p ~/.agents/skills
cp -R rust-libs-pr-review rust-stdlib-unsafe-review \
  rust-stdlib-unsoundness-response ~/.agents/skills/
```

You can install the other skills in the same way, or symlink their directories if you want to use this checkout directly. Keep one installation of each skill to avoid duplicate entries. See [Codex's skill documentation](https://learn.chatgpt.com/docs/build-skills#where-codex-loads-local-skills) for discovery locations and configuration.

Then invoke a skill in a Codex conversation, for example:

> Use $rust-libs-pr-review for an independent private review of this PR: `PR_URL`

> Use $rust-libs-meeting to check follow-through from the last two completed meetings.

> Use $creative-inquiry to explore this idea and build a small working version.

Provide the relevant PR, document, or project context. Code review benefits from access to a Rust checkout; meeting and queue work need access to the relevant project discussions and records.
