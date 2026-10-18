---
name: solidcraft-animation-dev
description: Plan and implement SolidCraft's Animation workspace or animation commands in this Rust repository, distinct from assembly Motion Study and the parametric feature timeline.
---

# SolidCraft Animation development

Use this skill for changes to an Animation workspace, storyboards, action tracks, playback, or exports in this repository. Read the root `AGENTS.md` and `CONTRIBUTING.md` first. If the local `plan/` files named by `AGENTS.md` exist, read them too; they are gitignored and may be absent in a fork.

## Establish the boundary

- Inspect `crates/doc/src/motion.rs`, `crates/engine/src/cmd/motion.rs`, `crates/engine/src/cmd/motion_animation.rs`, `crates/ui-egui/src/dialogs_motion.rs`, `crates/ui-egui/src/context_menu.rs`, `crates/ui-egui/src/render_workspace.rs`, and `ROADMAP.md` before proposing new structures. Motion Study already keys joint values, plays, analyzes motion, and exports animation. The feature timeline records parametric modelling history. The workspace menu already has a disabled Animation entry. Do not conflate these timelines.
- This fork's owner permits studying commercial CAD behavior, including direct inspection and reverse-engineering tools, for the fork-only Animation work. Record what was observed and how each behavior was independently implemented. Do not assume that observing behavior grants a licence to copy another product's code, assets, text, identifiers, or files. The root `AGENTS.md` and `CONTRIBUTING.md` still state a stricter clean-room rule; reconcile those files before persisting a fork policy change, and follow upstream's rules for any later upstream submission. Keep SolidCraft names and original command IDs.
- After the owner chooses a scope, prefer a vertical slice that covers saved definitions where needed, deterministic time evaluation without changing design placements, engine commands, a thin UI, and a meaningful regression. Reuse existing occurrence transforms, camera, renderer, and export services where their semantics fit.
- State how animation actions combine with joint poses, occurrence placement, visibility, and the camera. Define stable IDs, interpolation, time units, bounds, and behavior after referenced components are edited or removed before writing the file format.

## Contribution workflow

- Develop on the fork until the Animation feature works end to end, as the owner requested. Check issue and PR claims and follow the upstream contribution process only when preparing an upstream submission. Keep each PR to one coherent feature slice.
- Every user-visible operation is a `CommandSpec` with JSON parameters and measured tests. Programmatic execution must not open dialogs. Follow the L0–L5 crate layering, panic avoidance, and wasm requirements.
- When new data is saved, add one file under `crates/doc/src/format/pending/` as prescribed by `CONTRIBUTING.md`; do not hand-edit format version history.
- For visible behavior, capture an actual small, public-safe screenshot or recording. Run `cargo xtask ci` before committing when the toolchain is available. Report any gate that could not run, and check the upstream and PR state before handoff.
