# Gravity: High-Precision N-Body Simulator

A high-performance, low-level 3D orbital mechanics simulation optimized for Apple Silicon (M2). This project implements a data-oriented, symplectic physics engine capable of simulating millions of gravitational interactions per second.

## 🚀 Features

- **Symplectic Solver:** Uses the **Velocity Verlet** (Kick-Drift-Kick) integrator to ensure long-term energy conservation and orbital stability.
- **Apple Silicon Optimized:**
  - **AArch64 NEON SIMD:** Hand-written intrinsics to calculate gravitational forces in parallel on the CPU.
  - **Rayon Parallelism:** Automatically scales the $O(n^2)$ physics loop across all available CPU cores.
- **Data-Driven Architecture:**
  - **Structure of Arrays (SoA):** Memory layout optimized for L1/L2 cache efficiency.
  - **TOML Scenarios:** Define entire star systems, including masses, vectors, and colors, via external configuration files.
- **Interactive 3D Visualizer:**
  - **Low-Level Rendering:** Directly manipulates a raw pixel buffer via the `pixels` crate.
  - **3D Camera:** Supports real-time pitch, yaw, pan, and zoom.
  - **Orbital Trails:** Persistent visual history of planetary paths using Bresenham's line algorithm.
  - **Ecliptic Grid:** A projected reference frame for depth perception in the 3D void.

## 🛠 Controls

| Key | Action |
|-----|--------|
| **Space** | Pause / Resume Simulation |
| **R** | Reset Camera View & Speed |
| **Q** | Quit |
| **I / O** | Zoom In / Out (2x) |
| **[ / ]** | Adjust Simulation Speed (Steps per frame) |
| **Shift + [ / ]** | Adjust Speed by 10x |
| **Left-Click + Drag** | Pan View |
| **Right-Click + Drag** | Rotate View (3D) |
| **Scroll / Pinch** | Smooth Zoom |

## 🏁 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- A Mac with Apple Silicon (M1/M2/M3) is recommended for SIMD acceleration.

### Run the Solar System
The default scenario uses real-time NASA JPL Horizons state vectors for the major planets, Ceres, and Halley's Comet.
```bash
cargo run --release
```

### Run the Protostellar Disk Demo
Visualize a chaotic cloud of 200+ bodies clumping and interacting.
```bash
cargo run --release -- --demo --bodies 200
```
*Add `--chaos` for a fully random 3D cloud instead of a rotating disk.*

### Benchmarking
Stress-test your M2 CPU to see the raw throughput (interactions per second).
```bash
cargo run --release -- --bench --bodies 1000 --threads 8
```

## 📂 Project Structure

- `src/main.rs`: Entry point, CLI, and event loop.
- `src/physics.rs`: The SIMD-accelerated gravity kernel and Verlet integrator.
- `src/system.rs`: The SoA data structures and history management.
- `src/camera.rs`: 3D-to-2D projection and camera controls.
- `src/config.rs`: TOML scenario loading and momentum balancing.
- `scenarios/`: Directory for `.toml` star system definitions.

## 🔬 Mathematical Details

The simulation uses normalized units to maintain floating-point precision:
- **Mass:** Solar Masses ($M_\odot$)
- **Distance:** Astronomical Units (AU)
- **Time:** Years (yr)
- **Gravitational Constant:** $G = 4\pi^2$

Softening ($\epsilon = 10^{-3}$) is applied to prevent numerical singularities during close encounters.
