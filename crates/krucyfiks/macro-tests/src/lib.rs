//! External consumer checks using a genuinely renamed dependency and no direct
//! dependency on async-trait, flume, or the procedural macro crate.

#![doc = include_str!("compile_contracts.md")]

#[cfg(test)]
mod tests;

#[cfg(test)]
mod green_path;
