use macroquad::{
    audio::{load_sound, play_sound, set_sound_volume, PlaySoundParams},
    window::next_frame,
};

#[macroquad::main("Ljud")]
async fn main() {
    let sound = load_sound("assets/theme.mp3").await.unwrap();
    play_sound(
        sound,
        PlaySoundParams {
            looped: true,
            volume: 1.,
        },
    );
    loop {
        next_frame().await;
    }
}