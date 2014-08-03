#![feature(globs)]

extern crate graphics;
extern crate piston;
// extern crate sdl2_game_window;
extern crate glfw_game_window;

// use Window = sdl2_game_window::GameWindowSDL2;
use Window = glfw_game_window::GameWindowGLFW;
use piston::{
    keyboard,
    Game,
    GameIteratorSettings,
    GameWindow,
    GameWindowSettings,
    KeyPressArgs,
    KeyReleaseArgs,
    MouseMoveArgs,
    MouseRelativeMoveArgs,
    MousePressArgs,
    MouseReleaseArgs,
    MouseScrollArgs,
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
}

impl<W: GameWindow> Game<W> for App {
    fn key_press(
        &mut self,
        window: &mut W,
        args: &KeyPressArgs
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
        _window: &mut W,
        args: &KeyReleaseArgs
    ) {
        println!("Released keyboard key '{}'", args.key);
    }

    fn mouse_move(
        &mut self,
        _window: &mut W,
        args: &MouseMoveArgs
    ) {
        println!("Mouse moved '{} {}'", args.x, args.y);
    }

    fn mouse_relative_move(
        &mut self,
        _window: &mut W,
        args: &MouseRelativeMoveArgs
    ) {
        println!("Relative mouse moved '{} {}'", args.dx, args.dy);
    }

    fn mouse_press(
        &mut self,
        _window: &mut W,
        args: &MousePressArgs
    ) {
        println!("Pressed mouse button '{}'", args.button);
    }

    fn mouse_release(
        &mut self,
        _window: &mut W,
        args: &MouseReleaseArgs
    ) {
        println!("Released mouse button '{}'", args.button);
    }

    fn mouse_scroll(
        &mut self,
        _window: &mut W,
        args: &MouseScrollArgs
    ) {
        println!("Scrolled mouse '{}, {}'", args.x, args.y);
    }
}

fn main() {
    let mut window = Window::new(
        GameWindowSettings {
            title: "Keycode".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    println!("Press C to turn capture cursor on/off");

    let mut app = App::new();
    let game_iter_settings = GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    app.run(&mut window, &game_iter_settings);
}

