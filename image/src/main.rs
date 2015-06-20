extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate find_folder;

use opengl_graphics::{
    GlGraphics,
    OpenGL,
    Texture,
};
use sdl2_window::Sdl2Window;
use piston::window::WindowSettings;
use piston::event::*;

fn main() {
    let opengl = OpenGL::_3_2;
    let window: Sdl2Window =
        WindowSettings::new("piston-example-image", [300, 300])
        .exit_on_esc(true)
        .opengl(opengl)
        .into();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let rust_logo = assets.join("rust.png");
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
