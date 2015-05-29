extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate glutin_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;

use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;
use std::ops::DerefMut;
use piston::window::WindowSettings;
use piston_window::*;
use gfx_graphics::{ Texture, TextureSettings };
use glutin_window::{ GlutinWindow, OpenGL };

fn main() {
    let window = Rc::new(RefCell::new(
        GlutinWindow::new(
            OpenGL::_3_2,
            WindowSettings::new(
                "gfx_graphics: image_test",
                [300, 300]
            )
            .exit_on_esc(true)
        )
    ));

    let events = PistonWindow::new(window, empty_app());
    let rust_logo = Texture::from_path(events.factory.borrow_mut().deref_mut(),
                                       &Path::new("./assets/rust.png"),
                                       &TextureSettings::new()).unwrap();
    for e in events {
        use graphics::*;

        e.draw_2d(|c, g| {
            let transform = c.transform.trans(100.0, 100.0);

            clear([1.0; 4], g);
            Rectangle::new([1.0, 0.0, 0.0, 1.0])
                .draw([0.0, 0.0, 100.0, 100.0],
                      &c.draw_state,
                      c.transform,
                      g);
            Rectangle::new([0.0, 1.0, 0.0, 0.3])
                .draw([50.0, 50.0, 100.0, 100.0],
                      &c.draw_state,
                      c.transform,
                      g);
            image(&rust_logo, transform, g);
        });
    }
}
