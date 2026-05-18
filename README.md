# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## CI & Versioning

This repository includes a GitHub Actions workflow that builds the frontend and the Tauri Rust backend.

- The workflow uses GitVersion (GitTools) to compute a SemVer-style `VERSION` from your git history and tags.
- Each CI run attaches a monotonic build id composed from `${{ github.run_id }}-{{ github.run_attempt }}` which is embedded into both the frontend and the Rust binary.
- Release branches named like `vX.X` or tags (for RCs, e.g. `v1.2.0-rc.1`) will trigger the workflow automatically.

What you can see in the app:

- Frontend: a generated `src/build-info.json` is included in the bundle and shows `version`, `buildId`, and `commitSha`.
- Backend: the Rust side exposes a `get_build_info` Tauri command that returns the same metadata at runtime.

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
