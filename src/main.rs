use std::{ error::Error, io, sync::mpsc, thread, time::{ Duration, Instant } };

use crossterm::{
    cursor::{ Hide, Show },
    event::{ self, Event, KeyCode },
    terminal::{ self, EnterAlternateScreen, LeaveAlternateScreen },
    ExecutableCommand,
};
use invaders::{
    audio::setup_audio,
    frame::{ self, Drawable },
    invaders::Invaders,
    player::Player,
    render::render,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = setup_audio();

    audio.play("startup");

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let (render_sender, render_receiver) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut stdout = io::stdout();
        let mut prev_frame = frame::new_frame();
        render(&mut stdout, &prev_frame, &prev_frame, true);

        loop {
            let cur_frame = match render_receiver.recv() {
                Ok(frame) => frame,
                Err(_) => {
                    break;
                }
            };

            render(&mut stdout, &prev_frame, &cur_frame, false);
            prev_frame = cur_frame;
        }
    });

    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    'gameloop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();

        let mut cur_frame = frame::new_frame();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => {
                        player.move_left();
                    }
                    KeyCode::Right => {
                        player.move_right();
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }
        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut cur_frame);
        }

        let _ = render_sender.send(cur_frame);
        thread::sleep(Duration::from_millis(1));

        if invaders.is_all_killed() {
            audio.play("win");
            break 'gameloop;
        } else if invaders.has_reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }
    }

    drop(render_sender);
    render_handle.join().unwrap();

    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
