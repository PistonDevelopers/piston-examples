extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate glutin_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;

use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;
use piston::window::WindowSettings;
use piston_window::*;
use gfx_graphics::GlyphCache;
use glutin_window::{ GlutinWindow, OpenGL };

fn main() {
    let window = Rc::new(RefCell::new(
        GlutinWindow::new(
            OpenGL::_3_2,
            WindowSettings::new(
                "gfx_graphics: text_test",
                [300, 300]
            )
            .exit_on_esc(true)
        )
    ));

    let events = PistonWindow::new(window, empty_app());
    let ref font = Path::new("assets/FiraSans-Regular.ttf");
    let factory = events.factory.borrow().clone();
    let mut glyph_cache = GlyphCache::new(font, factory).unwrap();

    for e in events {
        e.draw_2d(|c, g| {
            use graphics::*;

            let transform = c.transform.trans(10.0, 100.0);

            clear([1.0; 4], g);
            text::Text::colored([0.0, 1.0, 0.0, 1.0], 32).draw(
                "Hello gfx_graphics!",
                &mut glyph_cache,
                &c.draw_state,
                transform, g
            );
        });
    }
}
