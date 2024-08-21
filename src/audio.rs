use rusty_audio::Audio;

pub fn setup_audio() -> Audio {
    let mut audio = Audio::new();
    let sounds = ["explode", "lose", "move", "pew", "startup", "win"];
    for sound in sounds {
        audio.add(sound, format!("sounds/{}.wav", sound));
    }
    audio
}