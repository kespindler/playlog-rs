extern crate serde_json;
extern crate serde;
extern crate chrono;
extern crate nix;

use handler::Handler;
use self::serde::ser::{Serialize, Serializer, SerializeMap};
use std::io::stdout;
use self::chrono::Utc;
//use serde_json::Serializer;
use std::process;
use std::io::prelude::*;
use record::Record;


fn get_hostname() -> String {
    /// Copied from slog-rs/bunyan
    let mut buf = vec!(0u8; 256);
    match nix::unistd::gethostname(&mut buf) {
        Ok(hostname_c) => {
            // TODO: BUG: use locale to known encoding?
            hostname_c.to_string_lossy().into()
        }
        Err(_) => "n/a".to_string(),
    }
}


pub struct JsonHandler {
    // Can't get ownership working for the stream. hard-code for now.
//    pub stream: Write,
    pub pretty_print: bool,
}


impl Serialize for Record {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        // ts, level, name, host, pid, message, data = 7
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("ts", &format!("{:?}", Utc::now()))?;
        map.serialize_entry("level", &format!("{}", self.level))?;
        map.serialize_entry("name", &self.name)?;
        // TODO make hostname lazy_static so we don't compute every time.
        map.serialize_entry("host", &get_hostname())?;
        map.serialize_entry("pid", &process::id())?;
        map.serialize_entry("msg", &self.message)?;
        map.serialize_entry("data", &self.data)?;
        map.end()
    }
}


impl Handler for JsonHandler {
    fn write(&self, record: &Record) -> () {
        let s = serde_json::to_string(record).unwrap() + "\n";
        stdout().write(s.as_bytes());
    }
}

#[cfg(test)]
mod tests {}
