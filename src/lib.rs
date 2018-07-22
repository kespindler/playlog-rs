extern crate log;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;


pub use log::Level;
#[macro_use]
mod map;
mod record;
mod handler;
mod json_handler;
use record::{Record, Args};
use json_handler::JsonHandler;
use handler::Handler;


fn __get_log_level(name: &str) -> Level {
    /// This is a stand-in for now,
    /// but this should be controlled similar to env_logger,
    /// or maybe support other methods for logging control.
    // TODO implement
    Level::Info
}


const DEFAULT_HANDLER: JsonHandler = JsonHandler{
    /// Stand-in for now. We do want a sane default, but
    /// Json is probably not that sane default.
    pretty_print: true
};


fn __get_log_handlers(name: &str) -> Vec<impl Handler> {
    /// This is a stand-in for now, but
    /// eventually we'd have methods to register handlers
    /// for any particular name.
    // TODO implement
    let v = vec!{
        DEFAULT_HANDLER,
    };
    v
}


fn __log(name: &str, level: Level, message: &str, args: Args) {
    let r = Record {
        name: name.to_string(),
        message: message.to_string(),
        data: args,
        level,
    };
    __get_log_handlers(name).iter().for_each(|h| {
        h.write(&r);
    });
}

/// Generic log macro.
/// name: the name of the logger (defaults to `module_path!()`)
/// level: the logging level
/// message: the event name
/// args: data to add to the log statement
macro_rules! log {
    ( $name:expr, $level:expr, $message:expr, $args:expr ) => {
        {
            if __get_log_level($name) >= $level {
                __log($name, $level, $message, $args);
            }
        }
    };
    ( $level:expr, $message:expr, $args:expr ) => {
        {
            let name = module_path!();
            if __get_log_level(name) >= $level {
                __log(name, $level, $message, $args);
            }
        }
    };
}


#[macro_export]
macro_rules! info {
    ( $message:expr , $($i:ident : $e:expr ),*  ) => {
        {
            let data = json!({
            $(
                stringify!($i): $e,
            )*
            });
            log!(Level::Info, $message, data);
        }
    };
}


#[cfg(test)]
mod tests {
    #[macro_use]
    use super::*;

    #[test]
    fn info_macro() {
        info!("request received",
            a: 1,
            b: 2
        );
        assert!(true);
    }
}
