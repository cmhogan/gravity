# N-Body Gravity Simulation Plan (M2 MacBook Air)

This plan outlines the implementation of a high-precision, low-level orbital mechanics simulation optimized for Apple Silicon (M2).

## Dependency Legend
- **[INDEPENDENT]**: Can be started immediately.
- **[PARALLEL OK]**: Can be done alongside other tasks in the same phase.
- **[BLOCKER]**: Must be completed before subsequent dependent tasks can begin.

---

## Phase 1: Foundation & Data-Oriented Setup
- [x] **Task 1.1: Initialize Rust Project** [BLOCKER]
- [x] **Task 1.2: Implement SoA Architecture & CLI** [BLOCKER]

## Phase 2: High-Precision Physics Engine
- [x] **Task 2.1: Parallel Gravitational Force Calculation** [PARALLEL OK]
- [x] **Task 2.2: Velocity Verlet (Symplectic) Integrator** [BLOCKER]
- [x] **Task 2.3: Inner Solar System Initialization** [PARALLEL OK]

## Phase 3: Low-Level Pixel Visualization
- [x] **Task 3.1: Graphics Boilerplate (pixels + winit)** [BLOCKER]
- [x] **Task 3.2: Coordinate Mapping & Camera** [PARALLEL OK]
- [x] **Task 3.3: Schematic Scanline Circle Rendering** [PARALLEL OK]

## Phase 4: Integration & Interactive Loop
- [x] **Task 4.1: The Simulation Loop** [BLOCKER]
- [x] **Task 4.2: Input Handling** [PARALLEL OK]

## Phase 5: Robust Validation Suite
- [x] **Task 5.1: Physics Invariants Verification** [PARALLEL OK]
- [x] **Task 5.2: Camera Projection Correctness** [PARALLEL OK]
- [x] **Task 5.3: System State Integrity** [PARALLEL OK]
- [x] **Task 5.4: Long-Term Integration Test** [BLOCKER]

## Phase 6: M2 Optimization & Polish
- [x] **Task 6.1: NEON SIMD Acceleration** [INDEPENDENT]
- [x] **Task 6.2: Orbital Trail Buffer** [INDEPENDENT]

## Phase 7: True 3D Visualization
- [x] **Task 7.1: Real Orbital Inclinations** [BLOCKER]
- [x] **Task 7.2: Rotatable 3D Camera** [BLOCKER]
- [x] **Task 7.3: 3D Input Handling** [PARALLEL OK]
- [x] **Task 7.4: Ecliptic Grid Overlay** [INDEPENDENT]

## Phase 8: High-Performance Benchmarking
- [x] **Task 8.1: CLI --bench Flag** [BLOCKER]
- [x] **Task 8.2: Headless Benchmark Loop** [BLOCKER]
- [x] **Task 8.3: Performance Reporting** [PARALLEL OK]

## Phase 9: Protostellar Disk Demo
- [x] **Task 9.1: CLI --demo Flag** [BLOCKER]
    - **Description:** Update `Args` in `main.rs` to include a `--demo` boolean flag.
    - **Model:** Fast
- [x] **Task 9.2: Random Disk Initialization** [BLOCKER]
    - **Description:** Implement a `init_random_disk(n: usize)` helper in `src/physics.rs` that populates `SystemState` with $N$ bodies in a random distribution.
    - **Model:** Fast
- [x] **Task 9.3: Demo Visualization Integration** [PARALLEL OK]
    - **Description:** When `--demo` is passed, initialize the simulation with the random disk and launch the `winit` window with a zoomed-out camera.
    - **Model:** Fast
