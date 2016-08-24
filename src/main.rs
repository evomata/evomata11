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

mod cell;
mod fluid;
mod grid;

use gg::render2::*;
use nalgebra as na;
use nalgebra::ToHomogeneous;
use num::One;

use rand::{Isaac64Rng, SeedableRng};

const GRID_WIDTH: usize = 192 / 2;
const GRID_HEIGHT: usize = 124 / 2;

fn main() {
    let mut rng = Isaac64Rng::from_seed(&[1, 2, 3, 4]);
    let mut g = grid::Grid::new(GRID_WIDTH, GRID_HEIGHT, &mut rng);
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new().with_vsync().build_glium().unwrap();
    // window.set_cursor_state(glium::glutin::CursorState::Hide).ok().unwrap();
    let glowy = Renderer::new(&display);

    loop {
        use glium::Surface;

        // // Get dimensions
        // let dims = display.get_framebuffer_dimensions();
        // let hscale = dims.1 as f32 / dims.0 as f32;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let width_height_ratio = 0.86602540378;

        let projection = [[1.0 / (GRID_WIDTH as f32 + 1.0), 0.0, 0.0],
                          [0.0, 1.0 / (GRID_HEIGHT as f32 + 1.0) / width_height_ratio, 0.0],
                          [0.0, 0.0, 1.0]];

        let mut qbeziers = Vec::new();

        // Render nodes
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                append_circle(&mut qbeziers,
                              g.get_hex(x, y).color(),
                              &na::Isometry2::new(na::Vector2::new(if y % 2 == 0 {
                                                                       1.0
                                                                   } else {
                                                                       0.0
                                                                   } +
                                                                   2.0 * (x as f32) +
                                                                   0.5 -
                                                                   (GRID_WIDTH as f32),
                                                                   width_height_ratio *
                                                                   (2.0 * (y as f32 + 0.5) -
                                                                    (GRID_HEIGHT as f32))),
                                                  na::Vector1::new(0.0))
                                  .to_homogeneous());
            }
        }


        glowy.render_qbeziers_flat(&mut target,
                                   na::Matrix3::one().as_ref().clone(),
                                   projection,
                                   &qbeziers[..]);

        g.cycle();

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}

fn append_circle(v: &mut Vec<QBezier>, color: [f32; 4], modelview: &na::Matrix3<f32>) {
    let transform = |n: [f32; 2]| {
        let na::Vector3 { x, y, .. } = *modelview * na::Vector3::new(n[0] * 0.8, n[1] * 0.8, 1.0);
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
                  falloff_radius0: 0.1,
                  falloff_radius1: 0.1,
                  inner_radius0: 0.1,
                  inner_radius1: 0.1,
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
                  falloff_radius0: 0.1,
                  falloff_radius1: 0.1,
                  inner_radius0: 0.1,
                  inner_radius1: 0.1,
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
                  falloff_radius0: 0.1,
                  falloff_radius1: 0.1,
                  inner_radius0: 0.1,
                  inner_radius1: 0.1,
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
                  falloff_radius0: 0.1,
                  falloff_radius1: 0.1,
                  inner_radius0: 0.1,
                  inner_radius1: 0.1,
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
                  falloff_radius0: 0.1,
                  falloff_radius1: 0.1,
                  inner_radius0: 0.1,
                  inner_radius1: 0.1,
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
                  falloff_radius0: 0.1,
                  falloff_radius1: 0.1,
                  inner_radius0: 0.1,
                  inner_radius1: 0.1,
              }]
        .into_iter());
}
