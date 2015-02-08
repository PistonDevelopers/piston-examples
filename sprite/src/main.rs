#![feature(core, path)]

extern crate piston;
extern crate ai_behavior;
extern crate sprite;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::cell::RefCell;
use std::rc::Rc;

use sprite::*;
use ai_behavior::{
    Action,
    Sequence,
    Wait,
    WaitForever,
    While,
};

use sdl2_window::Sdl2Window;
use opengl_graphics::{
    Gl,
    OpenGL,
    Texture,
};

fn main() {
    let (width, height) = (300, 300);
    let opengl = OpenGL::_3_2;
    let window = Sdl2Window::new(
        opengl,
        piston::window::WindowSettings {
            title: "Sprite".to_string(),
            size: [width, height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let mut scene = Scene::new();
    let tex = Path::new("./rust-logo.png");
    let tex = Rc::new(Texture::from_path(&tex).unwrap());
    let mut sprite = Sprite::from_texture(tex.clone());
    sprite.set_position(width as f64 / 2.0, height as f64 / 2.0);

    let id = scene.add_child(sprite);

    // Run a sequence or animations.
    let seq = Sequence(vec![
        Action(Ease(EaseFunction::CubicOut, Box::new(ScaleTo(2.0, 0.5, 0.5)))),
        Action(Ease(EaseFunction::BounceOut, Box::new(MoveBy(1.0, 0.0, 100.0)))),
        Action(Ease(EaseFunction::ElasticOut, Box::new(MoveBy(2.0, 0.0, -100.0)))),
        Action(Ease(EaseFunction::BackInOut, Box::new(MoveBy(1.0, 0.0, -100.0)))),
        Wait(0.5),
        Action(Ease(EaseFunction::ExponentialInOut, Box::new(MoveBy(1.0, 0.0, 100.0)))),
        Action(Blink(1.0, 5)),
        While(Box::new(WaitForever), vec![
            Action(Ease(EaseFunction::QuadraticIn, Box::new(FadeOut(1.0)))),
            Action(Ease(EaseFunction::QuadraticOut, Box::new(FadeIn(1.0)))),
        ]),
    ]);
    scene.run(id.clone(), &seq);

    // This animation and the one above can run in parallel.
    let rotate = Action(Ease(EaseFunction::ExponentialInOut, 
        Box::new(RotateTo(2.0, 360.0))));
    scene.run(id.clone(), &rotate);

    println!("Press any key to pause/resume the animation!");

    let ref mut gl = Gl::new(opengl);
    let window = RefCell::new(window);
    for e in piston::events(&window) {
        use piston::event::{ PressEvent, RenderEvent };

        scene.event(&e);

        if let Some(args) = e.render_args() {
            use graphics::*;
            gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                graphics::clear([1.0, 1.0, 1.0, 1.0], gl);
                scene.draw(&c, gl);
            });
        }
        if let Some(_) = e.press_args() {
            scene.toggle(id.clone(), &seq);
            scene.toggle(id.clone(), &rotate);
        }
    }
}
