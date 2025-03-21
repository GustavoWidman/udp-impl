mod enums;
pub use enums::*;

use std::{
    net::{Ipv4Addr, SocketAddrV4, ToSocketAddrs},
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use log::LevelFilter;

#[derive(Parser, Debug)]
pub struct Args {
    /// Sets the logger's verbosity level
    #[arg(long, short, default_value_t = LevelFilter::Info)]
    pub verbosity: LevelFilter,

    // /// Allows the user to specify the mode of operation
    // #[arg(long, short, default_value_t = Mode::Client)]
    // pub mode: Mode,
    #[command(subcommand)]
    pub command: Subcommands,
    // /// Set the file to read (or write) from (or to)
    // #[arg(long, short)]
    // pub file: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    Listener(ListenerArgs),
    Sender(SenderArgs),
}
#[derive(Parser, Debug)]
pub struct SenderArgs {
    /// Set the port to send data to
    #[arg(long, short)]
    pub addr: SocketAddrV4,
}

#[derive(Parser, Debug)]
pub struct ListenerArgs {
    /// Set the IP address to listen on
    #[arg(long, short)]
    pub addr: SocketAddrV4,
}
