extern crate shader_version;
extern crate graphics;
extern crate event;
extern crate input;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate drag_controller;

use std::cell::RefCell;
use opengl_graphics::{
    Gl,
    Texture,
};
use sdl2_window::Sdl2Window;
use event::{ Events, WindowSettings };
use graphics::{
    AddBorder,
    AddEllipse,
    AddRectangle,
    AddColor,
    Context,
    Draw,
    ImageSize,
};
use graphics::deform::DeformGrid;
use drag_controller::{
    DragController,
    Drag
};
use event::{
    PressEvent,
    RenderEvent,
};
use input::{
    keyboard,
    Keyboard,
};

fn main() {
    println!("Click in the red square and drag.");
    println!("Toggle grid with G.");
    println!("Reset grid with R.");

    let opengl = shader_version::opengl::OpenGL_3_2;
    let window = Sdl2Window::new(
        opengl,
        WindowSettings {
            title: "Deform".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4,
        }
    );

    let image = Path::new("./bin/assets/rust-logo.png");
    let image = Texture::from_path(&image).unwrap();

    let (width, height) = image.get_size();
    let width = width as f64;
    let height = height as f64;
    let mut grid = DeformGrid::new(
        [0.0, 0.0, width, height],
        20, 20
    );

    let ref mut gl = Gl::new(opengl);
    let mut drag = DragController::new();
    let mut draw_grid = true;
    let window = RefCell::new(window);
    for e in Events::new(&window) {
        drag.event(&e, |action| {
            match action {
                Drag::Start(x, y) => {
                    match grid.hit([x, y]) {
                        None => {
                            // Did not hit deformed grid.
                            grid.add_control_point([x, y]);
                        }
                        Some(pos) => {
                            // Add point to deformed grid.
                            grid.add_control_point(pos);
                            let n = grid.ps.len();
                            grid.set_current(n - 1, [x, y]);
                        }
                    }
                    grid.update();
                    true
                }
                Drag::Move(x, y) => {
                    let n = grid.ps.len();
                    grid.set_current(n - 1, [x, y]);
                    grid.update();
                    true
                }
                Drag::End(_, _) => false,
                // Continue dragging when receiving focus.
                Drag::Interrupt => true,
            }
        });
        e.press(|button| {
            if button == Keyboard(keyboard::G) {
                draw_grid = !draw_grid;
                println!("Draw grid {}", draw_grid);
            } else if button == Keyboard(keyboard::R) {
                grid.reset_control_points();
                grid.reset_vertices_and_texture_coords();
                grid.update();
                println!("Reset grid");
            }
        });
        e.render(|args| {
            gl.viewport(0, 0, 
                        args.width as i32, args.height as i32);
            let c = Context::abs(
                args.width as f64,
                args.height as f64
            );
            // Clear background.
            c.rgb(1.0, 1.0, 1.0).draw(gl);

            // Draw deformed image.
            grid.draw_image(&c, gl, &image);

            if draw_grid {
                // Draw grid.
                grid.draw_vertical_lines(
                    &c.rgb(0.0, 1.0, 0.0),
                    gl, 1.0
                );
                grid.draw_horizontal_lines(
                    &c.rgb(0.0, 0.0, 1.0),
                    gl, 1.0
                );
            }
            
            // Draw rect of the original grid.
            c.rgb(1.0, 0.0, 0.0)
            .rect(0.0, 0.0, width, height)
            .border_width(3.0)
            .draw(gl);

            // Draw control points.
            for i in range(0, grid.ps.len()) {
                // Original positions.
                let x = grid.ps[i][0];
                let y = grid.ps[i][1];
                c.rgba(1.0, 0.0, 0.0, 0.5)
                .circle(x, y, 3.0)
                .draw(gl);

                // Current positions.
                let x = grid.qs[i][0];
                let y = grid.qs[i][1];
                c.rgba(0.0, 0.0, 0.0, 0.5)
                .circle(x, y, 3.0)
                .draw(gl)
            };
        });
    }
}

