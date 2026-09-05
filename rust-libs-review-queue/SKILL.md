---
name: rust-libs-review-queue
description: Privately assess a current personal work queue for Rust libraries participation from assigned, authored, review-requested, or mentioned PRs and issues, and accessible GitHub notifications. Use for deciding what needs the user's attention and estimating effort, rather than reviewing one PR or choosing a meeting agenda.
---

# Rust libraries review queue

Provide a private planning report identifying what needs the user's attention. Recommend human next actions with their evidence, effort, and dependencies. Keep every continuation within private analysis; do not draft or offer replies, reviews, or other public-ready text, and do not offer to act on the user's behalf. Leave notification state, subscriptions, labels, assignments, and bot state unchanged.

Assignment, an old mention, an unchecked bot box, and an unread notification are signals to investigate; none alone establishes work the user needs to do now.

## Establish coverage

- Resolve the intended GitHub account from the request or authenticated profile. Do not assume the authenticated account is the requested person when they differ.
- Start with the requested repository and include related Rust repositories when the user's responsibilities or linked decisions make them relevant. A rust-lang/rust implementation may depend on an ACP in libs-team, an RFC, a tracking issue, or another PR. Do not turn a personal libraries queue into an inventory of the whole Rust organization.
- Record retrieval time and what was actually covered: assigned, authored, direct review requests, team review requests, mentions, and notifications. Keep these sources distinguishable even after deduplicating items.
- Use the available authenticated GitHub connector or CLI. Issue and PR searches do **not** establish private notification coverage. If notification access fails, continue with accessible sources and state the gap. Respect a user's decision to skip login or a source.
- Follow pagination and acknowledge result caps or incomplete history. A partial search is useful, but is not a complete inbox sweep.

## Determine the user's next action

Use broad searches for discovery, then read enough current context to classify candidates. For promising or ambiguous items, inspect the latest relevant human discussion, review comments and replies, current PR head, requested changes, CI, mergeability, draft status, and dependencies. A PR summary or its last updated timestamp is insufficient.

Distinguish the user's relationship to an item from the work it needs:

- **Author:** Is a revision, rebase, response, prerequisite decision, or follow-up actually waiting on them? A PR waiting for another reviewer is not automatically an author task.
- **Assignee or requested reviewer:** Is there a reviewable revision or a concrete question? A team request does not necessarily assign every team member an individual action. Check whether the user has already reviewed the current changes.
- **Mentioned participant:** Find the question, commitment, or decision behind the mention. Bot chatter, a resolved exchange, and an FYI are not outstanding requests.
- **FCP participant or facilitator:** Separate personal review from poll stewardship, recording consensus, and a substantive design decision. Read the active proposal and its team scope; labels alone do not identify who can advance it.

Review-thread resolution flags and approval states are evidence, not a substitute for reading: a fix may answer an unresolved thread, and an old approval may precede substantial new changes. State when the next step is to verify a revision rather than asserting that the old defect persists.

Trace linked blockers to the actual next actor. For partial stabilization, establish whether a concern affects the proposed stable surface or only deferred experimental work. Collapse duplicate venues and stacked drafts around the decision or reviewable slice that can advance them. Keep dependent work visible without presenting every assigned draft as a simultaneous review obligation.

Useful dispositions, adapted to the task:

| Disposition | Required basis |
| --- | --- |
| Ready for the user's review | Reviewable changes and a reason their review is needed |
| User response or author revision | A concrete unanswered request or unfinished commitment |
| Coordination or team decision | A named decision, ownership question, or consensus communication needed |
| Waiting on someone else | The next actor and what is awaited |
| Blocked by a dependency | The prerequisite and its current state |
| Already handled or watch only | Evidence no immediate user action is needed |
| Uncertain | The missing context needed to classify it |

A missing comment does not itself mean a team decision was not communicated. If meeting attendance or consensus delivery matters, use the meeting follow-up workflow and its evidence; do not infer a forgotten action from silence alone.

## Prioritize without fake precision

Rank by what the user's action can unblock, urgency supported by current evidence, review risk, and available effort. Age and changed-line count may inform the estimate; they do not decide importance or difficulty.

Estimate the **user's effort for the proposed next action**, separately from eventual implementation effort or waiting time. Coarse ranges such as minutes, a focused session, or several sessions are usually enough. Explain the main driver or uncertainty: platform semantics, soundness proof, unresolved API design, a large stack, or an ordinary documentation change. A five-minute coordination action can unblock a large review; do not price it as doing that whole review personally.

Offer a short ordered queue, then keep waiting, blocked, and handled work compact. For each actionable item give its link, next action, why now, rough effort, and any prerequisite. Base readiness on the current substantive review and decision state.

## Continue the private review

When the user chooses an item for technical review, use `rust-libs-pr-review`; for unsafe code or a concrete soundness finding, use `rust-stdlib-unsafe-review` or `rust-stdlib-unsoundness-response`. For synchronous FCP selection or missed consensus delivery, use `rust-libs-meeting`. Use these companion skills when available; otherwise apply the relevant workflow within the requested scope and state material gaps. Do not perform a full technical review of every search result just to produce the queue.

Old reports and prior conversations can supply leads or explain commitments. Recheck live state before carrying forward a next action, and identify any facts that could not be refreshed.
