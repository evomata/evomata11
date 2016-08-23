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

mod cell;
mod fluid;
mod grid;

use gg::render2::*;

fn main() {
    let mut g = grid::Grid::new(100, 180);
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new().with_vsync().build_glium().unwrap();
    // window.set_cursor_state(glium::glutin::CursorState::Hide).ok().unwrap();
    let glowy = Renderer::new(&display);

    loop {
        use glium::Surface;

        // Get dimensions
        let dims = display.get_framebuffer_dimensions();
        let hscale = dims.1 as f32 / dims.0 as f32;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // Render nodes
        draw_circle(&glowy,
                    &mut target,
                    [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
                    [[hscale, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);

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

fn draw_circle(rend: &Renderer,
               target: &mut glium::Frame,
               modelview: [[f32; 3]; 3],
               projection: [[f32; 3]; 3]) {

    rend.render_qbeziers_flat(target,
                              modelview,
                              projection,
                              &[QBezier {
                                    position0: [0.0, -1.0],
                                    position1: [0.5773502691896256, -1.0],
                                    position2: [0.8660254037844386, -0.5],
                                    inner_color0: [1.0, 1.0, 1.0, 0.2],
                                    inner_color1: [1.0, 1.0, 1.0, 0.2],
                                    falloff_color0: [1.0, 1.0, 1.0, 0.2],
                                    falloff_color1: [1.0, 1.0, 1.0, 0.2],
                                    falloff0: 1.0,
                                    falloff1: 1.0,
                                    falloff_radius0: 0.1,
                                    falloff_radius1: 0.1,
                                    inner_radius0: 0.1,
                                    inner_radius1: 0.1,
                                },
                                QBezier {
                                    position0: [0.8660254037844386, -0.5],
                                    position1: [1.1547005383792515, 0.0],
                                    position2: [0.8660254037844387, 0.5],
                                    inner_color0: [1.0, 1.0, 1.0, 0.2],
                                    inner_color1: [1.0, 1.0, 1.0, 0.2],
                                    falloff_color0: [1.0, 1.0, 1.0, 0.2],
                                    falloff_color1: [1.0, 1.0, 1.0, 0.2],
                                    falloff0: 1.0,
                                    falloff1: 1.0,
                                    falloff_radius0: 0.1,
                                    falloff_radius1: 0.1,
                                    inner_radius0: 0.1,
                                    inner_radius1: 0.1,
                                },
                                QBezier {
                                    position0: [0.8660254037844387, 0.5],
                                    position1: [0.5773502691896261, 1.0],
                                    position2: [0.0, 1.0],
                                    inner_color0: [1.0, 1.0, 1.0, 0.2],
                                    inner_color1: [1.0, 1.0, 1.0, 0.2],
                                    falloff_color0: [1.0, 1.0, 1.0, 0.2],
                                    falloff_color1: [1.0, 1.0, 1.0, 0.2],
                                    falloff0: 1.0,
                                    falloff1: 1.0,
                                    falloff_radius0: 0.1,
                                    falloff_radius1: 0.1,
                                    inner_radius0: 0.1,
                                    inner_radius1: 0.1,
                                },
                                QBezier {
                                    position0: [0.0, 1.0],
                                    position1: [-0.5773502691896254, 1.0],
                                    position2: [-0.8660254037844384, 0.5],
                                    inner_color0: [1.0, 1.0, 1.0, 0.2],
                                    inner_color1: [1.0, 1.0, 1.0, 0.2],
                                    falloff_color0: [1.0, 1.0, 1.0, 0.2],
                                    falloff_color1: [1.0, 1.0, 1.0, 0.2],
                                    falloff0: 1.0,
                                    falloff1: 1.0,
                                    falloff_radius0: 0.1,
                                    falloff_radius1: 0.1,
                                    inner_radius0: 0.1,
                                    inner_radius1: 0.1,
                                },
                                QBezier {
                                    position0: [-0.8660254037844384, 0.5],
                                    position1: [-1.1547005383792515, 0.0],
                                    position2: [-0.866025403784439, -0.5],
                                    inner_color0: [1.0, 1.0, 1.0, 0.2],
                                    inner_color1: [1.0, 1.0, 1.0, 0.2],
                                    falloff_color0: [1.0, 1.0, 1.0, 0.2],
                                    falloff_color1: [1.0, 1.0, 1.0, 0.2],
                                    falloff0: 1.0,
                                    falloff1: 1.0,
                                    falloff_radius0: 0.1,
                                    falloff_radius1: 0.1,
                                    inner_radius0: 0.1,
                                    inner_radius1: 0.1,
                                },
                                QBezier {
                                    position0: [-0.866025403784439, -0.5],
                                    position1: [-0.5773502691896263, -1.0],
                                    position2: [-0.0, -1.0],
                                    inner_color0: [1.0, 1.0, 1.0, 0.2],
                                    inner_color1: [1.0, 1.0, 1.0, 0.2],
                                    falloff_color0: [1.0, 1.0, 1.0, 0.2],
                                    falloff_color1: [1.0, 1.0, 1.0, 0.2],
                                    falloff0: 1.0,
                                    falloff1: 1.0,
                                    falloff_radius0: 0.1,
                                    falloff_radius1: 0.1,
                                    inner_radius0: 0.1,
                                    inner_radius1: 0.1,
                                }]);
}
