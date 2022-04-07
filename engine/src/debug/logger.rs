use std::sync::{Mutex, Arc};

use crate::events::{EventChannel, StatefulEventChannel};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Debug,
    Warn,
    Error
}

#[derive(Default)]
pub struct Logger {
    event_channel: Mutex<StatefulEventChannel<LogLevel, String>>
}

impl Logger {

    fn log(&self, level: LogLevel, msg: &str) {
        let mut channel = self.event_channel.lock();
        channel.as_mut().unwrap().publish((level, msg.to_string()));        
    }

    pub fn warn(&self, msg: &str) {
        self.log(LogLevel::Warn, msg);
    }

    pub fn info(&self, msg: &str) {
        self.log(LogLevel::Info, msg);
    }
    pub fn debug(&self, msg: &str) {
        self.log(LogLevel::Debug, msg);
    }
    pub fn error(&self, msg: &str) {
        self.log(LogLevel::Error, msg);
    }
}

