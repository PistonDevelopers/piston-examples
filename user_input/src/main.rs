#![feature(globs)]

extern crate graphics;
extern crate piston;
// extern crate sdl2_game_window;
extern crate glfw_game_window;

// use Window = sdl2_game_window::GameWindowSDL2;
use Window = glfw_game_window::GameWindowGLFW;
use piston::input::keyboard;
use piston::input;
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

    fn key_press<W: piston::GameWindow>(
        &mut self,
        window: &mut W,
        args: &input::KeyPressArgs
    ) {
        if args.key == keyboard::C {
            println!("Turned capture cursor on");
            self.capture_cursor = !self.capture_cursor;

            window.capture_cursor(self.capture_cursor);
        }

        println!("Pressed keyboard key '{}'", args.key);
    }

    fn key_release(
        &mut self,
        args: &input::KeyReleaseArgs
    ) {
        println!("Released keyboard key '{}'", args.key);
    }

    fn mouse_move(
        &mut self,
        args: &input::MouseMoveArgs
    ) {
        println!("Mouse moved '{} {}'", args.x, args.y);
    }

    fn mouse_relative_move(
        &mut self,
        args: &input::MouseRelativeMoveArgs
    ) {
        println!("Relative mouse moved '{} {}'", args.dx, args.dy);
    }

    fn mouse_press(
        &mut self,
        args: &input::MousePressArgs
    ) {
        println!("Pressed mouse button '{}'", args.button);
    }

    fn mouse_release(
        &mut self,
        args: &input::MouseReleaseArgs
    ) {
        println!("Released mouse button '{}'", args.button);
    }

    fn mouse_scroll(
        &mut self,
        args: &input::MouseScrollArgs
    ) {
        println!("Scrolled mouse '{}, {}'", args.x, args.y);
    }
}

fn main() {
    let mut window = Window::new(
        piston::shader_version::opengl::OpenGL_3_2,
        piston::GameWindowSettings {
            title: "Keycode".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    println!("Press C to turn capture cursor on/off");

    let mut app = App::new();
    let game_iter_settings = piston::GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    for e in piston::GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            Input(input::KeyPress(args)) => app.key_press(&mut window, &args),
            Input(input::KeyRelease(args)) => app.key_release(&args),
            Input(input::MousePress(args)) => app.mouse_press(&args),
            Input(input::MouseRelease(args)) => app.mouse_release(&args),
            Input(input::MouseMove(args)) => app.mouse_move(&args),
            Input(input::MouseScroll(args)) => app.mouse_scroll(&args),
            Input(input::MouseRelativeMove(args)) => app.mouse_relative_move(&args),
            Render(_) => {},
            Update(_) => {},
        }
    }
}

