use crate::serial_println;
use log::{LevelFilter, Log, Metadata, Record};

struct SerialLogger;

pub fn init_log() {
    log::set_logger(&SerialLogger).unwrap();
    log::set_max_level(LevelFilter::Trace);
}

impl Log for SerialLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        serial_println!("[{}] {}", record.level(), record.args());
    }

    fn flush(&self) {}
}
