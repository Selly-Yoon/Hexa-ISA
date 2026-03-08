use std::fs::{File, OpenOptions};
use std::io::Write;
use crate::event::TraceEvent;

pub struct TraceLogger {
    writer: Box<dyn Write>,
}

impl TraceLogger {
    pub fn to_file(path: &str) -> std::io::Result<Self> {
        let f = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(TraceLogger { writer: Box::new(f) })
    }

    pub fn to_stdout() -> Self {
        TraceLogger { writer: Box::new(std::io::stdout()) }
    }

    pub fn emit(&mut self, event: &TraceEvent) {
        if let Ok(line) = serde_json::to_string(event) {
            let _ = writeln!(self.writer, "{}", line);
        }
    }
}
