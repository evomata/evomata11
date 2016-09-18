#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate bincode;
extern crate rand;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;
#[macro_use]
extern crate enum_primitive;
extern crate num;
extern crate itertools;
extern crate glowygraph as gg;
extern crate glium;
extern crate nalgebra;
extern crate noise;
extern crate mli;
extern crate num_cpus;
extern crate crossbeam;

mod cell;
mod fluid;
mod grid;

use gg::render2::*;
use nalgebra as na;
use nalgebra::ToHomogeneous;
use num::One;

use std::sync::mpsc::channel;

use rand::{Isaac64Rng, SeedableRng};

const DEFAULT_SCREEN_HEX_RATIO: f32 = 64.0;

const GRID_WIDTH: usize = 192 * 3 / 2;
const GRID_HEIGHT: usize = 125 * 3 / 2;

// TODO: Figure out when lines are used and set it correctly.
const SCROLL_LINES_RATIO: f32 = 0.707;
const SCROLL_PIXELS_RATIO: f32 = 0.707;

const GRID_SPAWN_MULTIPLY: f64 = 1.25;

fn main() {
    let mut rng = Isaac64Rng::from_seed(&[2, 5, 3, 12454]);
    let mut g = grid::Grid::new(GRID_WIDTH, GRID_HEIGHT, &mut rng);
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new().with_vsync().build_glium().unwrap();
    // window.set_cursor_state(glium::glutin::CursorState::Hide).ok().unwrap();
    let glowy = Renderer::new(&display);

    let mut screen_hex_ratio = DEFAULT_SCREEN_HEX_RATIO;

    let mut center = (0.5 * GRID_WIDTH as f32, 0.5 * GRID_HEIGHT as f32);
    let mut last_mouse_pos = (0, 0);
    let mut mouse_pressed = false;

    let mut rendering_enabled = true;

    loop {
        use glium::Surface;

        // // Get dimensions
        let dims = display.get_framebuffer_dimensions();
        // Multiply this by width coordinates to get normalized screen coordinates.
        let hscale = dims.1 as f32 / dims.0 as f32;

        // Don't even vsync if rendering is disabled.
        let mut target = if rendering_enabled {
            Some(display.draw())
        } else {
            None
        };
        target.as_mut().map_or_else(|| {}, |t| t.clear_color(0.0, 0.0, 0.0, 1.0));

        // Ratio of width/height in a 2d circle tight-pack or a hex grid.
        let width_height_ratio = 0.86602540378;

        let (screen_width, screen_height) = (screen_hex_ratio / hscale, screen_hex_ratio);
        let (hex_per_width_pixel, hex_per_height_pixel) =
            (screen_width / dims.0 as f32, screen_height / width_height_ratio / dims.1 as f32);

        let center_mouse_coord = (dims.0 as f32 / 2.0, dims.1 as f32 / 2.0);

        let projection =
            [[1.0 / screen_width, 0.0, 0.0], [0.0, 1.0 / screen_height, 0.0], [0.0, 0.0, 1.0]];

        if rendering_enabled {

            let (render_tx, render_rx) = channel();

            let numcpus = num_cpus::get();

            crossbeam::scope(|scope| {
                let g = &g;
                // Render nodes
                for i in 0..numcpus {
                    let render_tx = render_tx.clone();
                    scope.spawn(move || {
                            let mut v = Vec::new();
                            for x in 0..GRID_WIDTH {
                                for y in (GRID_HEIGHT * i / numcpus)..(GRID_HEIGHT * (i + 1) / numcpus) {
                                    append_circle(&mut v,
                                                  0.6,
                                                  0.6,
                                                  g.hex(x, y).color(),
                                                  &na::Isometry2::new(na::Vector2::new(if y % 2 == 0 {
                                                                                           1.5
                                                                                       } else {
                                                                                           0.5
                                                                                       } +
                                                                                       2.0 *
                                                                                       (x as f32 -
                                                                                        center.0),
                                                                                       width_height_ratio *
                                                                                       (2.0 *
                                                                                        (y as f32 -
                                                                                         center.1 +
                                                                                         0.5))),
                                                                      na::Vector1::new(0.0))
                                                      .to_homogeneous());

                                    if let Some(ref c) = g.hex(x, y).cell {
                                        append_circle(&mut v,
                                                      0.3,
                                                      0.3,
                                                      c.color(),
                                                      &na::Isometry2::new(na::Vector2::new(if y % 2 == 0 {
                                                                                               1.5
                                                                                           } else {
                                                                                               0.5
                                                                                           } +
                                                                                           2.0 *
                                                                                           (x as f32 -
                                                                                            center.0),
                                                                                           width_height_ratio *
                                                                                           (2.0 *
                                                                                            (y as f32 -
                                                                                             center.1 +
                                                                                             0.5))),
                                                                          na::Vector1::new(0.0))
                                                          .to_homogeneous());
                                    }
                                }
                            }
                            render_tx.send(v).unwrap_or_else(|_| panic!("Render channel closed."));
                        });
                }

                for _ in 0..numcpus {
                    glowy.render_qbeziers_flat(target.as_mut().unwrap(),
                                               na::Matrix3::one().as_ref().clone(),
                                               projection,
                                               &render_rx.recv().unwrap_or_else(|e| {
                                                   panic!("Error: Render threads unexpectedly closed: \
                                                           {}",
                                                          e)
                                               })[..]);
                }
            });
        }

        g.cycle(&mut rng);

        // Don't even vsync if rendering is disabled.
        if rendering_enabled {
            target.unwrap().finish().unwrap();
        }

        for ev in display.poll_events() {
            use glium::glutin::{Event, ElementState, MouseButton, MouseScrollDelta,
                                VirtualKeyCode as VKC};
            use std::fs::File;
            match ev {
                Event::Closed => return,
                Event::KeyboardInput(ElementState::Pressed, _, Some(VKC::L)) => {
                    match File::open("gridstate") {
                        Ok(mut f) => {
                            match bincode::serde::deserialize_from(&mut f,
                                                                   bincode::SizeLimit::Infinite) {
                                Ok(t) => {
                                    g = t;
                                    println!("Successfully loaded grid from \"gridstate\".");
                                }
                                Err(e) => println!("Failed to load grid state: {}", e),
                            }
                        }
                        Err(e) => println!("Unable to create file \"gridstate\": {}", e),
                    }
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VKC::W)) => {
                    match File::create("gridstate") {
                        Ok(mut f) => {
                            match bincode::serde::serialize_into(&mut f,
                                                                 &g,
                                                                 bincode::SizeLimit::Infinite) {
                                Ok(()) => println!("Successfully saved grid to \"gridstate\"."),
                                Err(e) => println!("Failed to save grid state: {}", e),
                            }
                        }
                        Err(e) => println!("Unable to open file \"gridstate\": {}", e),
                    }
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VKC::U)) => {
                    g.spawn_rate *= GRID_SPAWN_MULTIPLY;
                    println!("New spawn rate: {}", g.spawn_rate);
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VKC::D)) => {
                    g.spawn_rate /= GRID_SPAWN_MULTIPLY;
                    println!("New spawn rate: {}", g.spawn_rate);
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VKC::P)) => {
                    g.movement_cost += 1;
                    println!("New movement cost: {}", g.movement_cost);
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VKC::O)) => {
                    if g.movement_cost > 0 {
                        g.movement_cost -= 1;
                    }
                    println!("New movement cost: {}", g.movement_cost);
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VKC::R)) => {
                    g.randomize(&mut rng);
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VKC::S)) => {
                    g.spawning = !g.spawning;
                    println!("Spawning {}",
                             if g.spawning {
                                 "enabled"
                             } else {
                                 "disabled"
                             });
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(VKC::T)) => {
                    rendering_enabled = !rendering_enabled;
                    println!("Rendering {}",
                             if rendering_enabled {
                                 "enabled"
                             } else {
                                 "disabled"
                             });
                }
                Event::MouseWheel(MouseScrollDelta::LineDelta(_, lines), _) => {
                    screen_hex_ratio -= lines * SCROLL_LINES_RATIO;
                }
                Event::MouseWheel(MouseScrollDelta::PixelDelta(_, pixels), _) => {
                    screen_hex_ratio -= pixels * SCROLL_PIXELS_RATIO;
                }
                Event::MouseMoved(x, y) => {
                    if mouse_pressed {
                        center.0 -= hex_per_width_pixel * (x - last_mouse_pos.0) as f32;
                        center.1 += hex_per_height_pixel * (y - last_mouse_pos.1) as f32;
                    }
                    last_mouse_pos = (x, y);
                }
                Event::MouseInput(ElementState::Released, MouseButton::Left) => {
                    let relative_coord = (last_mouse_pos.0 as f32 - center_mouse_coord.0,
                                          last_mouse_pos.1 as f32 - center_mouse_coord.1);

                    let hex = (center.0 + relative_coord.0 * hex_per_width_pixel,
                               center.1 - relative_coord.1 * hex_per_height_pixel);
                    // Adjust the width based on the height.
                    let hex = (if hex.1 as isize % 2 == 0 {
                        hex.0 - 0.25
                    } else {
                        hex.0 + 0.25
                    },
                               hex.1);
                    if hex.0 > 0.0 && hex.0 < GRID_WIDTH as f32 && hex.1 > 0.0 &&
                       hex.1 < GRID_HEIGHT as f32 {
                        println!("{:?}", g.hex(hex.0 as usize, hex.1 as usize));
                    }
                }
                Event::MouseInput(state, MouseButton::Right) => {
                    match state {
                        ElementState::Pressed => mouse_pressed = true,
                        ElementState::Released => mouse_pressed = false,
                    }
                }
                Event::Focused(_) => {
                    // Always stop handling mouse press if we loose or gain focus.
                    mouse_pressed = false;
                }
                _ => (),
            }
        }
    }
}

fn append_circle(v: &mut Vec<QBezier>,
                 radius: f32,
                 circle_scale: f32,
                 color: [f32; 4],
                 modelview: &na::Matrix3<f32>) {
    let transform = |n: [f32; 2]| {
        let na::Vector3 { x, y, .. } =
            *modelview * na::Vector3::new(n[0] * circle_scale, n[1] * circle_scale, 1.0);
        [x, y]
    };
    v.extend([QBezier {
                  position0: transform([0.0, -1.0]),
                  position1: transform([0.5773502691896256, -1.0]),
                  position2: transform([0.8660254037844386, -0.5]),
                  inner_color0: color,
                  inner_color1: color,
                  falloff_color0: color,
                  falloff_color1: color,
                  falloff0: 0.25,
                  falloff1: 0.25,
                  falloff_radius0: radius / 2.0,
                  falloff_radius1: radius / 2.0,
                  inner_radius0: radius / 2.0,
                  inner_radius1: radius / 2.0,
              },
              QBezier {
                  position0: transform([0.8660254037844386, -0.5]),
                  position1: transform([1.1547005383792515, 0.0]),
                  position2: transform([0.8660254037844387, 0.5]),
                  inner_color0: color,
                  inner_color1: color,
                  falloff_color0: color,
                  falloff_color1: color,
                  falloff0: 0.25,
                  falloff1: 0.25,
                  falloff_radius0: radius / 2.0,
                  falloff_radius1: radius / 2.0,
                  inner_radius0: radius / 2.0,
                  inner_radius1: radius / 2.0,
              },
              QBezier {
                  position0: transform([0.8660254037844387, 0.5]),
                  position1: transform([0.5773502691896261, 1.0]),
                  position2: transform([0.0, 1.0]),
                  inner_color0: color,
                  inner_color1: color,
                  falloff_color0: color,
                  falloff_color1: color,
                  falloff0: 0.25,
                  falloff1: 0.25,
                  falloff_radius0: radius / 2.0,
                  falloff_radius1: radius / 2.0,
                  inner_radius0: radius / 2.0,
                  inner_radius1: radius / 2.0,
              },
              QBezier {
                  position0: transform([0.0, 1.0]),
                  position1: transform([-0.5773502691896254, 1.0]),
                  position2: transform([-0.8660254037844384, 0.5]),
                  inner_color0: color,
                  inner_color1: color,
                  falloff_color0: color,
                  falloff_color1: color,
                  falloff0: 0.25,
                  falloff1: 0.25,
                  falloff_radius0: radius / 2.0,
                  falloff_radius1: radius / 2.0,
                  inner_radius0: radius / 2.0,
                  inner_radius1: radius / 2.0,
              },
              QBezier {
                  position0: transform([-0.8660254037844384, 0.5]),
                  position1: transform([-1.1547005383792515, 0.0]),
                  position2: transform([-0.866025403784439, -0.5]),
                  inner_color0: color,
                  inner_color1: color,
                  falloff_color0: color,
                  falloff_color1: color,
                  falloff0: 0.25,
                  falloff1: 0.25,
                  falloff_radius0: radius / 2.0,
                  falloff_radius1: radius / 2.0,
                  inner_radius0: radius / 2.0,
                  inner_radius1: radius / 2.0,
              },
              QBezier {
                  position0: transform([-0.866025403784439, -0.5]),
                  position1: transform([-0.5773502691896263, -1.0]),
                  position2: transform([-0.0, -1.0]),
                  inner_color0: color,
                  inner_color1: color,
                  falloff_color0: color,
                  falloff_color1: color,
                  falloff0: 0.25,
                  falloff1: 0.25,
                  falloff_radius0: radius / 2.0,
                  falloff_radius1: radius / 2.0,
                  inner_radius0: radius / 2.0,
                  inner_radius1: radius / 2.0,
              }]
        .into_iter());
}
