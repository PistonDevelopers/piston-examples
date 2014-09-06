
#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use opengl_graphics::{
    Gl,
    Texture,
};
use sdl2_game_window::WindowSDL2;
use graphics::*;
use piston::{
    AssetStore,
    EventIterator,
    EventSettings,
    WindowSettings,
    Render,
};

fn main() {
    let opengl = piston::shader_version::opengl::OpenGL_3_2;
    let mut window = WindowSDL2::new(
        opengl,
        WindowSettings {
            title: "Image".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let asset_store = AssetStore::from_folder("../bin/assets");

    let image = asset_store.path("rust-logo.png").unwrap();
    let image = Texture::from_path(&image).unwrap();
    let event_settings = EventSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    let ref mut gl = Gl::new(opengl);
    for e in EventIterator::new(&mut window, &event_settings) {
        match e {
            Render(args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);

                let c = Context::abs(args.width as f64, args.height as f64);
                c.rgb(1.0, 1.0, 1.0).draw(gl);
                    c.image(&image).draw(gl);
            },
            _ => {},
        }
    }
}


