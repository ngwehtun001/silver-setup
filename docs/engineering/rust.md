# Rust and Tauri engineering

This guide applies to future `crates/*` code and `apps/desktop/src-tauri`. The current scaffold has no product crates or commands. The root [agent guide](../../AGENTS.md) owns scope and required checks. The [architecture boundaries](../architecture/boundaries.md) and [security baseline](../security/threat-model.md) are authoritative when they are more specific than this guide.

## Design and ownership

- Put product concepts and invariants in `domain`, use-case coordination and ports in `application`, and operating-system effects in `platform`. Keep the Tauri crate a composition and transport boundary. Follow the existing crate map; do not move logic to a convenient caller merely to avoid a dependency decision.
- Design types so invalid states cannot be constructed casually. Keep fields private when construction needs validation, use named constructors or methods for transitions, and make unsupported operations return a typed failure. Do not encode important states as magic strings, booleans with unclear meaning, or unrelated `Option` fields.
- Prefer cohesive functions and modules with clear names over speculative traits or generic frameworks. Introduce a trait when a real boundary needs interchangeable implementations or deterministic fakes; do not abstract every concrete type. Avoid duplicating a rule across crates.
- Keep public APIs small. Document externally meaningful invariants and failure behavior, especially for exported types and fallible functions. Treat serialized names and generated TypeScript bindings as contracts: change them deliberately and test compatibility.

## Errors and effects

- Return `Result` for expected failures and use `?` to propagate them within the owning layer. Define stable error variants that callers can act on; map low-level failures to safe public errors at the boundary. Do not expose raw operating-system, network, process, or path text in UI errors or logs.
- Do not use panic, `unwrap`, or `expect` to handle runtime input or host conditions. The workspace forbids unsafe code and denies those Clippy lints; do not suppress them to make a check pass. Any other lint exception needs a narrow scope and an explanation.
- Make side effects explicit and bounded. Follow the [security baseline](../security/threat-model.md) for commands and diagnostics. Validate paths, sources, and state before effects, then revalidate assumptions that can change before committing a host mutation. Never use a shell command string or overwrite an existing user-owned target as a shortcut.
- Keep pure decision logic separate from I/O where practical. Prefer borrowed data when ownership is not needed, but do not add lifetimes or micro-optimizations that make the API harder to understand without evidence.

## Tauri boundary

- Expose only reviewed commands. A command accepts a narrow, typed intent, validates it, calls an application use case or trusted platform operation, and maps the result to a stable, sanitized response. It must not become a second business-logic layer.
- The webview cannot select an executable, argument list, download URL, destination, or privilege level. For a mutating operation, use a backend-owned plan and explicit confirmation as required by its accepted ADR. A catalog entry or frontend control never grants capability by itself.
- Use the generated Rust-to-TypeScript bindings for shared payloads; regenerate and check them rather than hand-editing generated files. Keep command names, payloads, and error codes intentional because frontend callers depend on them.
- Move blocking filesystem, process, or network work off the async runtime. Keep locks scoped to short state transitions and never hold a lock across an `.await`. Treat poisoned locks, cancelled tasks, expired plans, and repeated confirmation as ordinary failure paths.
- Changes to commands, capabilities, CSP, downloads, elevation, persistence, or host mutation require a recorded architecture/security decision and threat-model review before implementation.

## Tests and review evidence

- Test invariants and error mapping beside the code that owns them. Use fake ports and deterministic fixtures for platform behavior; never depend on host-installed tools, live network responses, or the developer's home directory for unit tests.
- For effects, cover success plus failure, timeout, malformed input, path conflict, repeated action, and cleanup where relevant. Assert externally meaningful results and absence of forbidden effects, not incidental implementation details.
- Test serialized wire values and regenerate bindings when a public payload changes. A new platform capability also needs platform-specific qualification; unit tests on one OS do not prove Windows, macOS, native Linux, or WSL behavior.
- Run the checks in [AGENTS.md](../../AGENTS.md). Do not lower thresholds or add broad lint allowances to pass a gate. Report precisely which checks and native scenarios ran.
