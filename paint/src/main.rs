
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
use event::{ Events, WindowSettings };
use image::GenericImage;
use input::{ Button, MouseButton };

fn main() {
    let opengl = shader_version::OpenGL::_3_2;
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

    let mut image = image::ImageBuffer::new(width, height);
    let mut draw = false;
    let mut texture = Texture::from_image(&image);
    let ref mut gl = Gl::new(opengl);
    let window = RefCell::new(window);
    for e in Events::new(&window) {
        use event::{ MouseCursorEvent, PressEvent, ReleaseEvent, RenderEvent };
        e.render(|args| {
            gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                graphics::clear([1.0, ..4], gl);
                graphics::image(&texture, &c, gl);
            });
        });
        e.press(|button| {
            if button == Button::Mouse(MouseButton::Left) {
                draw = true
            }
        });
        e.release(|button| {
            if button == Button::Mouse(MouseButton::Left) {
                draw = false
            }
        });
        if draw {
            e.mouse_cursor(|x, y| {
                let (x, y) = (x as u32, y as u32);
                if x < width && y < height {
                    image.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
                    texture.update(&image);
                }
            });
        }
    }
}

