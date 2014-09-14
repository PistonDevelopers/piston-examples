
#![feature(globs)]

extern crate piston;
extern crate graphics;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use std::rc::Rc;

use piston::{
    AssetStore,
    EventIterator,
    EventSettings,
    WindowSettings,
    Render,
    Scene,
    Sprite,
};
use piston::action::*;
use piston::event::{
    Action,
    Sequence,
    Wait,
    WaitForever,
    While,
};

use graphics::*;

use sdl2_game_window::WindowSDL2;
use opengl_graphics::{
    Gl,
    Texture,
};

fn main() {
    let (width, height) = (300, 300);
    let opengl = piston::shader_version::opengl::OpenGL_3_2;
    let mut window = WindowSDL2::new(
        opengl,
        WindowSettings {
            title: "Sprite".to_string(),
            size: [width, height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let asset_store = AssetStore::from_folder("../");
    let mut scene = Scene::new();

    let tex = Rc::new(Texture::from_path(&asset_store.path("rust-logo.png").unwrap()).unwrap());
    let mut sprite = Sprite::from_texture(tex.clone());
    sprite.set_position(width as f64 / 2.0, height as f64 / 2.0);

    let id = scene.add_child(sprite);

    // Run a sequence actions
    scene.run_action(id, Sequence(vec![
        Action(EaseOut(box ScaleTo(2.0, 0.5, 0.5))),
        Action(EaseInOut(box MoveBy(0.5, 0.0, -100.0))),
        Wait(1.0),
        Action(Blink(1.0, 5)),
        While(box WaitForever, vec![
            Action(EaseIn(box FadeOut(1.0))),
            Action(EaseOut(box FadeIn(1.0))),
        ]),
    ]));

    // This action and above one can run in parallel
    scene.run_action(id, Action(RotateTo(2.0, 360.0)));

    let event_settings = EventSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };
    let ref mut gl = Gl::new(opengl);
    for e in EventIterator::new(&mut window, &event_settings) {
        scene.update(&e);

        match e {
            Render(args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);

                let c = Context::abs(args.width as f64, args.width as f64);
                c.rgb(1.0, 1.0, 1.0).draw(gl);

                scene.draw(&c, gl);
            },
            _ => {},
        }
    }
}
