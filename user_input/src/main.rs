extern crate piston;
extern crate shader_version;
extern crate sdl2_window;

use std::cell::RefCell;
use piston::quack::Set;
use piston::window::{ WindowSettings, CaptureCursor };
use piston::input::Button;
use piston::input::keyboard::Key;
use piston::event::{
    PressEvent,
    ReleaseEvent,
    MouseCursorEvent,
    MouseScrollEvent,
    MouseRelativeEvent,
    TextEvent,
    ResizeEvent,
    FocusEvent,
    RenderEvent,
    UpdateEvent
};
use shader_version::OpenGL;
use sdl2_window::Sdl2Window as Window;

fn main() {
    let window = Window::new(
        OpenGL::_3_2,
        WindowSettings {
            title: "piston-examples/user_input".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    println!("Press C to turn capture cursor on/off");

    let mut capture_cursor = false;
    let ref window = RefCell::new(window);
    for e in piston::events(window) {
        if let Some(Button::Mouse(button)) = e.press_args() {
            println!("Pressed mouse button '{:?}'", button);
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::C {
                println!("Turned capture cursor on");
                capture_cursor = !capture_cursor;
                window.set(CaptureCursor(capture_cursor));
            }

            println!("Pressed keyboard key '{:?}'", key);
        };
        if let Some(button) = e.release_args() {
            match button {
                Button::Keyboard(key) => println!("Released keyboard key '{:?}'", key),
                Button::Mouse(button) => println!("Released mouse button '{:?}'", button),
            }
        };
        e.mouse_cursor(|x, y| println!("Mouse moved '{} {}'", x, y));
        e.mouse_scroll(|dx, dy| println!("Scrolled mouse '{}, {}'", dx, dy));
        e.mouse_relative(|dx, dy| println!("Relative mouse moved '{} {}'", dx, dy));
        e.text(|text| println!("Typed '{}'", text));
        e.resize(|w, h| println!("Resized '{}, {}'", w, h));
        if let Some(focused) = e.focus_args() {
            if focused { println!("Gained focus"); }
            else { println!("Lost focus"); }
        };
        e.render(|_| {});
        e.update(|_| {});
    }
}

