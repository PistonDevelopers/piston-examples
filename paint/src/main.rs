extern crate piston;
extern crate image;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use opengl_graphics::{ GlGraphics, OpenGL, Texture };
use sdl2_window::Sdl2Window;
use image::GenericImage;
use piston::input::{ Button, MouseButton };
use piston::window::WindowSettings;
use piston::event::*;

fn main() {
    let opengl = OpenGL::_3_2;
    let (width, height) = (300, 300);
    let window = Sdl2Window::new(
        opengl,
        WindowSettings::new("piston-example-paint", (width, height))
        .exit_on_esc(true)
    );

    let mut image = image::ImageBuffer::new(width, height);
    let mut draw = false;
    let mut texture = Texture::from_image(&image);
    let ref mut gl = GlGraphics::new(opengl);
    for e in window.events() {
        use piston::event::*;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                graphics::clear([1.0; 4], gl);
                graphics::image(&texture, c.transform, gl);
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
            if let Some(pos) = e.mouse_cursor_args() {
                let (x, y) = (pos[0] as u32, pos[1] as u32);
                if x < width && y < height {
                    image.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
                    texture.update(&image);
                }
            };
        }
    }
}

