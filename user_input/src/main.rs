extern crate piston;
extern crate opengl_graphics;
extern crate graphics;
extern crate touch_visualizer;

#[cfg(feature = "include_sdl2")]
extern crate sdl2_window;
#[cfg(feature = "include_glfw")]
extern crate glfw_window;
#[cfg(feature = "include_glutin")]
extern crate glutin_window;

use touch_visualizer::TouchVisualizer;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::{ Context, Graphics };
use std::collections::HashMap;
use piston::window::{ AdvancedWindow, Window, WindowSettings };
use piston::input::*;
use piston::event_loop::*;
#[cfg(feature = "include_sdl2")]
use sdl2_window::Sdl2Window as AppWindow;
#[cfg(feature = "include_glfw")]
use glfw_window::GlfwWindow as AppWindow;
#[cfg(feature = "include_glutin")]
use glutin_window::GlutinWindow as AppWindow;

type AxisValues = HashMap<(i32, u8), f64>;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: AppWindow = WindowSettings::new("piston-example-user_input", [600, 600])
        .exit_on_esc(true).graphics_api(opengl).build().unwrap();

    println!("Press C to turn capture cursor on/off");

    let mut capture_cursor = false;
    let ref mut gl = GlGraphics::new(opengl);
    let mut cursor = [0.0, 0.0];

    let mut touch_visualizer = TouchVisualizer::new();
    let mut axis_values: AxisValues = HashMap::new();

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        touch_visualizer.event(window.size(), &e);
        if let Some(Button::Mouse(button)) = e.press_args() {
            println!("Pressed mouse button '{:?}'", button);
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::C {
                println!("Turned capture cursor on");
                capture_cursor = !capture_cursor;
                window.set_capture_cursor(capture_cursor);
            }

            println!("Pressed keyboard key '{:?}'", key);
        };
        if let Some(args) = e.button_args() {
            println!("Scancode {:?}", args.scancode);
        }
        if let Some(button) = e.release_args() {
            match button {
                Button::Keyboard(key) => println!("Released keyboard key '{:?}'", key),
                Button::Mouse(button) => println!("Released mouse button '{:?}'", button),
                Button::Controller(button) => println!("Released controller button '{:?}'", button),
                Button::Hat(hat) => println!("Released controller hat `{:?}`", hat),
            }
        };
        if let Some(args) = e.controller_axis_args() {
            axis_values.insert((args.id, args.axis), args.position);
        }
        e.mouse_cursor(|x, y| {
            cursor = [x, y];
            println!("Mouse moved '{} {}'", x, y);
        });
        e.mouse_scroll(|dx, dy| println!("Scrolled mouse '{}, {}'", dx, dy));
        e.mouse_relative(|dx, dy| println!("Relative mouse moved '{} {}'", dx, dy));
        e.text(|text| println!("Typed '{}'", text));
        e.resize(|w, h| println!("Resized '{}, {}'", w, h));
        if let Some(cursor) = e.cursor_args() {
            if cursor { println!("Mouse entered"); }
            else { println!("Mouse left"); }
        };
        if let Some(args) = e.render_args() {
            // println!("Render {}", args.ext_dt);
            gl.draw(args.viewport(), |c, g| {
                    graphics::clear([1.0; 4], g);
                    draw_rectangles(cursor, &window, &c, g);
                    draw_axis_values(&mut axis_values, &window, &c, g);
                    touch_visualizer.draw(&c, g);
                }
            );
        }
        if let Some(_args) = e.idle_args() {
            // println!("Idle {}", _args.dt);
        }
        if let Some(_args) = e.update_args() {
            /*
            // Used to test CPU overload.
            println!("Update {}", _args.dt);
            let mut x: f64 = 0.0;
            for _ in 0..500_000 {
                x += (1.0 + x).sqrt();
            }
            println!("{}", x);
            */
        }
    }
}

fn draw_rectangles<G: Graphics>(
    cursor: [f64; 2],
    window: &Window,
    c: &Context,
    g: &mut G,
) {
    let size = window.size();
    let draw_size = window.draw_size();
    let zoom = 0.2;
    let offset = 30.0;

    let rect_border = graphics::Rectangle::new_border([1.0, 0.0, 0.0, 1.0], 1.0);

    // Cursor.
    let cursor_color = [0.0, 0.0, 0.0, 1.0];
    let zoomed_cursor = [offset + cursor[0] * zoom, offset + cursor[1] * zoom];
    graphics::ellipse(
        cursor_color,
        graphics::ellipse::circle(zoomed_cursor[0], zoomed_cursor[1], 4.0),
        c.transform,
        g
    );

    // User coordinates.
    rect_border.draw([
            offset,
            offset,
            size.width as f64 * zoom,
            size.height as f64 * zoom
        ],
        &c.draw_state, c.transform, g);
    let rect_border = graphics::Rectangle::new_border([0.0, 0.0, 1.0, 1.0], 1.0);
    rect_border.draw(
        [
            offset + size.width as f64 * zoom,
            offset,
            draw_size.width as f64 * zoom,
            draw_size.height as f64 * zoom
        ],
        &c.draw_state, c.transform, g);
}

fn draw_axis_values<W: Window, G: Graphics>(
    axis_values: &mut AxisValues,
    window: &W,
    c: &Context,
    g: &mut G
) {
    let window_height = window.size().height as f64;
    let max_axis_height = 200.0;
    let offset = 10.0;
    let top = window_height - (max_axis_height + offset);
    let color = [1.0, 0.0, 0.0, 1.0];
    let width = 10.0;
    let mut draw = |i, v: f64| {
        let i = i as f64;
        let height = (v + 1.0) / 2.0 * max_axis_height;
        let rect = [offset + i * (width + offset),
            top + max_axis_height - height, width, height];
        graphics::rectangle(color, rect, c.transform, g);
    };
    for (i, &v) in axis_values.values().enumerate() {
        draw(i, v);
    }
}
