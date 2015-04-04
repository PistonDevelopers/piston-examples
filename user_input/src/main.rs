extern crate piston;
extern crate opengl_graphics;
extern crate graphics;
#[cfg(feature = "include_sdl2")]
extern crate sdl2_window;
#[cfg(feature = "include_glfw")]
extern crate glfw_window;
#[cfg(feature = "include_glutin")]
extern crate glutin_window;

use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::Graphics;
use std::rc::Rc;
use std::cell::RefCell;
use piston::window::{ AdvancedWindow, WindowSettings, Size };
use piston::input::Button;
use piston::input::keyboard::Key;
#[cfg(feature = "include_sdl2")]
use sdl2_window::Sdl2Window as Window;
#[cfg(feature = "include_glfw")]
use glfw_window::GlfwWindow as Window;
#[cfg(feature = "include_glutin")]
use glutin_window::GlutinWindow as Window;

fn main() {
    let opengl = OpenGL::_3_2;
    let window = Window::new(
        opengl,
        WindowSettings::new(
            "piston-example-user_input".to_string(),
            Size { width: 600, height: 600 }
        )
        .exit_on_esc(true)
    );

    println!("Press C to turn capture cursor on/off");

    let mut capture_cursor = false;
    let window = Rc::new(RefCell::new(window));
    let ref mut gl = GlGraphics::new(opengl);
    let mut cursor = [0.0, 0.0];
    for e in piston::events(window.clone()) {
        use piston::event::*;

        if let Some(Button::Mouse(button)) = e.press_args() {
            println!("Pressed mouse button '{:?}'", button);
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::C {
                println!("Turned capture cursor on");
                capture_cursor = !capture_cursor;
                window.borrow_mut().set_capture_cursor(capture_cursor);
            }

            println!("Pressed keyboard key '{:?}'", key);
        };
        if let Some(button) = e.release_args() {
            match button {
                Button::Keyboard(key) => println!("Released keyboard key '{:?}'", key),
                Button::Mouse(button) => println!("Released mouse button '{:?}'", button),
            }
        };
        e.mouse_cursor(|x, y| {
            cursor = [x, y];
            println!("Mouse moved '{} {}'", x, y);
        });
        e.mouse_scroll(|dx, dy| println!("Scrolled mouse '{}, {}'", dx, dy));
        e.mouse_relative(|dx, dy| println!("Relative mouse moved '{} {}'", dx, dy));
        e.text(|text| println!("Typed '{}'", text));
        e.resize(|w, h| println!("Resized '{}, {}'", w, h));
        if let Some(focused) = e.focus_args() {
            if focused { println!("Gained focus"); }
            else { println!("Lost focus"); }
        };
        if let Some(args) = e.render_args() {
            gl.draw(
                [0, 0, args.width as i32, args.height as i32],
                |_, g| {
                    graphics::clear([1.0; 4], g);
                    draw_rectangles(cursor, &window.borrow(), g);
                }
            );
        }
        e.update(|_| {});
    }
}

fn draw_rectangles<G: Graphics>(
    cursor: [f64; 2],
    window: &Window,
    g: &mut G,
) {
    use piston::window::Window;

    let size = window.size();
    let draw_size = window.draw_size();
    let zoom = 0.2;
    let offset = 30.0;

    let draw_state = graphics::default_draw_state();
    let transform = graphics::abs_transform(size.width as f64, size.height as f64);
    let rect_border = graphics::Rectangle::new_border([1.0, 0.0, 0.0, 1.0], 1.0);

    // Cursor.
    let cursor_color = [0.0, 0.0, 0.0, 1.0];
    let zoomed_cursor = [offset + cursor[0] * zoom, offset + cursor[1] * zoom];
    graphics::ellipse(
        cursor_color,
        graphics::ellipse::circle(zoomed_cursor[0], zoomed_cursor[1], 4.0),
        transform,
        g
    );

    // User coordinates.
    rect_border.draw([
            offset,
            offset,
            size.width as f64 * zoom,
            size.height as f64 * zoom
        ],
        draw_state, transform, g);
    let rect_border = graphics::Rectangle::new_border([0.0, 0.0, 1.0, 1.0], 1.0);
    rect_border.draw(
        [
            offset + size.width as f64 * zoom,
            offset,
            draw_size.width as f64 * zoom,
            draw_size.height as f64 * zoom
        ],
        draw_state, transform, g);
}
