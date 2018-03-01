use log::{self, Level, Metadata, Record};
use colored::*;

pub struct Logger {
    pub level: Level,
}
impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let flag = metadata.level() <= self.level;
        return flag;
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) && record.target().starts_with("ims") {
            let mut level = format!("{}", record.level());
            if level == "WARN" {
                level = "warning".to_string();
            }
            let level = level.to_uppercase();
            let color_level = match level.as_ref() {
                "TRACE" => level.bright_black(),
                "DEBUG" => level.bright_white(),
                "INFO" => level.green(),
                "WARNING" => level.yellow(),
                "ERROR" => level.red(),
                _ => level.normal(),
            };

            println!("{0:>12} {1}", color_level.bold(), record.args());
        }
    }
    fn flush(&self) {}
}
