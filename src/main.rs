use macroquad::{
    audio::{load_sound, play_sound, PlaySoundParams},
    window::next_frame,
};

#[macroquad::main("Safari sound issue")]
async fn main() {
    let sound = load_sound("assets/theme.ogg").await.unwrap();
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
