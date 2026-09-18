# Desktop frontend engineering

This guide applies to `apps/desktop`. The root [agent guide](../../AGENTS.md) owns scope, security rules, and required verification; the [architecture boundary](../architecture/boundaries.md) and [security baseline](../security/threat-model.md) govern future capabilities. Use the [design system](../design/design-system.md) for visual decisions. The current app is an empty scaffold.

## Ownership and structure

- `src/app` owns the current empty shell. Add navigation, global state, and shell styles only when a scoped feature needs them; page-specific components do not belong here.
- `src/features`, `src/shared`, `src/platform/tauri`, and `src/bindings` are empty reserved directories. When features arrive, colocate screens and workflows in `features/<feature>`, share only genuinely reused code, keep typed command wrappers at the platform edge, and generate bindings rather than hand-editing them.
- `tests` currently checks the shell. Add user-visible workflow and boundary tests with each feature, using deterministic fixtures independent of host-installed tools.

Dependencies flow from app and features into shared code and typed platform wrappers. Shared code must not import a feature; presentation must not create process commands or infer install capability beyond the backend contract.

## React, TypeScript, and Vite

- Inspect the existing screen, styles, translations, and tests before changing its structure. Keep rendering pure; derive display values from current inputs, handle user actions in event handlers, and use Effects only to synchronize with external systems, cleaning them up when needed.
- Keep state near its owner and represent mutually exclusive loading, success, failure, and cancellation states explicitly. Avoid duplicate derived state or new abstractions without a repeated need.
- Preserve strict TypeScript checks and use generated Tauri types at the boundary. Narrow or validate untrusted runtime data. Avoid `any`, unchecked assertions, non-null assertions, and suppression comments; if unavoidable, isolate and explain them.
- Vite preview fixtures and development behavior must not authorize packaged-app actions. `VITE_` values are public build-time data, never secrets or authority for host changes. Bundle UI assets locally.

## Product quality and evidence

- Show only backend-supported actions and provide meaningful loading, empty, error, disabled, progress, success, and recovery states where applicable. Preserve keyboard access, visible focus, and accessible names.
- When copy is added, provide English/Myanmar translations and bundle the intended UI and technical fonts. Allow text and technical values to wrap; consult the [localization glossary](../localization/glossary.md) for recurring terms.
- Add or update behavior tests for changed paths. Inspect rendered changes in Light/Dark and English/Myanmar, including compact layouts and affected dialogs. Browser captures aid iteration but do not qualify packaged WebKitGTK, Windows, or macOS behavior; report native checks separately.
- Keep existing lint, typecheck, format, test, and coverage gates intact. Report what was actually verified rather than claiming completion from source inspection alone.
