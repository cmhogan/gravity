use clap::Parser;
use glam::DVec3;
use gravity::camera::Camera;
use gravity::physics::{init_random_disk, init_solar_system, step, update_accelerations};
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use std::time::Instant;
use winit::{
    event::{ElementState, Event, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of threads for parallel calculation
    #[arg(short, long, default_value_t = 1)]
    threads: usize,

    /// If true, runs the headless benchmark
    #[arg(long, default_value_t = false)]
    bench: bool,

    /// If true, uses a chaotic random cloud instead of a disk (for --demo)
    #[arg(long, default_value_t = false)]
    chaos: bool,

    /// If true, runs the protostellar disk demo
    #[arg(long, default_value_t = false)]
    demo: bool,

    /// Number of random bodies for the benchmark or demo
    #[arg(short, long, default_value_t = 100)]
    bodies: usize,

    /// Number of points in the orbital trails (default 250)
    #[arg(long, default_value_t = 250)]
    tails: usize,

    /// Number of physics steps to perform per frame (default 1)
    #[arg(short, long, default_value_t = 1)]
    steps: usize,
    }
/// Clears the pixel buffer by zeroing it (black screen)
fn clear_screen(pixels: &mut Pixels) {
    let frame = pixels.frame_mut();
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(&[0, 0, 0, 255]); // RGBA: Black, fully opaque
    }
}

/// Draws a filled circle directly into the pixel buffer
fn draw_circle(pixels: &mut Pixels, center_x: i32, center_y: i32, radius: i32, color: [u8; 4]) {
    if radius <= 0 {
        return;
    }

    let frame = pixels.frame_mut();
    for y in -radius..=radius {
        for x in -radius..=radius {
            if x * x + y * y <= radius * radius {
                let px = center_x + x;
                let py = center_y + y;

                if px >= 0 && px < WIDTH as i32 && py >= 0 && py < HEIGHT as i32 {
                    let index = (py as usize * WIDTH as usize + px as usize) * 4;
                    frame[index..index + 4].copy_from_slice(&color);
                }
            }
        }
    }
}

/// Draws a line using Bresenham's algorithm
fn draw_line(pixels: &mut Pixels, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: [u8; 4]) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    let frame = pixels.frame_mut();

    loop {
        if x0 >= 0 && x0 < WIDTH as i32 && y0 >= 0 && y0 < HEIGHT as i32 {
            let index = (y0 as usize * WIDTH as usize + x0 as usize) * 4;
            frame[index..index + 4].copy_from_slice(&color);
        }

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

/// Draws a dim ecliptic grid overlay on the z=0 plane
fn draw_grid(pixels: &mut Pixels, camera: &Camera) {
    let color = [40, 40, 40, 255]; // Dim dark gray

    // Concentric circles: every 1 AU up to 10 AU, then every 5 AU up to 35 AU
    let mut radii = Vec::new();
    for i in 1..=10 {
        radii.push(i as f64);
    }
    for i in (15..=35).step_by(5) {
        radii.push(i as f64);
    }

    for radius in radii {
        let mut prev_screen: Option<(i32, i32)> = None;
        let segments = 64;
        for j in 0..=segments {
            let angle = (j as f64 / segments as f64) * 2.0 * std::f64::consts::PI;
            let world_pos = DVec3::new(radius * angle.cos(), radius * angle.sin(), 0.0);
            let curr_screen = camera.world_to_screen(world_pos, WIDTH, HEIGHT);
            if let Some((x0, y0)) = prev_screen {
                draw_line(pixels, x0, y0, curr_screen.0, curr_screen.1, color);
            }
            prev_screen = Some(curr_screen);
        }
    }

    // Radial lines every 45° from the origin out to 35.0 AU
    for i in 0..8 {
        let angle = (i as f64 * 45.0).to_radians();
        let start_pos = DVec3::ZERO;
        let end_pos = DVec3::new(35.0 * angle.cos(), 35.0 * angle.sin(), 0.0);
        
        let (sx0, sy0) = camera.world_to_screen(start_pos, WIDTH, HEIGHT);
        let (sx1, sy1) = camera.world_to_screen(end_pos, WIDTH, HEIGHT);
        
        draw_line(pixels, sx0, sy0, sx1, sy1, color);
    }
}

fn get_body_color(index: usize) -> [u8; 4] {
    match index {
        0 => [255, 255, 0, 255],     // Sun: Yellow
        1 => [165, 165, 165, 255],   // Mercury: Grey
        2 => [255, 198, 107, 255],   // Venus: Pale Yellow
        3 => [100, 149, 237, 255],   // Earth: Cornflower Blue
        4 => [255, 69, 0, 255],     // Mars: Red-Orange
        5 => [210, 180, 140, 255],   // Jupiter: Tan/Brown
        6 => [238, 232, 170, 255],   // Saturn: Pale Gold
        7 => [173, 216, 230, 255],   // Uranus: Light Blue
        8 => [65, 105, 225, 255],    // Neptune: Royal Blue
        9 => [224, 255, 255, 255],   // Halley: Light Cyan
        10 => [139, 69, 19, 255],    // Ceres: Saddle Brown
        _ => [200, 200, 200, 255],   // Others: Greyish
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize Rayon thread pool with user-specified thread count
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()
        .unwrap_or_else(|_| println!("Rayon global thread pool already initialized"));

    let mut system = if args.bench {
        // Benchmark mode: Random cloud, no trails needed
        init_random_disk(args.bodies, true, 0)
    } else if args.demo {
        init_random_disk(args.bodies, args.chaos, args.tails)
    } else {
        init_solar_system(args.tails)
    };

    update_accelerations(&mut system);

    if args.bench {
        let dt = 0.001;
        let steps = 1000;
        
        println!("Running benchmark with {} bodies for {} steps...", args.bodies, steps);
        
        let start = Instant::now();
        for _ in 0..steps {
            step(&mut system, dt);
        }
        let total_time = start.elapsed();
        
        let total_time_ms = total_time.as_secs_f64() * 1000.0;
        let mean_step_time_us = (total_time.as_secs_f64() * 1_000_000.0) / steps as f64;
        let n = args.bodies as f64;
        let interactions = n * n * (steps as f64);
        let throughput = interactions / total_time.as_secs_f64();

        println!("--- Benchmark Results ---");
        println!("Total Bodies:    {}", args.bodies);
        println!("Total Steps:     {}", steps);
        println!("Total Time:      {:.2} ms", total_time_ms);
        println!("Mean Step Time:  {:.2} μs", mean_step_time_us);
        println!("Throughput:      {:.2e} interactions/sec", throughput);
        
        return Ok(());
    }
    
    println!(
        "System initialized with {} bodies using {} threads",
        system.masses.len(),
        args.threads
    );

    let mut camera = Camera::new(if args.demo { 100.0 } else { 200.0 });
    let dt = 0.001; // ~8.7 hours per step
    let mut is_paused = false;
    let mut steps_per_frame = args.steps;
    let mut total_years = 2026.2685;
    let mut mouse_pos = (0.0, 0.0);
    let mut is_left_clicked = false;
    let mut is_right_clicked = false;
    let mut modifiers = winit::keyboard::ModifiersState::default();

    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Gravity Simulation")
            .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
            .build(&event_loop)?,
    );

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, Arc::clone(&window));
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut frame_count = 0;
    let mut last_fps_update = Instant::now();

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                elwt.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => {
                if let MouseScrollDelta::LineDelta(_, y) = delta {
                    let zoom_factor = if y > 0.0 { 1.1 } else { 0.9 };
                    camera.zoom(zoom_factor);
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                let new_mouse_pos = (position.x, position.y);
                if is_left_clicked {
                    let dx = new_mouse_pos.0 - mouse_pos.0;
                    let dy = new_mouse_pos.1 - mouse_pos.1;
                    camera.pan(dx, dy);
                }
                if is_right_clicked {
                    let dx = new_mouse_pos.0 - mouse_pos.0;
                    let dy = new_mouse_pos.1 - mouse_pos.1;
                    camera.rotate(dy * 0.005, dx * 0.005);
                }
                mouse_pos = new_mouse_pos;
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                if button == MouseButton::Left {
                    is_left_clicked = state == ElementState::Pressed;
                } else if button == MouseButton::Right {
                    is_right_clicked = state == ElementState::Pressed;
                }
            }
            Event::WindowEvent {
                event: WindowEvent::ModifiersChanged(m),
                ..
            } => {
                modifiers = m.state();
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { event: key_event, .. },
                ..
            } => {
                if let KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state: ElementState::Pressed,
                    ..
                } = key_event
                {
                    match code {
                        KeyCode::Space => is_paused = !is_paused,
                        KeyCode::KeyR => {
                            camera.reset();
                            steps_per_frame = 1;
                        }
                        KeyCode::KeyQ => elwt.exit(),
                        KeyCode::KeyI => camera.zoom(2.0),
                        KeyCode::KeyO => camera.zoom(0.5),
                        KeyCode::BracketLeft => {
                            let shift = modifiers.shift_key();
                            let amount = if shift { 10 } else { 1 };
                            if steps_per_frame > amount {
                                steps_per_frame -= amount;
                            } else {
                                steps_per_frame = 1;
                            }
                        }
                        KeyCode::BracketRight => {
                            let shift = modifiers.shift_key();
                            let amount = if shift { 10 } else { 1 };
                            steps_per_frame += amount;
                        }
                        _ => {}
                    }
                }
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Update FPS in title every second
                frame_count += 1;
                let now = Instant::now();
                if now.duration_since(last_fps_update).as_secs() >= 1 {
                    let fps = frame_count;
                    window.set_title(&format!(
                        "Gravity Simulation | FPS: {} | Steps/Frame: {} | Year: {:.2}", 
                        fps, steps_per_frame, total_years
                    ));
                    frame_count = 0;
                    last_fps_update = now;
                }

                if !is_paused {
                    for _ in 0..steps_per_frame {
                        step(&mut system, dt);
                        system.record_history();
                        total_years += dt;
                    }
                }

                clear_screen(&mut pixels);

                // Draw grid
                draw_grid(&mut pixels, &camera);

                // Draw trails
                for i in 0..system.masses.len() {
                    let color = get_body_color(i);
                    // Dimmer version of the color for trails
                    let trail_color = [
                        (color[0] as f32 * 0.5) as u8,
                        (color[1] as f32 * 0.5) as u8,
                        (color[2] as f32 * 0.5) as u8,
                        255,
                    ];

                    let history = &system.history[i];
                    let mut prev_screen: Option<(i32, i32)> = None;

                    for &h_pos in history {
                        let curr_screen = camera.world_to_screen(h_pos, WIDTH, HEIGHT);
                        
                        if let Some((x0, y0)) = prev_screen {
                            draw_line(&mut pixels, x0, y0, curr_screen.0, curr_screen.1, trail_color);
                        }
                        prev_screen = Some(curr_screen);
                    }
                }

                // Draw bodies
                for i in 0..system.masses.len() {
                    let pos = system.positions[i];
                    let (sx, sy) = camera.world_to_screen(pos, WIDTH, HEIGHT);

                    // Schematic scaling: ensure sun and planets are visible
                    let radius = (system.masses[i].log10() + 8.0).max(1.0) * 1.5;
                    let color = get_body_color(i);

                    draw_circle(&mut pixels, sx, sy, radius as i32, color);
                }

                if let Err(err) = pixels.render() {
                    eprintln!("pixels.render() failed: {}", err);
                    elwt.exit();
                    return;
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}
