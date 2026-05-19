# Glossary & Domain Context

## Version & Release Terminology

### SemVer
Semantic Versioning in the format `MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]`. Represents API/feature compatibility semantics.

### Pre-release Version
A version with a pre-release tag (e.g., `1.0.0-alpha.1`, `1.0.0-beta.5`). Indicates the version is not yet stable for production use.

### Stable Version
A version without pre-release or build metadata (e.g., `1.0.0`). Production-ready.

### Build ID
A unique identifier assigned by the CI system (`${{ github.run_id }}-${{ github.run_attempt }}`) that uniquely identifies a single CI run. Embedded in binaries and bundles alongside SemVer.

### Commit SHA
The git commit hash (short form: first 7 characters). Uniquely identifies source code state. Embedded in binaries and bundles for traceability.

---

## Branching Terminology

### Development Branch (`dev`)
The main integration branch where features are merged. Uses ContinuousDeployment mode; every commit produces a pre-release (`X.(Y+1).0-alpha.N`).

### Release Branch (`release/version-demo_vX.Y`)
A branch created to prepare a specific Minor version for release. Uses ContinuousDelivery mode; every commit produces a beta pre-release (`X.Y.0-beta.N`). Only bugfixes; no new features.

### Release Tag (`release/version-demo_vX.Y.Z`)
A git tag that marks a specific commit as a stable, production-ready version. Required to finalize a release and proceed to production.

### Hotfix
A critical bugfix (security patch, data loss prevention, etc.) applied to a release branch. After being tagged, must be cherry-picked back to `dev` to maintain synchronization.

---

## CI & Versioning Concepts

### ContinuousDeployment
GitVersion mode where the pre-release counter increments on every commit. Every commit is immediately deployable as a pre-release. Used on `dev`.

### ContinuousDelivery
GitVersion mode where the pre-release counter is stable unless explicitly tagged. Commits on the branch share the same base version but have distinct pre-release numbers. Used on release branches.

### Monotonic Build ID
A sequentially increasing identifier assigned by the CI system. Ensures that two different CI runs never produce the same build ID, enabling deterministic artifact correlation.

### Version Collision
When two branches produce the same SemVer version at the same time. Example: dev and a newly-created release branch both target the same version. Prevented by `tracks-release-branches: true` setting.

---

## Processes

### Release Workflow
1. Feature development happens on `dev`
2. When ready to stabilize, create `release/version-demo_vX.Y` branch (e.g., `release/version-demo_v1.1`)
3. Release branch enters beta testing; bugfixes accumulate as `1.1.0-beta.N` versions
4. When stable, tag the commit as `release/version-demo_v1.1.0`
5. This tag produces stable version `1.1.0`; CI artifacts are published

### Hotfix Workflow
1. Critical bug found in production (currently on `1.1.0`)
2. Hotfix is committed to `release/version-demo_v1.1` branch
3. Create tag `release/version-demo_v1.1.1` on that commit
4. Hotfix is cherry-picked back to `dev` to prevent regression in future releases

