# Security baseline

The current scaffold opens an empty webview. It has no product commands, inventory, network request, persistence, or host mutation. The Tauri capability and CSP are intentionally narrow.

## Assets and boundaries

Future features must protect user files, credentials, configuration, installed executables, and release artifacts. Treat webview input, operating-system paths and environment values, process output, downloaded data, and third-party binaries as untrusted at their respective boundaries.

## Rules for future capabilities

- Frontend input may express intent but must not select an executable, shell command, download URL, destination, or privilege level. Rust owns validation and action plans.
- Processes use an executable and argument array, explicit working directory, minimal environment, timeout, output limit, and sanitized result. No implicit shell invocation.
- Downloads require a reviewed source, integrity policy, size/time limits, staged verification, and no-clobber publication. Mutating actions require explicit confirmation and safe failure cleanup.
- Logs and UI errors must not reveal credentials, raw home paths, environment dumps, or unsanitized command output.
- Do not add elevation, remote UI assets, broad capabilities, or weaker CSP settings without a written decision and risk review.

Before enabling any command, network access, persistence, or host mutation, update this threat model with the concrete data flow, failure cases, platform behavior, and tests. Product or design plans are not implementation authorization.
