//! Playing a music file with SDL2 mixer.

extern crate sdl2_game_window;
extern crate sdl2_mixer;
extern crate sdl2;
extern crate piston;

use sdl2_game_window::WindowSDL2;
use sdl2_mixer as mix;
use piston::{
    AssetStore,
    EventIterator,
    EventSettings,
    WindowSettings,
};

fn init_audio() {
    sdl2::init(sdl2::InitAudio | sdl2::InitTimer);
    // Load dynamic libraries.
    mix::init(
          mix::InitMp3 
        | mix::InitFlac 
        | mix::InitMod 
        | mix::InitFluidSynth
        | mix::InitModPlug
        | mix::InitOgg
    );
    mix::open_audio(
        mix::DEFAULT_FREQUENCY,
        mix::DEFAULT_FORMAT,
        mix::DEFAULT_CHANNELS,
        1024
    ).unwrap();
    mix::allocate_channels(mix::DEFAULT_CHANNELS); 
}

fn main() {
    let mut window = WindowSDL2::new(
        piston::shader_version::opengl::OpenGL_3_2,
        WindowSettings {
            title: "Music".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    init_audio();
 
    // Load music file. 
    let asset_store = AssetStore::from_folder("../bin/assets");
    let file = asset_store.path("piano.wav").unwrap();
    let music = mix::Music::from_file(&file).unwrap();
   
    // Loop once. 
    music.play(4).unwrap();

    let event_settings = EventSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    for _e in EventIterator::new(&mut window, &event_settings) {}
}

