# Contributing

SilverSetup is security-sensitive desktop software. Keep pull requests narrow, tested, and easy to audit.

Follow the stack-specific coding guidance for [React/TypeScript/Vite](docs/engineering/frontend.md) and [Rust/Tauri](docs/engineering/rust.md). Architecture and security policies remain authoritative for trust-boundary decisions.

1. Select one task with explicit acceptance criteria.
2. Record the decision and update the security baseline before changing an architectural or security boundary.
3. Add tests with the implementation.
4. Run `pnpm run check`. Run the three-platform desktop build before a release and when native or platform behavior changes.
5. Describe security, privacy, and platform implications in the pull request.

Commits use an imperative Conventional Commit prefix such as `feat:`, `fix:`, `test:`, `docs:`, `build:`, or `chore:`. Generated changes and dependency updates must be isolated from behavioral changes when practical.

For substantial or security-sensitive work, agree on the outcome, non-goals, and verification before implementation. Small fixes can be handled directly when those are clear.
