#![feature(core)]

extern crate piston;
extern crate image;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::cell::RefCell;
use opengl_graphics::{ Gl, OpenGL, Texture };
use sdl2_window::Sdl2Window;
use image::GenericImage;
use piston::input::{ Button, MouseButton };

fn main() {
    let opengl = OpenGL::_3_2;
    let (width, height) = (300, 300);
    let window = Sdl2Window::new(
        opengl,
        piston::window::WindowSettings {
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
    for e in piston::events(&window) {
        use piston::event::{ MouseCursorEvent, PressEvent, ReleaseEvent, RenderEvent };
        
        if let Some(args) = e.render_args() {
            gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                graphics::clear([1.0; 4], gl);
                graphics::image(&texture, &c, gl);
            });
        };
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = true
            }
        };
        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = false
            }
        };
        if draw {
            if let Some([x, y]) = e.mouse_cursor_args() {
                let (x, y) = (x as u32, y as u32);
                if x < width && y < height {
                    image.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
                    texture.update(&image);
                }
            };
        }
    }
}

