#![allow(unknown_lints)]

#[cfg(target_os = "windows")]
extern crate winapi;

mod daemon;
pub use daemon::*;

mod singleton;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;
