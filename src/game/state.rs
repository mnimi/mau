//! Implements a simplistic state-tracking system.

pub trait State
{
  // TODO.
  fn update();
  fn display();
  fn context();
}
