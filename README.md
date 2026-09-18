# SilverSetup

SilverSetup is a planned local-first desktop application for preparing a development workstation.

> **Status:** Empty React + Tauri scaffold. It has no product commands, inventory, installers, or upgrade flow. No release scope or date is committed yet.

Start with the short [documentation index](docs/README.md) before adding a feature. Define its scope and acceptance criteria first; the scaffold does not contain an active product plan.

## Prerequisites

- Node.js 24.20.0
- pnpm 11.9.0 through Corepack
- Rust 1.94.1; the repository toolchain file selects it automatically
- Tauri 2 platform prerequisites for your operating system

## Start developing

```bash
corepack enable
pnpm run bootstrap
pnpm run check
pnpm run tauri:dev
```

## Architecture

The React frontend and Tauri host are intentionally minimal. Feature, shared, platform, binding, and Rust crate directories reserve the planned boundaries without implementing them. See [docs/architecture/boundaries.md](docs/architecture/boundaries.md).

## Security and privacy

The scaffold has no product-level host access, network request, telemetry, account, background service, or privilege elevation. Planned installers and upgrades require security review before implementation. See [SECURITY.md](SECURITY.md) for the current support and reporting status.

GitHub vulnerability alerts and Dependabot security-fix PRs are enabled. Routine dependency upgrades are reviewed manually; automated version-update PRs are disabled.

## License

No public license has been granted yet. All rights are reserved until the project owner selects and adds a license.
