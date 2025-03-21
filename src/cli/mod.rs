use std::net::SocketAddrV4;

use clap::{Parser, Subcommand};
use log::LevelFilter;

#[derive(Parser, Debug)]
pub struct Args {
    /// Sets the logger's verbosity level
    #[arg(long, short, default_value_t = LevelFilter::Info)]
    pub verbosity: LevelFilter,

    #[command(subcommand)]
    pub command: Subcommands,
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
    /// Set the IP address to bind to
    #[arg(long, short)]
    pub bind: SocketAddrV4,
}

#[derive(Parser, Debug)]
pub struct ListenerArgs {
    /// Set the IP address to listen on
    #[arg(long, short)]
    pub addr: SocketAddrV4,
}

pub enum Mode {
    Listener,
    Sender,
}
impl From<&Subcommands> for Mode {
    fn from(subcommand: &Subcommands) -> Self {
        match subcommand {
            Subcommands::Listener(_) => Mode::Listener,
            Subcommands::Sender(_) => Mode::Sender,
        }
    }
}
impl ToString for Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::Listener => "listener".to_string(),
            Mode::Sender => "sender".to_string(),
        }
    }
}
