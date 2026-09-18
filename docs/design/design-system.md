# SilverSetup design system

**Status:** Design direction for future UI. The current app is an empty scaffold; this document does not authorize a feature or host action.

## Character

SilverSetup should feel like a precise, calm desktop utility for developers. Prioritize alignment, readable density, and a clear next action. Use cool neutral surfaces, thin borders, and one muted blue action accent. Status colors communicate status only.

Avoid gradients, glass effects, glow, decorative charts, giant headings, excessive cards, pill-shaped controls, emoji icons, and shadows on normal surfaces. Use native window chrome on each platform.

## Foundations

Components use semantic tokens rather than raw colors. Keep the same roles in Light and Dark themes:

| Role | Light | Dark |
| --- | --- | --- |
| Canvas | `#F5F7FA` | `#0C1015` |
| Surface | `#FFFFFF` | `#121820` |
| Subtle surface | `#F0F3F7` | `#171E27` |
| Primary text | `#17202B` | `#EFF3F7` |
| Secondary text | `#556273` | `#A8B3C0` |
| Border | `#CDD5DF` | `#303B48` |
| Action blue | `#1F63D5` | `#6A9FF8` |

Expose roles such as `--bg-canvas`, `--bg-surface`, `--bg-subtle`, `--text-primary`, `--text-secondary`, `--border-default`, `--action-primary`, `--status-success`, `--status-warning`, and `--status-danger`. Define hover, focus, disabled, and status variants within the token layer. A status must always include a label and icon, not color alone.

Follow the system theme by default; a future Light/Dark choice should persist locally without changing the layout structure.

- Use a 4px spacing rhythm: 4, 8, 12, 16, 24, and 32px. Normal page padding is 24px; compact padding is 16px.
- Use 1px borders, 4–6px ordinary radii, and little or no shadow. Reserve a restrained shadow for popovers and dialogs.
- Body text is typically 13–14px; page titles are about 20–24px. Use sentence case, not all caps. Interactive targets remain at least 32×32px.
- Bundle fonts locally: Inter for English UI, Noto Sans Myanmar for Myanmar UI, and JetBrains Mono for versions, commands, paths, hashes, and technical metadata. Technical text containing Myanmar needs a Myanmar fallback. Never transform Myanmar text to uppercase or add artificial letter spacing; allow taller line height and wrapping.

## Layout and components

The intended normal window is about 1280×820, but the interface must remain usable down to 400×500. A future expanded navigation rail is about 208px and may collapse to an icon rail when space is tight. Keep the top toolbar compact. Put page identity and the primary action first, then current state, attention items, working data, and secondary detail. Repeated data belongs in aligned rows or tables, not a card per item.

Use flexible widths and `min-height` rather than fixed text heights. Do not truncate primary actions, status messages, or essential technical values. At normal desktop sizes, a long tool table may scroll within its region while the page summary stays visible; at compact sizes, allow the page to scroll and reflow rows. Never use a size-warning overlay as the only response to a narrow window.

Build reusable controls only when a feature needs them. Inputs have visible labels. Buttons have clear verbs and stable width through loading states. Tables have readable headers, keyboard-operable actions, and meaningful empty content. Tooltips explain secondary details; they never hide critical text. Dialogs name the target, fit translated text, manage focus, and require confirmation before destructive actions. Progress reflects actual work; failures remain visible with a recovery path rather than disappearing in a toast.

## Language, trust, and accessibility

English (`en`) and Myanmar (`my`) are equal, left-to-right locales. Keep product and technology names unchanged, use the [terminology glossary](../localization/glossary.md), and format dates and numbers with `Intl`. Important labels and descriptions must wrap in both languages; icons stay aligned with taller Myanmar glyphs.

Only backend-supported actions may appear actionable. Distinguish detected, verified, managed, external, missing, and failed states accurately. Do not imply that a completed scan means a workstation is ready. Before any future host mutation, show a backend-owned plan with exact changes, source, target, and risk; the frontend never constructs commands or grants capability.

All core flows must work by keyboard, with visible focus, accessible names, logical focus order, and focus restoration after dialogs. Meet applicable WCAG 2.2 AA contrast and interaction requirements. Honor reduced motion; never use animation or color as the sole carrier of information.

## Review gate

For each UI change, test populated, loading, empty, error, disabled, long-text, and compact states as applicable. Inspect all four theme/locale combinations: Light/English, Dark/English, Light/Myanmar, and Dark/Myanmar. Check clipping, wrapping, contrast, focus, table readability, and dialogs at normal and compact sizes. Browser screenshots help iteration, but packaged Linux, Windows, and macOS webviews require their own review before release. Report what was actually inspected.
