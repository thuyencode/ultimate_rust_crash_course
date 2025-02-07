use crossterm::{
  cursor::{Hide, Show},
  event::{self, Event, KeyCode},
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
  ExecutableCommand,
};
use invaders::{
  frame::{self, Drawable},
  invaders::Invaders,
  player::Player,
  render,
};
use rusty_audio::Audio;
use std::{
  env::current_dir,
  fs::{read_dir, ReadDir},
  io::{self, Error},
  path::PathBuf,
  sync::mpsc,
  thread,
  time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Get the current working directory
  let current_dir: Result<PathBuf, Error> = current_dir();

  if let Err(err) = current_dir {
    return Err(err.into());
  }

  // Get the list of files inside the "sounds" folder
  let audio_files: Result<ReadDir, Error> =
    read_dir(format!("{}/sounds", current_dir.unwrap().to_string_lossy()));

  if let Err(err) = audio_files {
    return Err(err.into());
  }

  // Add all the audio files into audio
  let mut audio: Audio = Audio::new();

  for file in audio_files.unwrap() {
    match file {
      Ok(entry) => {
        let file_name: String = entry.file_name().to_string_lossy().to_string();
        let file_path: String = entry.path().display().to_string();

        audio.add(file_name, file_path);
      }
      Err(e) => {
        return Err(e.into());
      }
    }
  }

  // Terminal
  let mut stdout = io::stdout();

  terminal::enable_raw_mode()?;
  stdout.execute(EnterAlternateScreen)?;
  stdout.execute(Hide)?;

  // Render loop in a separate thread
  let (render_tx, render_rx) = mpsc::channel();

  let render_handle = thread::spawn(move || {
    let mut last_frame: Vec<Vec<&str>> = frame::new_frame();
    let mut stdout = io::stdout();

    render::render(&mut stdout, &last_frame, &last_frame, true);

    while let Ok(current_frame) = render_rx.recv() {
      render::render(&mut stdout, &last_frame, &current_frame, false);
      last_frame = current_frame;
    }
  });

  // Game loop
  let mut player: Player = Player::new();
  let mut instant: Instant = Instant::now();
  let mut invaders: Invaders = Invaders::new();

  'gameloop: loop {
    // Per-frame init
    let mut current_frame: Vec<Vec<&str>> = frame::new_frame();
    let delta: Duration = instant.elapsed();

    instant = Instant::now();

    // Input
    while event::poll(Duration::default())? {
      if let Event::Key(key_event) = event::read()? {
        match key_event.code {
          KeyCode::Esc | KeyCode::Char('q') => {
            audio.play("lose.wav");
            break 'gameloop;
          }
          KeyCode::Left => player.move_left(),
          KeyCode::Right => player.move_right(),
          KeyCode::Enter | KeyCode::Char(' ') => {
            if player.shoot() {
              audio.play("pew.wav");
            }
          }
          _ => {}
        }
      }
    }

    // Updates
    player.update(delta);

    if invaders.update(delta) {
      audio.play("move.wav");
    }

    if player.detect_hits(&mut invaders) {
      audio.play("explode.wav");
    }

    // Draw and render
    let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];

    drawables
      .iter()
      .for_each(|drawable: &&dyn Drawable| drawable.draw(&mut current_frame));

    let _ = render_tx.send(current_frame);

    thread::sleep(Duration::from_millis(5));

    // Win or lose
    if invaders.all_killed() {
      audio.play("win.wav");
      break 'gameloop;
    }

    if invaders.reached_bottom() {
      audio.play("lose.wav");
      break 'gameloop;
    }
  }
  // Cleanup
  drop(render_tx);
  render_handle.join().unwrap();

  audio.wait();
  stdout.execute(Show)?;
  stdout.execute(LeaveAlternateScreen)?;

  terminal::disable_raw_mode()?;

  Ok(())
}
