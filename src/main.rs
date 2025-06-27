use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use invaders::{
    frame::{self, Drawable, new_frame},
    player::Player,
    render::render,
};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();

    audio.add("explosion", "sounds/explosion.wav");
    audio.add("invader_killed", "sounds/invader_killed.wav");
    audio.add("invader", "sounds/invader.wav");
    audio.add("shoot", "sounds/shoot.wav");

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();

        render(&mut stdout, &last_frame, &last_frame, true);

        while let Ok(curr_frame) = render_rx.recv() {
            render(&mut stdout, &last_frame, &curr_frame, false);

            last_frame = curr_frame;
        }
    });

    let mut player = Player::new();
    let mut instant = Instant::now();

    'gameloop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("shoot");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("explosion");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        player.update(delta);

        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(cursor::Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
