# Agenda and evidence mechanics

## Establish the local and live snapshot

Read the current generator, its CLI, repository status, and recent minutes before relying on remembered labels or team structure. The observations below come from `rust-lang/libs-team` commit `a42791873701a0ed2c926c729bed078f1c351b23` (2026-09-05); recheck changes that affect the task. Preserve unrelated edits.

From `tools/agenda-generator`, the explicit Libs invocation is:

```sh
cargo run -- --agenda Libs
```

The generator reads `GITHUB_TOKEN` if supplied. Use existing credential tooling without printing secrets. In a read-only checkout, an external `CARGO_TARGET_DIR` keeps build output out of it; direct read-only API queries are an alternative if building is unnecessary. Keep generated output in private scratch space and leave the tracked agendas and minutes unchanged. It dates the header using current UTC, so verify the requested meeting date separately.

An API, auth, rate-limit, or local runtime failure is not an empty agenda. Use another available read-only source and state remaining coverage limits. Do not claim freshness from a failed generator run.

## What this generator actually selects

| Section | Selection at the inspected revision |
| --- | --- |
| FCPs | `https://rfcbot.rs/api/all`, filtered by issue label `T-libs`; globally ascending active concern count. |
| Critical | `T-libs` + `P-critical`, in rust and rfcs. |
| Backports | `T-libs` + either `stable-nominated` or `beta-nominated`; any issue state; exclude `beta-accepted`, in rust and rfcs. |
| Prioritization Requested | `T-libs` + `I-prioritize`, in rust and rfcs. |
| Nominated | `I-libs-nominated`, in rust, rfcs, and libs-team. |
| waiting on team | `S-waiting-on-t-libs`, in rust and rfcs. |
| needs decision | `T-libs` + `I-needs-decision`, in rust. |
| Regressions | `T-libs` + any of the four regression labels in source; excludes `I-libs-nominated` and `P-low`, in rust and rfcs. |
| New / stalled ACPs | Open `api-change-proposal` without `ACP-accepted`, in libs-team; up to 10 new, then up to 10 least recently updated candidates with shuffling and deduplication. |

Except backports, these GitHub sections default to open issues. API `/issues` includes PRs. Multiple `.labels(...)` groups are alternative queries; labels within one group are conjunctive. Section ordering is not a complete priority model.

Non-obvious limitations:

- FCP filtering uses labels, not the proposing team's identity. Historical minutes may show separate `T-libs-api` / `T-libs` lists and duplicates. Recover the active initiating command and its bot receipt; an explicit team selection beats current labels. For an implicit team selection, use the recorded proposal-time state, not today's labels alone. Check current team data if names or membership have changed.
- `reviews` is a list of `(user, checked)` pairs. The rendered "checkboxes left" counts *all* unchecked reviewers, not necessarily approvals needed to start FCP. Use the active bot receipt for its threshold and actual phase; don't hardcode unanimity or a fixed threshold.
- `concerns` lists active bot concerns. The tracking comment can also contain struck-through resolved concerns. Zero active concerns does not exclude ordinary objections elsewhere. A substantive resolution in conversation does not itself perform `resolved`.
- FCPs are not filtered by age or waiting-on-author status: that filter is disabled in source. They are not added to the regular issue dedup set and can reappear in another section.
- GitHub queries use a single API page at this revision. Deduplication marks URLs seen before exclusion and truncation, so an omitted item can also disappear from a later section. ACP sampling and shuffled output are not backlog coverage. For an exhaustive request, query/paginate the relevant sources independently.

## Fetch evidence for the question, not only the listing

Use an available GitHub connector, `gh api`, or public web sources. Follow pagination for issue comments, review comments, reviews, and timeline/events as needed. `gh pr view --comments` alone is not complete review-discussion coverage. Useful read-only examples:

```sh
gh api repos/rust-lang/rust/issues/NUMBER
gh api --paginate repos/rust-lang/rust/issues/NUMBER/comments
gh api --paginate repos/rust-lang/rust/issues/NUMBER/timeline
gh api repos/rust-lang/rust/pulls/NUMBER
gh api --paginate repos/rust-lang/rust/pulls/NUMBER/reviews
gh api --paginate repos/rust-lang/rust/pulls/NUMBER/comments
```

For FCPs, use `/api/all` identifiers to open the initiating and tracking comments. Recover cancellation/restart chains and link canonical tracking issue, ACP, implementation PR, or RFC. Absence from `/api/all` is not proof of cancellation or completion: inspect thread receipts for the actual transition. A current PR head and check results matter only when claiming readiness to implement or merge; `mergeable: null` is unknown, not a conflict.

For follow-through, inspect edits, labels, assignment history, linked PRs, later minutes, and any cited Zulip thread when needed. Current author/assignee fields do not prove responsibility at the meeting date. Resolve participant handles from evidence; don't infer a person was absent because the attendance line used their name instead of their handle.
