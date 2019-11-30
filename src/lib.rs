//pub mod parse_ssh_config;

//! # ssh_host
//! 
//! `ssh_host` is a simple crate holding the nessecary value to connect to a `ssh2` host.
//!
//! This crate is created with a maximum compatibility with [ssh2](https://docs.rs/ssh2/0.5.0/ssh2/index.html).
pub use crate::ssh_host::SshHost;

mod ssh_host;
