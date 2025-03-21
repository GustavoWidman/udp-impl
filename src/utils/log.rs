use colog::format::CologStyle;
use colored::Colorize;
use env_logger::Builder;
use log::Level;

use crate::cli;

struct CustomLevelTokens {
    pub mode: cli::Mode,
}
impl From<cli::Mode> for CustomLevelTokens {
    fn from(mode: cli::Mode) -> Self {
        Self { mode }
    }
}

impl CologStyle for CustomLevelTokens {
    fn level_token(&self, level: &Level) -> &str {
        match *level {
            Level::Error => "ERR",
            Level::Warn => "WRN",
            Level::Info => "INF",
            Level::Debug => "DBG",
            Level::Trace => "TRC",
        }
    }

    fn prefix_token(&self, level: &Level) -> String {
        format!(
            "{}{}{} {}{}{} {}{}{}",
            "[".blue().bold(),
            chrono::Local::now()
                .format("%Y-%m-%d %H:%M:%S.%6f")
                .to_string()
                .white()
                .bold(),
            "]".blue().bold(),
            "[".blue().bold(),
            self.level_color(level, self.level_token(level)),
            "]".blue().bold(),
            "[".blue().bold(),
            self.mode.to_string().to_uppercase().purple().bold(),
            "]".blue().bold()
        )
    }
}

pub struct Logger;

impl Logger {
    pub fn init(args: &cli::Args) {
        Builder::new()
            .filter(None, args.verbosity)
            .target(env_logger::Target::Stdout)
            .format(colog::formatter(CustomLevelTokens::from(cli::Mode::from(
                &args.command,
            ))))
            .write_style(env_logger::WriteStyle::Always)
            .init();
    }
}
