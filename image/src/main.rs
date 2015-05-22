extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::path::Path;
use opengl_graphics::{
    GlGraphics,
    OpenGL,
    Texture,
};
use glutin_window::GlutinWindow;
use piston::window::WindowSettings;
use piston::event::*;

fn main() {
    let opengl = OpenGL::_3_2;
    let window = GlutinWindow::new(
        opengl,
        WindowSettings::new("piston-example-image", [300, 300])
        .exit_on_esc(true)
    );

    let rust_logo = Path::new("./bin/assets/rust-logo.png");
    let rust_logo = Texture::from_path(&rust_logo).unwrap();
    let ref mut gl = GlGraphics::new(opengl);
    for e in window.events() {
        if let Some(args) = e.render_args() {
            use graphics::*;

            gl.draw(args.viewport(), |c, gl| {
                clear([1.0; 4], gl);
                image(&rust_logo, c.transform, gl);
            });
        };
    }
}
