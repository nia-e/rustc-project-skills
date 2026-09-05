# Check whether meeting decisions were carried through

The central question is: **Did the people who needed the team decision receive it, and did the agreed next step happen?** This is not an acknowledgment audit.

## Recover what the meeting actually decided

Read the full relevant topic, attendance, explicit action list, and linked issue. Include decisions inside dialogue and chat transcripts, not just `Actions`. For each actual decision or commitment, retain a small working record:

`minutes anchor · subject · consensus and conditions · required audience · owner if explicit · expected action · evidence · disposition`

Separate a participant's suggestion from consensus; "maybe", a question, silence in incomplete notes, and "skip/defer" are not acceptance. Preserve a decision to seek input or delegate discretion as its own outcome. Do not silently turn "accept the ACP" into approval of every detail or permission to stabilize.

Many committed files are agenda-only stubs: `Attendees: ...`, bare headings, and a generic instruction to reply to everything. These cannot prove a meeting occurred, who attended, or what was decided. Find substantive minutes/linked notes or report the evidence gap. In rough notes, a speaker's recorded participation can establish presence even when the attendance list is incomplete; identity still needs to be clear.

## Evaluate communication by audience

- If the author or responsible assignee was present for the relevant discussion, normally mark the consensus as communicated in the meeting. Do not flag them just because they did not repeat it in the thread. Name this evidence basis rather than claiming a public reply exists.
- An unrelated attendee or unassigned reviewer does not establish that an absent author received a decision. A present responsible assignee is normally sufficient for ordinary review follow-up; do not demand a duplicate reply from them. If the meeting explicitly requires a message to an absent author or consultation with an external team or stakeholder, track that additional audience separately.
- A substantive thread reply, relevant review, linked decision, or visible change implementing the agreed request can establish follow-through. No particular person needs to post if somebody else conveyed the consensus accurately.
- A public reply need not quote minutes verbatim. It should preserve the material acceptance/rejection, conditions, rationale needed by the recipient, and next step. A ping alone does not communicate the decision it was meant to carry.
- Merely committing minutes does not prove absent parties were reached. A linked or explicitly shared minutes section may suffice when it clearly conveys the decision to that audience.
- Honor explicit communication commitments: "I'll write up why we deferred this" is a deliverable even if the PR author attended. Its audience or durable rationale may extend beyond that author. Do not invent a general obligation to publish every discussion.

Map author, assignee, requester, concern owner, and outside stakeholders at the time of the decision. Read later discussion and linked work through the requested cutoff; do not use today's assignee to decide who heard last month's consensus.

## Keep separate dispositions

One topic may have more than one: communication can be complete while implementation or bot action remains pending.

| Disposition | Evidence and interpretation |
| --- | --- |
| Communicated / complete | Intended audience heard the consensus in the meeting or received it later; required action is complete or none was required. No acknowledgment nag. |
| Communication gap | A material decision or request needs to reach an absent party, and accessible evidence does not show that it did. Say "no evidence found" where appropriate, with the searched scope. |
| Implementation pending | Consensus reached its recipient; a PR, documentation change, investigation, or other promised work remains. Report its owner/status without reclassifying it as silence. |
| Formal action pending | Agreed ACP disposition, nomination/priority/backport label, FCP proposal/cancellation/review/concern resolution, or issue disposition has no confirming event or bot receipt. A spoken promise or typed command alone is not completion. |
| Uncertain / conflicting evidence | Missing notes, inaccessible thread, unclear identity, partial pagination, or a conflict between minutes and public disposition prevents a confident conclusion. Identify the smallest verification needed. |
| Superseded | A later decision, replacement proposal, closure, or completed work displaced the old action. Link the reason and do not revive stale work. |

Formal state needs special care. A stale rfcbot concern can remain after agreement; a struck-through resolved concern is not still open; a cancellation/restart invalidates the old checklist. Check actual bot acknowledgments, errors, and current state before reporting a missed action. Verify who is permitted to act rather than assuming someone can resolve another person's concern.

Minutes and later public disposition can disagree. Report communication separately, inspect later decisions/edits for a superseding explanation, and flag any remaining substantive discrepancy for reconciliation. Do not automatically demand the older action or treat the newer comment as proof the minutes were accurate.

## Report gaps proportionately

Lead with actionable communication gaps and explicit commitments that appear outstanding, with minutes link, current thread evidence, intended audience, and the smallest next step. Include a compact list of checked items already covered by in-room participation or later action when needed to explain coverage. No finding is a valid result.

Call something overdue only when a deadline or real dependency supports that judgment. A newly assigned task can simply be pending. Avoid assigning blame or a new owner; use the recorded owner, otherwise say who would need to take it on. Describe the human follow-up needed to close each gap, without drafting its public wording or offering to carry it out.
