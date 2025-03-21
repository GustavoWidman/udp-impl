use std::io::Result;

use clap::Parser;
use colored::Colorize;

mod cli;
mod common;
mod proto;
mod utils;

// Example usage
pub fn main() -> Result<()> {
    let args = cli::Args::parse();
    utils::log::Logger::init(&args);

    match args.command {
        cli::Subcommands::Listener(listener_args) => {
            let socket = proto::socket::UDPSocket::new(listener_args.addr)?;
            log::info!(
                "UDP socket created successfully! Listening in on {}",
                listener_args.addr.to_string().green().bold()
            );

            loop {
                let (response, from_addr) = socket.receive_packet(1024)?;
                log::info!(
                    "Received packet from {}:\n\n{}\n",
                    from_addr.to_string().blue(),
                    response.to_string()
                );
            }
        }
        cli::Subcommands::Sender(sender_args) => {
            let src_socket = proto::socket::UDPSocket::new(sender_args.bind)?;
            log::info!(
                "UDP socket created successfully! Bount to {}",
                sender_args.bind.to_string().green().bold()
            );

            for line_result in std::io::stdin().lines() {
                match line_result {
                    Ok(mut line) => {
                        line.push('\n');
                        let payload = line.as_bytes().to_vec();
                        let (_, elapsed) = utils::macros::timeit!({
                            src_socket.send(payload, &sender_args.addr)?
                        });
                        log::info!("Packet sent! Took {}", format!("{:?}", elapsed).blue());
                    }
                    Err(e) => {
                        log::error!("Error reading line:\n\n{}\n", e);
                    }
                }
            }
        }
    }

    Ok(())
}
