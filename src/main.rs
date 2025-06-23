use std::error::Error;

use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();

    audio.add("explosion", "sounds/explosion.wav");
    audio.add("invader_killed", "sounds/invader_killed.wav");
    audio.add("invader", "sounds/invader.wav");
    audio.add("shoot", "sounds/shoot.wav");
    audio.add("song", "sounds/song.mpeg");

    audio.play("song");

    audio.wait();

    Ok(())
}
