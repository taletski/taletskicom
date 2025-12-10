#![deny(
    clippy::print_stdout,
    clippy::print_stderr,
    clippy::dbg_macro,
    clippy::unimplemented,
    clippy::unwrap_used
)]

pub mod config;
pub mod server;

mod handlers;
mod middleware;
mod route;
