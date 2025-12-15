
use std::rc::Rc;

use piston_window::*;
use sprite::*;
use ai_behavior::{
    Action,
    Sequence,
    Wait,
    WaitForever,
    While,
};
use wgpu_graphics::{Texture, TextureSettings};

fn main() {
    let (width, height) = (300, 300);
    let mut window: PistonWindow =
        WindowSettings::new("piston: sprite", (width, height))
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let id;
    let mut scene = Scene::new();
    let mut texture_context = window.create_texture_context();
    let tex = Rc::new(Texture::from_path(
            &mut texture_context,
            assets.join("rust.png"),
            &TextureSettings::new()
        ).unwrap());
    let mut sprite = Sprite::from_texture(tex);
    sprite.set_position(width as f64 / 2.0, height as f64 / 2.0);

    id = scene.add_child(sprite);

    // Run a sequence of animations.
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
    scene.run(id, &seq);

    // This animation and the one above can run in parallel.
    let rotate = Action(Ease(EaseFunction::ExponentialInOut,
        Box::new(RotateTo(2.0, 360.0))));
    scene.run(id, &rotate);

    println!("Press any key to pause/resume the animation!");

    while let Some(e) = window.next() {
        scene.event(&e);

        window.draw_2d(&e, |c, g, _| {
            use graphics::*;

            clear([1.0, 1.0, 1.0, 1.0], g);
            scene.draw(c.transform, g);
        });
        if e.press_args().is_some() {
            scene.toggle(id, &seq);
            scene.toggle(id, &rotate);
        }
    }
}
