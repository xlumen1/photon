#[cfg(test)]
mod tests;

mod cpu;
mod state;

pub use crate::cpu::{CPU,Status};
pub use crate::state::State;

