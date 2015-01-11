
extern crate piston;
extern crate opengl_graphics;

fn main() {
    println!("Click in the red square and drag.");
    println!("Toggle grid with G.");
    println!("Reset grid with R.");

    piston::start(
        piston::shader_version::OpenGL::_3_2,
        piston::WindowSettings {
            title: "Deform".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4,
        },
        || start()
    );
}

pub fn start() {
    use piston::graphics;
    use piston::graphics::ImageSize;
    use piston::drag_controller::{
        DragController,
        Drag
    };
    use opengl_graphics::Texture;

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

    for e in piston::events() {
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
        e.press(|button| {
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
        });
        e.render(|_args| {
            piston::render_2d_opengl(
                Some(graphics::color::WHITE),
                |c, g| {

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
            for i in range(0, grid.ps.len()) {
                use piston::graphics::ellipse::circle;

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
        }); // end render
    }
}

