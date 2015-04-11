extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::path::Path;
use opengl_graphics::{
    GlGraphics,
    OpenGL,
    Texture,
};
use sdl2_window::Sdl2Window;
use piston::window::{ WindowSettings, Size };
use piston::event::*;

fn main() {
    let opengl = OpenGL::_3_2;
    let window = Sdl2Window::new(
        opengl,
        WindowSettings::new(
            "piston-example-image".to_string(),
            Size { width: 300, height: 300 })
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
