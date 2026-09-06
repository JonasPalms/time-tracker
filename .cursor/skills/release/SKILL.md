---
name: release
description: >-
  Cut a TimeTracker desktop release. Use when the user asks to release, bump
  the version, write release notes, tag vX.Y.Z, ship a GitHub/Tauri update, or
  include a release in a pull request to main. A release is the version bump on
  main plus the vX.Y.Z tag.
---

# Release TimeTracker

A release is **one workflow**: version bump on `main` → **tag**. The tag is what CI, GitHub Releases, and the in-app updater key off. Stopping after only committing the bump is not a release.

The tag is a git ref, not a source file. Do not write `vX.Y.Z` anywhere in the tree.

| What | Where | Example |
|---|---|---|
| App version | `src-tauri/tauri.conf.json` → `version` | `0.5.17` |
| Changelog heading | `CHANGELOG.md` → `## [0.5.17] - YYYY-MM-DD` | same number, no `v` |
| Git tag | annotated tag on the release commit, after it is on `main` | `v0.5.17` |
| CI | `.github/workflows/release.yml` on push of `v*` | draft GitHub release |

Do **not** bump `package.json` or `src-tauri/Cargo.toml` (`0.1.0` is unused).
Do **not** use `swift-v*` tags (parked native experiment).

The tag name is always `v` + `tauri.conf.json` version. The updater looks up notes by that number (`getChangelogSections` strips a leading `v`).

Not every merge to `main` is a release. Only tag when the version and changelog heading landed on `main`.

## Standard: put the release in the PR

Ship the bump on the same branch as the work (last commit), then tag after merge. Do not ask whether to include the bump when the user asked to release or ship. Do not tag the feature branch.

1. Confirm the branch is otherwise ready to merge.
2. Read `src-tauri/tauri.conf.json` and `git tag -l 'v0.5.*' 'v0.6.*'` for the current version.
3. Choose the next semver from commits since the last release tag:
   - user-facing fix → patch
   - user-facing feature → minor
   - breaking behavior → major
   - tooling-only → patch
4. Set `version` in `src-tauri/tauri.conf.json`.
5. Move `## [Unreleased]` notes in `CHANGELOG.md` to `## [X.Y.Z] - <today>` (Added / Changed / Fixed). Leave an empty `## [Unreleased]` above it. Date is today from the conversation.
6. Run `pnpm check`.
7. Commit **only** those two files as `chore: release vX.Y.Z`.
8. Push the branch. Open or update the PR to `main`.
9. Push / merge **only if the user asked to push, merge, or ship**.
10. After the PR is on `main`, tag **that** release commit (not the merge commit unless they are the same) and push the tag:

```bash
git checkout main
git pull
git tag -a "vX.Y.Z" -m "vX.Y.Z"
git push origin "vX.Y.Z"
```

Never `--force` a release tag. Never tag a commit that is not the version bump. Never push the tag before `main` contains that commit.

## Fallback: release from `main`

Use this for a hotfix already on `main`, or when there is no feature PR.

1. Confirm `main` and a clean tree (or only the intended release edits).
2. Follow steps 2–7 above.
3. Tag that commit immediately, then push `main` and the tag **only if the user asked to push or ship**:

```bash
git tag -a "vX.Y.Z" -m "vX.Y.Z"
git push origin main
git push origin "vX.Y.Z"
```

## After the tag is pushed

- Actions builds Apple Silicon and Intel and opens a draft `TimeTracker vX.Y.Z`.
- Publishing the draft ships updater artifacts (`latest.json`).
- In-app notes come from `CHANGELOG.md`, not the GitHub release body.
