# Gravity Movie Mode & Auto-Zoom Plan

This plan tracks the implementation of headless rendering (movie mode) and automatic camera framing for the Gravity simulation.

## Phase 1: Infrastructure & Configuration
- [x] **Dependency Update:** Add the `image` crate to `Cargo.toml`.
- [x] **CLI Expansion:** Update `Args` struct in `src/main.rs` to include movie rendering parameters.
- [x] **Headless Initialization:** Drawing functions refactored to work with raw byte buffers.

## Phase 2: The "Director" (Auto-Zoom)
- [x] **Bounding Box Logic:** Implement `fit_to_system` in `src/camera.rs`.
- [x] **Scale & Offset Calculation:** Implemented in `fit_to_system`.
- [x] **Initial Framing:** `fit_to_system` called in `main.rs` for both modes.

## Phase 3: The Render Engine
- [x] **Headless Loop:** `render_mode` function implemented in `src/main.rs`.
- [x] **PNG Export:** Implemented via `image` crate in `render_mode`.
- [x] **Progress Feedback:** Basic progress printing added to `render_mode`.

## Phase 4: Final Polish & Documentation
- [x] **FFmpeg Integration:** README updated with the command.
- [x] **Directory Handling:** `render_mode` creates the output directory.
- [x] **Verification:** Verified with a 10-frame test render.
