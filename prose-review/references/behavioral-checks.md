# Behavioral checks

Use small, self-contained fixtures with a stated document purpose and reader. Supply the text and any necessary source evidence to an independent reviewer without these expected judgments. Request private review only; evaluate the reasoning and scope, not exact phrasing or the number of notes.

## Evidence-limited claim

A memo for engineers choosing storage hardware says: "The benchmark improved median write latency by 12%, but covered only warm-cache workloads." The supplied results contain only warm-cache runs.

Expected judgment: retain the limitation because it changes how the result can inform the choice. Do not flag "but" or weaken the caveat merely for concision. Do not demand unsupported performance conclusions.

## An alternative the reader does not need

A short onboarding guide explains a concrete way to request access. It adds: "This is not an exercise in institutional gatekeeping; it is about empowering your work." No gatekeeping concern appears in the request, intended audience context, or surrounding document.

Expected judgment: identify the unsupported contrast and its distraction from the task. Recommend a conceptual removal or refocus without writing replacement copy or declaring all negative constructions bad.

## One passage, two document purposes

Text: "We first proposed daily snapshots, but the retention requirement ruled them out. We selected continuous backup after the recovery trial."

Evaluate once as an architecture decision record whose readers need the selection rationale, then as an everyday restore procedure whose readers only need to recover a file. The retention requirement and trial are established facts in both fixtures.

Expected judgment: preserve useful causal history in the decision record. For the procedure, consider relocating the selection history while retaining any constraints that affect recovery. The answer should change with purpose, not apply a universal ban on chronology.

## Drafting residue versus a real limitation

A freestanding product brief says: "The initial draft described three supported regions. After checking the source again, we corrected this to two. The source covers enterprise plans only." Its intended reader has not seen earlier drafts and needs the current supported scope; the supplied source confirms two regions for enterprise plans.

Expected judgment: distinguish unnecessary narration of the correction from the material plan limitation. Recommend organizing the current claim and its scope coherently, without reproducing a revision timeline or silently generalizing to all plans.

## Clear text with a consequential condition

An internal instruction says: "Delete the temporary export after the recipient confirms successful import. Keep it while import is incomplete so the transfer can be retried." The stated purpose is recoverable transfer, and the supplied retention policy permits this interval.

Expected judgment: no material finding is a valid result. Preserve the condition and explanation; do not invent redundancy, demand a shorter version, or provide a replacement passage.
