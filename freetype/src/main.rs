extern crate graphics;
extern crate freetype as ft;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate piston;
extern crate find_folder;

use sdl2_window::Sdl2Window;
use opengl_graphics::{ GlGraphics, Texture, TextureSettings, OpenGL };
use piston::window::WindowSettings;
use piston::input::*;
use piston::event_loop::{Events, EventSettings, EventLoop};
use graphics::{Context, Graphics, ImageSize};

fn glyphs(face: &mut ft::Face, text: &str) -> Vec<(Texture, [f64; 2])> {
    let mut x = 10;
    let mut y = 0;
    let mut res = vec![];
    for ch in text.chars() {
        face.load_char(ch as usize, ft::face::LoadFlag::RENDER).unwrap();
        let g = face.glyph();

        let bitmap = g.bitmap();
        let texture = Texture::from_memory_alpha(
            bitmap.buffer(),
            bitmap.width() as u32,
            bitmap.rows() as u32,
            &TextureSettings::new()
        ).unwrap();
        res.push((texture, [(x + g.bitmap_left()) as f64, (y - g.bitmap_top()) as f64]));

        x += (g.advance().x >> 6) as i32;
        y += (g.advance().y >> 6) as i32;
    }
    res
}

fn render_text<G, T>(glyphs: &[(T, [f64; 2])], c: &Context, gl: &mut G)
    where G: Graphics<Texture = T>, T: ImageSize
{
    for &(ref texture, [x, y]) in glyphs {
        use graphics::*;

        Image::new_color(color::BLACK).draw(
            texture,
            &c.draw_state,
            c.transform.trans(x, y),
            gl
        );
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Sdl2Window =
        WindowSettings::new("piston-example-freetype", [300, 300])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let freetype = ft::Library::init().unwrap();
    let font = assets.join("FiraSans-Regular.ttf");
    let mut face = freetype.new_face(&font, 0).unwrap();
    face.set_pixel_sizes(0, 48).unwrap();

    let ref mut gl = GlGraphics::new(opengl);
    let glyphs = glyphs(&mut face, "Hello Piston!");

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            use graphics::*;

            gl.draw(args.viewport(), |c, gl| {
                clear(color::WHITE, gl);
                render_text(&glyphs, &c.trans(0.0, 100.0), gl);
            });
        }
    }
}
