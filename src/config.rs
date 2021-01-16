//! Implements configurations-related data structures.

pub const APP_SETTINGS_FILENAME: &str = "app.toml";
pub const GAME_SETTINGS_FILENAME: &str = "game_data.toml";

#[derive(Debug)]
pub struct AppSettings
{
  title: String,
  description: String,
  display_res: (u32, u32),
}

impl AppSettings
{
  pub fn new() -> Self
  {
    Self {
      title: "".to_string(),
      description: "".to_string(),
      display_res: (1280, 720)
    }
  }
}

#[derive(Debug)]
pub struct GameSettings
{
  username: String,
}
