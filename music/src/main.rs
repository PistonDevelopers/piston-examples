//! Playing a music file with SDL2 mixer.

extern crate sdl2_game_window;
extern crate sdl2_mixer;
extern crate piston;

use Window = sdl2_game_window::GameWindowSDL2;
use mix = sdl2_mixer;
use piston::{
    AssetStore,
    GameIterator,
    GameIteratorSettings,
    GameWindowSettings,
};

fn init_audio() {
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
    let mut window = Window::new(
        GameWindowSettings {
            title: "Music".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true
        }
    );

    init_audio();
 
    // Load music file. 
    let asset_store = AssetStore::from_folder("assets");
    let file = asset_store.path("piano.wav").unwrap();
    let music = mix::Music::from_file(&file).unwrap();
   
    // Loop once. 
    music.play(1).unwrap();

    let game_iter_settings = GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    for _e in GameIterator::new(&mut window, &game_iter_settings) {}
}

