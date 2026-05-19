# ADR 0001: Automated Git-Driven Versioning with GitVersion

**Status:** Accepted

**Date:** 2026-05-19

**Decision Makers:** [Your team]

---

## Context

The project needs a versioning strategy that:
- Works across a full release lifecycle (development, pre-release, stable)
- Minimizes manual intervention and human error
- Provides complete auditability and traceability from binary back to source
- Supports both continuous development and controlled release cycles
- Is deterministic and reproducible from the git history alone

---

## Decision

Adopt **GitVersion v6** with a three-tier branching strategy:

### 1. Development Branch (`dev`)
- **Mode:** ContinuousDeployment
- **Versioning:** `X.(Y+1).0-alpha.N` where N increments on every commit and `X.Y` is last release
- **Target:** Next Minor version
- **Increment tracking:** Uses `tracks-release-branches: true` to auto-sense release branch creation and prevent version collisions
- **Policy:** Major bumps permitted via `+semver: breaking` / `+semver: major` in commit messages

### 2. Release Branches (`release/version-demo_vX.Y`)
- **Mode:** ContinuousDelivery
- **Versioning:** `X.Y.0-beta.N` where N increments only on new commits (pre-release counter is stable across untagged commits)
- **Target:** Next Patch version
- **Policy:** Stabilization only; no breaking changes or feature development

### 3. Release Tags (`release/version-demo_vX.Y.Z`)
- **Mode:** Exact version match (e.g., `3.1.0`)
- **Policy:** Requires explicit tag ceremony; no auto-release
- **Gate:** Deliberate milestone signaling production readiness per company release process

---

## Rationale

### Why Automatic Versioning from Git History?

- **Reduces friction:** Eliminates version-bump PRs and synchronization overhead
- **Eliminates human error:** SemVer sequences are computed deterministically from git state

### Why Embed `buildId` and `commitSha`?

SemVer alone is insufficient for production traceability:
- Two commits at the same distance from the last release tag produce identical SemVer versions
- Encoding branch name would break SemVer semantics and fail when branches are deleted/recreated
- Rebuilding the same commit can yield different artifacts due to external environment changes

**Solution:** Embed a unique `buildId` (from CI run) and the exact `commitSha` alongside SemVer. This ensures:
- Every deployed binary is uniquely traceable to a specific CI run and source commit
- Incident response and reproducible builds are trivial without requiring a rebuild

### Why Separate Modes for Dev vs. Release Branches?

**ContinuousDeployment on dev:**
- Every commit is immediately shippable as an alpha pre-release
- Enables continuous alpha deployments for testing and early adoption
- Reflects the nature of dev: features are always being added

**ContinuousDelivery on release branches:**
- Pre-release number stays stable across bugfix commits, reducing noise
- Prevents version churn during QA/stabilization cycles
- Reflects the nature of release branches: only bugfixes, no new features

### Why Increment Minor on Dev but Patch on Release?

The branching strategy mirrors the version strategy:
- **Dev increments Minor:** Development work adds features; the next Minor captures "what features are coming"
- **Release increments Patch:** Release branches contain only bugfixes; the next Patch captures "what's being stabilized"

### Why Require Explicit Tags Rather Than Auto-Release?

The tag ceremony (`release/version-demo_vX.Y.Z`) is a deliberate gate:
- Signals that a *specific commit* has been validated and is production-ready
- Enforces company release process discipline
- Prevents auto-release on every beta commit, which would lose intentionality

### Why Allow Major Bumps Only on Dev?

Release branches are for stabilization, not breaking changes:
- Major version bumps belong to dev where feature development happens
- Disallowing them on release branches prevents confusion (e.g., "should a beta suddenly jump to the next major?")
- Reinforces the policy: release branches contain only bugfixes

### Why Use the `release/version-demo_v` Tag Prefix?

- **Disambiguates** release tags from other git tags (build artifacts, milestones, etc.)
- **Supports monorepos:** The product-name pattern (`version-demo_v`) enables multi-repo or multi-product scenarios
- **Self-documenting:** Tag format immediately signals its purpose

### Why Auto-Sense Release Branches on Dev?

The `tracks-release-branches: true` setting in GitVersion:
- When a new `release/version-demo_vX.Y` branch is created, dev automatically yields that version to it
- Dev then auto-increments to the next Minor target
- **Benefit:** Prevents version collisions between dev and release branches without manual coordination

### Why Mandate Release Branches?

Direct tagging on dev is forbidden:
- Releases must go through the release branch gate to ensure proper stabilization
- Release branches provide a formal checkpoint for QA and validation
- Enforces intentionality and process discipline

### Why Cherry-Pick Hotfixes Back to Dev?

Hotfixes (security patches, critical bugs) applied to release branches must be cherry-picked back to dev:
- Keeps dev synchronized with production fixes
- Prevents the divergence problem where dev lacks critical patches
- Ensures future releases inherit all hotfixes

---

## Consequences

### Positive
- **No manual version management:** Versions are computed from git state; no risk of skipping numbers or duplicates
- **Complete traceability:** `buildId + commitSha + SemVer` triple uniquely identifies every artifact
- **Reduced coordination:** Teams don't need to coordinate version bumps; they happen automatically
- **Clear release gates:** Explicit tagging ceremony enforces discipline and intentionality
- **Branching clarity:** Three tiers (dev/release/stable) provide unambiguous handoff points
- **Flexible pre-releases:** ContinuousDeployment on dev enables continuous alpha testing

### Negative
- **Learning curve:** Team must understand the three-tier branching strategy and GitVersion concepts
- **Tag naming discipline:** Strict tag prefix (`release/version-demo_v...`) must be enforced; typos break versioning
- **Limited flexibility:** Cannot release directly from dev; release branches are mandatory
- **Cherry-pick overhead:** Hotfixes require manual cherry-picks back to dev (but necessary for sync)
- **Dependency on GitVersion:** Project is locked to GitVersion v6; major version upgrades require careful planning

---

## Alternatives Considered

### 1. Simpler Tag Scheme (e.g., `v1.0.0`)
- **Rejected:** Cannot disambiguate release tags from other git metadata; unsuitable for multi-repo scenarios

### 2. Auto-Release from Release Branches
- **Rejected:** Loses intentionality; a beta commit would instantly become a stable release without a deliberate gate

### 3. Single Branch with Tag-Driven Versioning
- **Rejected:** Conflates development and release; no way to run parallel alpha + beta cycles

### 4. Manual SemVer Bumps in Code
- **Rejected:** Requires coordination, is error-prone, and defeats the purpose of git-driven versioning

---

## Implementation Notes

- **GitVersion Configuration:** See [GitVersion.yml](../../GitVersion.yml) for branch-specific modes, tag patterns, and increment rules
- **CI Integration:** The [ci-build.yml](.github/workflows/ci-build.yml) workflow computes version via GitVersion and injects it into both frontend (`src/build-info.json`) and Rust backend
- **Reproducible Local Testing:** Local builds use fallback version `0.1.0-local` (see [build-info.json](src/build-info.json))
- **Release Process:** Releases follow: `dev` → `release/version-demo_vX.Y` branch → `release/version-demo_vX.Y.Z` tag → stable version

---

## Related Decisions

- Branching strategy (dev, release/vX.Y, tags) is interdependent with this versioning model
- Build metadata injection (buildId, commitSha) requires CI/build script integration

