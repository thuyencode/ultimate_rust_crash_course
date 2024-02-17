use std::{
  env::current_dir,
  fs::{read_dir, ReadDir},
  io::Error,
  path::PathBuf,
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

  // Cleanup
  audio.wait();
  Ok(())
}
