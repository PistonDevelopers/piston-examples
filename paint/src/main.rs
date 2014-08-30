
#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use opengl_graphics::{ Gl,Texture };
use sdl2_game_window::GameWindowSDL2;
use graphics::*;
use piston::{
    GameIterator,
    GameIteratorSettings,
    GameWindowSettings,
    Render,
    Input,
};
use piston::image;
use piston::image::GenericImage;
use piston::input;

fn main() {
    let (width, height) = (300, 300);
    let mut window = GameWindowSDL2::new(
        piston::shader_version::opengl::OpenGL_3_2,
        GameWindowSettings {
            title: "Paint".to_string(),
            size: [width, height],
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    let mut image = image::ImageBuf::new(width, height);
    let mut draw = false;
    let mut texture = Texture::from_image(&image);
    let game_iter_settings = GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    let ref mut gl = Gl::new();
    for e in GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            Render(args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);

                let c = Context::abs(args.width as f64, args.height as f64);
                c.rgb(1.0, 1.0, 1.0).draw(gl);
                c.image(&texture).draw(gl);
            },
            Input(input::Press(input::Mouse(input::mouse::Left))) => {
                draw = true
            }
            Input(input::Release(input::Mouse(input::mouse::Left))) => {
                draw = false
            }
            Input(input::Move(input::MouseCursor(x, y))) if draw => {
                image.put_pixel(x as u32, y as u32, image::Rgba(0, 0, 0, 255));
                texture.update(&image);
            }
            _ => {},
        }
    }
}

