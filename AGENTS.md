# SilverSetup agent guide

## Authority and current scope

- There is no active product plan in this scaffold. Agree on a small feature scope and acceptance criteria before implementing behavior.
- `docs/design/design-system.md` is the design reference for future UI work.
- The current repository is an empty React + Tauri scaffold. No product feature, command, inventory, installer, upgrade, or host mutation is implemented. There is no active release plan or date; agree a small feature scope before adding behavior.

## Working rules

1. Read the relevant product context, architecture boundary, and security baseline before editing.
2. Keep changes limited to one reviewable objective. Preserve unrelated user changes.
3. Put business rules in Rust crates, not Tauri handlers or React components.
4. Frontend input must never select an executable path or form a process command.
5. Represent processes as an executable and argument array. Never interpolate shell command strings.
6. Do not log credentials, environment values, usernames, raw home paths, or unsanitized command output.
7. Do not add a dependency without documenting why it is needed and checking its maintenance and license.
8. Add or update tests for behavior changes. Coverage thresholds may not be reduced to pass CI.
9. Do not weaken lint, capability, CSP, or security settings without an accepted ADR.
10. Update the relevant documentation when behavior or architecture changes.

## Agentic development workflow

- Establish the requested outcome and current behavior before editing. Inspect the relevant implementation, tests, and working-tree changes; do not overwrite unrelated work.
- Make one coherent, reviewable change at a time. Write down the outcome and acceptance criteria for substantial or security-sensitive work; a small fix does not need a separate task document.
- Treat product and design documents as intent, not permission to enable a capability. Before crossing a trust boundary or adding host mutation, record an architecture/security decision and update the threat model.
- Verify changed behavior with deterministic tests and the required checks below. Report what was verified, what remains unverified, and any material risk or decision. Do not describe an untested state as complete.

## Frontend work

For changes in `apps/desktop`, read `docs/engineering/frontend.md` before editing. It owns the frontend-specific structure and React, TypeScript, Vite, accessibility, and localization guidance.

## Rust and Tauri work

For changes in `crates/*` or `apps/desktop/src-tauri`, read `docs/engineering/rust.md` before editing. It owns the Rust coding practices and Tauri integration guidance.

## Required verification

Run from the repository root:

```text
pnpm run check
pnpm run tauri:build
```

If a platform bundle cannot be built locally, run `cargo check --workspace --all-targets --all-features` and rely on the platform CI matrix. Report exactly what was and was not run.

## Repository map

- `apps/desktop/src`: empty React presentation shell and reserved feature, shared, platform, and binding directories.
- `apps/desktop/src-tauri`: minimal native host with no product commands.
- `crates/*`: reserved directories for future domain, application, catalog, execution, platform, security, and adapter crates; they are not workspace members yet.
- `fixtures`: reserved deterministic test-data location; tests must not depend on host-installed tools.

Dependencies should point inward toward `domain`. Cycles are forbidden.
