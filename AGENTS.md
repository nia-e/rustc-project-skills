# Working on this repository

Each top-level skill directory is a self-contained package. Keep the README focused on what a GitHub visitor needs to choose and use the skills.

## Maintain the skill packages

- Keep essential instructions and routing in `SKILL.md`; keep substantial examples and supporting evidence in linked references.
- Preserve each skill's purpose and output boundaries. Review, queue, and meeting skills provide private analysis; creative inquiry can support execution within the user's task. These scopes govern use of those skills.
- Treat historical PRs and meeting records as examples of review techniques, not permanent rulings or current task state. Do not turn an individual decision into a universal requirement.
- When refreshing examples or process guidance, check the relevant current source, discussions, and project decisions. Preserve dates and evidence links where they explain the scope of a claim.

## Validate changes

Run the installed `skill-creator` skill's `scripts/quick_validate.py` on changed skill packages, and check affected relative links and metadata. For substantive changes to behavior, exercise a realistic request using the skill's evaluation material where available. Format validation alone does not establish review quality; keep validation proportional to the change.

## Synchronize installations

Maintain the source packages here. When updating installed copies as part of a task, locate the configured skill directory and compare it with the source before copying, so a newer installed change is not overwritten. Synchronize the changed package and verify that the resulting trees match. Symlinked installations follow the source directly. Keep one discoverable installation per skill name.
