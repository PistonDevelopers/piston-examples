#![feature(globs)]

extern crate shader_version;
extern crate input;
extern crate event;
extern crate sdl2_game_window;
// extern crate glfw_game_window;

use sdl2_game_window::WindowSDL2;
// use glfw_game_window::WindowGLFW;
use input::{ keyboard, Keyboard, Mouse };
use event::{
    EventIterator,
    EventSettings,
    FocusEvent,
    PressEvent,
    MouseCursorEvent,
    MouseRelativeEvent,
    MouseScrollEvent,
    ReleaseEvent,
    RenderEvent,
    ResizeEvent,
    TextEvent,
    UpdateEvent,
    Window,
    WindowSettings,
};

fn main() {
    let mut window = WindowSDL2::new(
        shader_version::opengl::OpenGL_3_2,
        WindowSettings {
            title: "Keycode".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    println!("Press C to turn capture cursor on/off");

    let mut capture_cursor = false;
    let event_settings = EventSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    let mut event_iter = EventIterator::new(&mut window, &event_settings);
    loop {
        let e = match event_iter.next() {
                None => { break; }
                Some(e) => e
            };

        e.press(|button| {
            match button {
                Keyboard(key) => {
                    if key == keyboard::C {
                        println!("Turned capture cursor on");
                        capture_cursor = !capture_cursor;
                        event_iter.window.capture_cursor(capture_cursor);
                    }

                    println!("Pressed keyboard key '{}'", key);
                }, 
                Mouse(button) => println!("Pressed mouse button '{}'", button),
            }
        });
        e.release(|button| {
            match button {
                Keyboard(key) => println!("Released keyboard key '{}'", key),
                Mouse(button) => println!("Released mouse button '{}'", button),
            }
        });
        e.mouse_cursor(|x, y| println!("Mouse moved '{} {}'", x, y));
        e.mouse_scroll(|dx, dy| println!("Scrolled mouse '{}, {}'", dx, dy));
        e.mouse_relative(|dx, dy| println!("Relative mouse moved '{} {}'", dx, dy));
        e.text(|text| println!("Typed '{}'", text));
        e.resize(|w, h| println!("Resized '{}, {}'", w, h));
        e.focus(|focused| {
            if focused { println!("Gained focus"); }
            else { println!("Lost focus"); }
        });
        e.render(|_| {});
        e.update(|_| {});
    }
}

