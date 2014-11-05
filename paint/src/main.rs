
#![feature(globs)]

extern crate shader_version;
extern crate input;
extern crate event;
extern crate image;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::cell::RefCell;
use opengl_graphics::{ Gl,Texture };
use sdl2_window::Sdl2Window;
use event::{
    Events,
    EventSettings,
    WindowSettings,
};
use image::GenericImage;

fn main() {
    let opengl = shader_version::opengl::OpenGL_3_2;
    let (width, height) = (300, 300);
    let window = Sdl2Window::new(
        opengl,
        WindowSettings {
            title: "Paint".to_string(),
            size: [width, height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let mut image = image::ImageBuf::new(width, height);
    let mut draw = false;
    let mut texture = Texture::from_image(&image);
    let event_settings = EventSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    let ref mut gl = Gl::new(opengl);
    let window = RefCell::new(window);
    for e in Events::new(&window, &event_settings) {
        use event::{ MouseCursorEvent, PressEvent, ReleaseEvent, RenderEvent };
        e.render(|args| {
            use graphics::*;

            gl.viewport(0, 0, args.width as i32, args.height as i32);

            let c = Context::abs(args.width as f64, args.height as f64);
            c.rgb(1.0, 1.0, 1.0).draw(gl);
            c.image(&texture).draw(gl);
        });
        e.press(|button| {
            if button == input::Mouse(input::mouse::Left) {
                draw = true
            }
        });
        e.release(|button| {
            if button == input::Mouse(input::mouse::Left) {
                draw = false
            }
        });
        if draw {
            e.mouse_cursor(|x, y| {
                let (x, y) = (x as u32, y as u32);
                if x < width && y < height {
                    image.put_pixel(x, y, image::Rgba(0, 0, 0, 255));
                    texture.update(&image);
                }
            });
        }
    }
}

