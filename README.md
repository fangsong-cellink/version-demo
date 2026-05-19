# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## CI & Versioning

For reasoning about this decision, refer to the [ADR (Architecture Decision Record) doc](./docs/adr/0001-automated-git-driven-versioning-with-gitversion.md).

This repository uses [GitVersion](https://gitversion.net/) (v6) to derive a SemVer version automatically from the git graph — no manual version bumps needed in most cases.

### Branch model

| Branch pattern | Example | Version produced | Pre-release tag |
|---|---|---|---|
| `dev` / `main` / `master` | `dev` | `X.(Y+1).0-alpha.N` | `alpha` |
| `release/version-demo_vX.Y` | `release/version-demo_v3.1` | `X.Y.0-beta.N` | `beta` |
| Tagged commit | `release/version-demo_v3.1.0` | `3.1.0` (stable) | — |

- **`dev` branch** uses ContinuousDeployment mode. Every commit produces `X.(Y+1).0-alpha.N` where `N` is the commit count since the last release tag. Minor version increments automatically.
- **`release/version-demo_vX.Y` branches** use ContinuousDelivery mode. The `X.Y` in the branch name drives the base version — no anchor tag required. Every commit produces `X.Y.0-beta.N` where `N` is the commit count on that branch.
- **Tagging a release commit** as `release/version-demo_vX.Y.Z` (or `release/version-demo_vX.Y.Z-beta.N`) produces exactly `X.Y.Z` (or `X.Y.Z-beta.N`) with no increment.

### Bumping the major version

Include `+semver: breaking` or `+semver: major` in `dev` in a commit message to trigger a major version bump.

### Tag prefix

Tags must be prefixed with `release/version-demo_v` (e.g. `release/version-demo_v3.1.0`).

### CI workflow

- The workflow triggers on pushes to `dev`, `release/*` branches, and `release/*` tags.
- GitVersion computes the `VERSION` and injects it before the build.
- Each CI run embeds a monotonic build id `${{ github.run_id }}-${{ github.run_attempt }}` into both the frontend bundle and the Rust binary.

### What the app shows

- **Frontend**: `src/build-info.json` (generated at build time) contains `version`, `buildId`, and `commitSha`.
- **Backend**: the `get_build_info` Tauri command returns the same metadata at runtime.

Local testing:

1. Install dependencies and build the frontend:

```bash
npm install -g pnpm
pnpm install
pnpm build
```

2. Build the Rust backend (Tauri):

```bash
cd src-tauri
cargo build --release
```

In CI, the `src/build-info.json` file is created by the workflow before the frontend build so the version is baked into the produced bundle.
