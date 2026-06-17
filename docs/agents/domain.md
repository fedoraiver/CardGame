# Domain Docs

How the engineering skills should consume this repo's domain documentation when exploring the codebase.

## Before exploring, read these

- **`CONTEXT-MAP.md`** at the repo root. It points at one `CONTEXT.md` file per context; read each one relevant to the topic.
- **`docs/adr/`** - read system-wide ADRs that touch the area you're about to work in.
- **`src/<context>/docs/adr/`** - read context-scoped ADRs when working in that context.

If any of these files don't exist, **proceed silently**. Don't flag their absence; don't suggest creating them upfront. The producer skill (`/grill-with-docs`) creates them lazily when terms or decisions actually get resolved.

## File structure

Multi-context repo:

```
/
|-- CONTEXT-MAP.md
|-- docs/adr/
|   |-- 0001-system-wide-decision.md
|   `-- 0002-system-wide-decision.md
`-- src/
    |-- gameplay/
    |   |-- CONTEXT.md
    |   `-- docs/adr/
    |       `-- 0001-context-specific-decision.md
    `-- ui/
        |-- CONTEXT.md
        `-- docs/adr/
            `-- 0001-context-specific-decision.md
```

## Use the glossary's vocabulary

When your output names a domain concept (in an issue title, a refactor proposal, a hypothesis, a test name), use the term as defined in `CONTEXT.md`. Don't drift to synonyms the glossary explicitly avoids.

If the concept you need isn't in the glossary yet, that's a signal - either you're inventing language the project doesn't use, or there's a real gap to note for `/grill-with-docs`.

## Flag ADR conflicts

If your output contradicts an existing ADR, surface it explicitly rather than silently overriding:

> _Contradicts ADR-0007 - but worth reopening because..._
