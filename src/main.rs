//! Knock-off Among Us game project.
//!
//! Written in Rust, using Amethyst as the game engine.
//!
//! To build the game, you can run 'cargo build'.
//!
//! --BUILD/RUN--------------------------------------------------------
//! -------------------------------------------------------------------
//! To start the game in normal mode, you can run 'cargo run -- basic'
//! To start the game in expert mode, you can run 'cargo run -- expert'

extern crate amethyst;

#[macro_use]
extern crate serde;
extern crate serde_derive;

#[macro_use]
extern crate toml;

pub mod config;
pub mod game;

fn main()
{

}
