use std::{
  env::current_dir,
  fs::{read_dir, ReadDir},
  io::{self, Error},
  path::PathBuf,
  sync::mpsc,
  thread,
  time::Duration,
};

use crossterm::{
  cursor::{Hide, Show},
  event::{self, Event, KeyCode},
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
  ExecutableCommand,
};
use invaders::{
  frame::{self, Drawable},
  player::Player,
  render,
};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Get the current working directory
  let current_dir: Result<PathBuf, Error> = current_dir();

  if !current_dir.is_ok() {
    return Err(current_dir.unwrap_err().into());
  }

  // Get the list of files inside the "sounds" folder
  let audio_files: Result<ReadDir, Error> =
    read_dir(format!("{}/sounds", current_dir.unwrap().to_string_lossy()));

  if !audio_files.is_ok() {
    return Err(audio_files.unwrap_err().into());
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

    loop {
      let current_frame: Vec<Vec<&str>> = match render_rx.recv() {
        Ok(x) => x,
        Err(_) => break,
      };

      render::render(&mut stdout, &last_frame, &current_frame, false);

      last_frame = current_frame;
    }
  });

  // Game loop
  let mut player = Player::new();

  'gameloop: loop {
    // Per-frame init
    let mut current_frame = frame::new_frame();

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
          _ => {}
        }
      }
    }

    // Draw and render
    player.draw(&mut current_frame);
    let _ = render_tx.send(current_frame);

    thread::sleep(Duration::from_millis(5));
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
