/// Logger implementation for colors in the terminal and in the future
/// logging to a file and to a remote log server

extern crate chrono;
extern crate colored;
extern crate log;

use colored::*;
use log::{Log, LogLevel, LogLevelFilter, LogMetadata, LogRecord, set_logger};

#[cfg(test)]
mod tests;
mod error;

use error::RagResult;

struct Logger;

impl Log for Logger {
	fn enabled(&self, metadata: &LogMetadata) -> bool {
		metadata.level() <= LogLevel::Info
	}

	fn log(&self, record: &LogRecord) {
		if self.enabled(record.metadata()) {
			let now = chrono::Local::now();
			let ts = now.format("%Y-%m-%dT%H:%M:%S%.3f%z").to_string();

			let (msg, _level) = match record.level() {
				LogLevel::Error => {
					(format!("{} {} {}", ts.white().bold(), "ERR".red(), record.args()), 3)
				}
				LogLevel::Warn => {
					(format!("{} {} {}", ts.white().bold(), "WRN".purple(), record.args()), 4)
				}
				LogLevel::Info => {
					(format!("{} {} {}", ts.white().bold(), "INF".cyan(), record.args()), 6)
				}
				LogLevel::Debug => {
					(format!("{} {} {}", ts.white().bold(), "DBG".yellow(), record.args()), 7)
				}
				LogLevel::Trace => {
					(format!("{} {} {}", ts.white().bold(), "TRC".green(), record.args()), 0)
				}
			};
			println!("{}", msg);
		}
	}
}

pub fn init() -> RagResult<()> {
	Ok(set_logger(|max_log_level| {
		max_log_level.set(LogLevelFilter::Info);
		Box::new(Logger)
	})?)
}
