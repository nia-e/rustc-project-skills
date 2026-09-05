---
name: rust-libs-meeting
description: Privately prepare Rust Libs meeting FCP discussion candidates or audit whether meeting decisions reached the relevant people and received their agreed follow-up. Use for weekly Libs facilitation and minutes follow-through, not ordinary PR code review.
---

# Rust Libs meeting facilitation

Provide private analysis that helps a facilitator choose useful synchronous discussions and check whether agreed follow-up happened. Deliver recommendations and evidence for the user; keep every continuation within private advice. Do not draft or offer public-ready agendas, minutes, messages, or external actions. Communication, assignments, labels, reviews, and bot commands remain human responsibilities.

Choose the requested mode:

- **Before a meeting:** Read [sync selection](references/sync-selection.md), using [agenda and evidence mechanics](references/agenda-and-evidence.md) to establish current candidates.
- **After a meeting:** Read [follow-through](references/follow-through.md). Consult the evidence mechanics when retrieving discussion or bot state.
- **Both:** Audit the preceding meeting first; carry only still-relevant decisions into the new shortlist.

Locate the `libs-team` checkout from the user's context (often next to the Rust checkout or this skills repository); use `minutes/` and `tools/agenda-generator/`. Establish the target meeting date and retrieval time. For an unspecified upcoming meeting, use the current announcement or calendar rather than hardcoding a weekday. For unspecified follow-through, start with the latest substantive minutes and state that scope. Historical "as of" requests need evidence available by that cutoff, with later developments explicitly separated.

Keep these distinctions throughout:

- Agenda inclusion is a discovery hint. The active proposal, discussion, and current bot receipt establish what is being decided and by whom.
- A meeting opinion is not automatically team consensus. Preserve conditions, dissent, deferrals, and the exact accepted scope.
- **An author or assignee who attended normally does not owe a written acknowledgment.** In-room communication to the responsible person is usually sufficient. The question is whether the consensus reached the relevant parties, especially absent authors, other teams, and external stakeholders.
- Attendance does not execute a promised PR, update a label, record an FCP, or resolve a bot concern. Track communication separately from implementation and formal action.
- Missing accessible evidence means unverified, not neglected. Bare agenda headings and generic action checklists do not establish that a topic was discussed.

Produce a concise private advisory with links to the relevant minutes and current thread/bot evidence, distinguishing the findings from recommended human next steps.

Read [research provenance](references/research-provenance.md) only when checking why these distinctions exist or refreshing the skill. Its examples are historical evidence, never a reusable live candidate list.
