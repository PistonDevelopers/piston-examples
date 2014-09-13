#![feature(globs)]

extern crate graphics;
extern crate piston;
// extern crate sdl2_game_window;
extern crate glfw_game_window;

// use sdl2_game_window::WindowSDL2;
use glfw_game_window::WindowGLFW;
use piston::input::keyboard;
use piston::input::{
    Focus,
    Keyboard,
    Mouse,
    MouseCursor,
    MouseRelative,
    MouseScroll,
    Move,
    Press,
    Release,
    Resize,
    Text,
};
use piston::{
    Input,
    Render,
    Update
};

pub struct App {
    capture_cursor: bool
}

impl App {
    /// Creates a new application.
    pub fn new() -> App {
        App { 
            capture_cursor: false 
        }
    }

    fn key_press<W: piston::Window>(
        &mut self,
        window: &mut W,
        key: keyboard::Key
    ) {
        if key == keyboard::C {
            println!("Turned capture cursor on");
            self.capture_cursor = !self.capture_cursor;

            window.capture_cursor(self.capture_cursor);
        }

        println!("Pressed keyboard key '{}'", key);
    }
}

fn main() {
    let mut window = WindowGLFW::new(
        piston::shader_version::opengl::OpenGL_3_2,
        piston::WindowSettings {
            title: "Keycode".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    println!("Press C to turn capture cursor on/off");

    let mut app = App::new();
    let event_settings = piston::EventSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    let mut event_iter = piston::EventIterator::new(&mut window, &event_settings);
    loop {
        let e = match event_iter.next() {
                None => { break; }
                Some(e) => e
            };

        match e {
            Input(Press(Keyboard(key))) => 
                app.key_press(event_iter.window, key),
            Input(Release(Keyboard(key))) => 
                println!("Released keyboard key '{}'", key),
            Input(Press(Mouse(button))) => 
                println!("Pressed mouse button '{}'", button),
            Input(Release(Mouse(button))) => 
                println!("Released mouse button '{}'", button),
            Input(Move(MouseCursor(x, y))) => 
                println!("Mouse moved '{} {}'", x, y),
            Input(Move(MouseScroll(dx, dy))) => 
                println!("Scrolled mouse '{}, {}'", dx, dy),
            Input(Move(MouseRelative(dx, dy))) => 
                println!("Relative mouse moved '{} {}'", dx, dy),
            Input(Text(text)) => println!("Typed '{}'", text),
            Input(Resize(w, h)) => println!("Resized '{}, {}'", w, h),
            Input(Focus(true)) => println!("Gained focus"),
            Input(Focus(false)) => println!("Lost focus"),
            Render(_) => {},
            Update(_) => {},
        }
    }
}

