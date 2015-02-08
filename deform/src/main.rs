#![feature(core, path)]

extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate drag_controller;
extern crate sdl2_window;

use std::cell::RefCell;

use graphics::ImageSize;
use drag_controller::{
    DragController,
    Drag
};
use opengl_graphics::{ Gl, OpenGL, Texture };
use sdl2_window::Sdl2Window as Window;

fn main() {
    println!("Click in the red square and drag.");
    println!("Toggle grid with G.");
    println!("Reset grid with R.");

    let opengl = OpenGL::_3_2;
    let window = Window::new(
        opengl,
        piston::window::WindowSettings {
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
    let mut grid = graphics::deform::DeformGrid::new(
        [0.0, 0.0, width, height],
        20, 20
    );

    let mut drag = DragController::new();
    let mut draw_grid = true;

    let mut gl = Gl::new(opengl);
    let window = RefCell::new(window);
    for e in piston::events(&window) {
        use piston::event::{ RenderEvent, PressEvent };

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
        if let Some(button) = e.press_args() {
            use piston::input::Button::Keyboard;
            use piston::input::keyboard::Key;

            if button == Keyboard(Key::G) {
                draw_grid = !draw_grid;
                println!("Draw grid {}", draw_grid);
            } else if button == Keyboard(Key::R) {
                grid.reset_control_points();
                grid.reset_vertices_and_texture_coords();
                grid.update();
                println!("Reset grid");
            }
        }
        if let Some(args) = e.render_args() {
            gl.draw(
                [0, 0, args.width as i32, args.height as i32],
                |c, g| {

            graphics::clear(graphics::color::WHITE, g);

            // Draw deformed image.
            grid.draw_image(&image, &c, g);

            if draw_grid {
                // Draw grid.
                grid.draw_vertical_lines(
                    &graphics::Line::new([0.0, 1.0, 0.0, 1.0], 0.5),
                    &c,     
                    g
                );
                grid.draw_horizontal_lines(
                    &graphics::Line::new([0.0, 0.0, 1.0, 1.0], 0.5),
                    &c,
                    g
                );
            }
            
            // Draw rect of the original grid.
            graphics::Rectangle::border([1.0, 0.0, 0.0, 1.0], 1.5)
                .draw([0.0, 0.0, width, height], &c, g);

            // Draw control points.
            let original = graphics::Ellipse::new([1.0, 0.0, 0.0, 0.5]);
            let current = graphics::Ellipse::new([0.0, 0.0, 0.0, 0.5]);
            for i in (0..grid.ps.len()) {
                use graphics::ellipse::circle;

                // Original positions.
                let x = grid.ps[i][0];
                let y = grid.ps[i][1];
                original.draw(circle(x, y, 3.0), &c, g);

                // Current positions.
                let x = grid.qs[i][0];
                let y = grid.qs[i][1];
                current.draw(circle(x, y, 3.0), &c, g);
            };

            });
        } // end render
    }
}

