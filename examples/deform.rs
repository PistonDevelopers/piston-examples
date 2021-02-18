extern crate piston_window;
extern crate drag_controller;
extern crate find_folder;
extern crate deform_grid;

use deform_grid::DeformGrid;
use piston_window::*;
use drag_controller::{ DragController, Drag };

fn main() {
    println!("Click in the red square and drag.");
    println!("Toggle grid with G.");
    println!("Reset grid with R.");

    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("piston-example-deform", [300, 300])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .samples(4)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let image = assets.join("rust.png");
    let image = Texture::from_path(
            &mut window.create_texture_context(),
            &image,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

    let (width, height) = image.get_size();
    let width = width as f64;
    let height = height as f64;
    let mut grid = DeformGrid::new(
        [0.0, 0.0, width, height],
        20, 20
    );

    let mut drag = DragController::new();
    let mut draw_grid = true;

    while let Some(e) = window.next() {
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
            use piston_window::Button::Keyboard;

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
        window.draw_2d(&e, |c, g, _| {
            clear(color::WHITE, g);

            // Draw deformed image.
            grid.draw_image(&image, &c.draw_state, c.transform, g);

            if draw_grid {
                // Draw grid.
                grid.draw_vertical_lines(
                    &Line::new([0.0, 1.0, 0.0, 1.0], 0.5),
                    &c.draw_state,
                    c.transform,
                    g
                );
                grid.draw_horizontal_lines(
                    &Line::new([0.0, 0.0, 1.0, 1.0], 0.5),
                    &c.draw_state,
                    c.transform,
                    g
                );
            }

            // Draw rect of the original grid.
            Rectangle::new_border([1.0, 0.0, 0.0, 1.0], 1.5)
                .draw([0.0, 0.0, width, height], &c.draw_state, c.transform, g);

            // Draw control points.
            let original = Ellipse::new([1.0, 0.0, 0.0, 0.5]);
            let current = Ellipse::new([0.0, 0.0, 0.0, 0.5]);
            for i in 0..grid.ps.len() {
                use piston_window::ellipse::circle;

                // Original positions.
                let x = grid.ps[i][0];
                let y = grid.ps[i][1];
                original.draw(circle(x, y, 3.0), &c.draw_state, c.transform, g);

                // Current positions.
                let x = grid.qs[i][0];
                let y = grid.qs[i][1];
                current.draw(circle(x, y, 3.0), &c.draw_state, c.transform, g);
            };

        }); // end draw_2d
    }
}
