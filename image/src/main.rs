#![feature(old_path)]

extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::cell::RefCell;
use opengl_graphics::{
    Gl,
    OpenGL,
    Texture,
};
use sdl2_window::Sdl2Window;

fn main() {
    let opengl = OpenGL::_3_2;
    let window = Sdl2Window::new(
        opengl,
        piston::window::WindowSettings {
            title: "Image".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let image = Path::new("./bin/assets/rust-logo.png");
    let image = Texture::from_path(&image).unwrap();
    let ref mut gl = Gl::new(opengl);
    let window = RefCell::new(window);
    for e in piston::events(&window) {
        use piston::event::RenderEvent;

        if let Some(args) = e.render_args() {
            use graphics::*;
            gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                graphics::clear([1.0; 4], gl);
                graphics::image(&image, &c, gl);
            });
        };
    }
}


