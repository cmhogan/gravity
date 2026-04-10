# Gravity: High-Precision N-Body Simulator

A high-performance, low-level 3D orbital mechanics simulation optimized for Apple Silicon (M2). This project implements a data-oriented, symplectic physics engine capable of simulating millions of gravitational interactions per second.

## 🚀 Features

- **Symplectic Solver:** Uses the **Velocity Verlet** (Kick-Drift-Kick) integrator to ensure long-term energy conservation and orbital stability.
- **Apple Silicon Optimized:**
  - **AArch64 NEON SIMD:** Hand-written intrinsics to calculate gravitational forces in parallel on the CPU.
  - **Rayon Parallelism:** Automatically scales the $O(n^2)$ physics loop across all available CPU cores.
- **Data-Driven Architecture:**
  - **Structure of Arrays (SoA):** Memory layout optimized for L1/L2 cache efficiency.
  - **Scenario Recipes:** Define entire star systems or procedural galaxies via TOML configuration files.
- **Interactive 3D Visualizer:**
  - **Low-Level Rendering:** Directly manipulates a raw pixel buffer via the `pixels` crate.
  - **3D Camera:** Supports real-time pitch, yaw, pan, and zoom with intelligent auto-framing.
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

## ✨ Featured Scenarios

### 🌌 The Great Galactic Collision
A procedural interaction between two disk galaxies, demonstrating tidal stripping and the formation of galactic bridges and tails.
```bash
cargo run --release -- --scenario scenarios/galaxy_collision.toml --steps 20 --tails 1000
```

### ♾️ Stable Three-Body Figure-Eight
A mathematical marvel: three equal masses chasing each other on a single periodic path. This serves as a "soak test" for the integrator's stability.
```bash
cargo run --release -- --scenario scenarios/figure_eight.toml --tails 1000
```

### 🪐 Protostellar Disk
Visualize a chaotic cloud of 200+ bodies clumping and interacting.
```bash
cargo run --release -- --demo --bodies 200
```
*Add `--chaos` for a fully random 3D cloud instead of a rotating disk.*

### 🎬 Movie Mode (Headless Rendering)
Gravity can render high-resolution frames to disk for creating cinematic movies. This mode skips window creation and runs at maximum CPU speed.

```bash
# Render 1200 frames of a galaxy collision at 1080p
cargo run --release -- --scenario scenarios/galaxy_collision.toml --render --width 1920 --height 1080 --limit 1200 --steps 40 --tails 5000
```

To stitch the frames into an MP4 movie (requires [FFmpeg](https://ffmpeg.org/)):
```bash
ffmpeg -framerate 60 -i output/frame_%04d.png -c:v libx264 -pix_fmt yuv420p movie.mp4
```

## 📊 Benchmarking
Stress-test your CPU to see the raw throughput (interactions per second).
```bash
cargo run --release -- --bench --bodies 1000 --threads 8
```

## 📂 Project Structure

- `src/main.rs`: Entry point, CLI, and movie rendering engine.
- `src/physics.rs`: SIMD-accelerated gravity kernel and procedural generation logic.
- `src/camera.rs`: 3D-to-2D projection and auto-zoom logic.
- `src/system.rs`: SoA data structures and history management.
- `src/config.rs`: TOML scenario and galaxy recipe loading.
- `scenarios/`: Star system and collision definitions.

## 🔬 Mathematical Details

The simulation uses normalized units to maintain floating-point precision:
- **Mass:** Solar Masses ($M_\odot$)
- **Distance:** Astronomical Units (AU)
- **Time:** Years (yr)
- **Gravitational Constant:** $G = 4\pi^2$

Softening ($\epsilon = 10^{-3}$) is applied to prevent numerical singularities during close encounters.

## ⚖️ License

This project is licensed under the BSD 3-Clause License. See the [LICENSE](LICENSE) file for details.
