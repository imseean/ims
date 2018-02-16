use log::{self, Level, Metadata, Record};
pub struct Logger {
    pub level: Level,
}
impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let flag = metadata.level() <= self.level;
        return flag;
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) &&record.target().starts_with("ims"){

            let mut level = format!("{}", record.level());
            if level == "WARN" {
                level = "warning".to_string();
            }
            println!("{}: {}", level.to_lowercase(), record.args());
        }
    }
    fn flush(&self) {}
}
