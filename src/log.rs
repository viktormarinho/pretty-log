use std::sync::Mutex;

use colored::Colorize;

use crate::PrettyError;

pub static LOGGER: Mutex<Option<Logger>> = Mutex::new(None);
// Customize the global logger so that it contains a title on every message
//  
// Example:
// use pretty_log::log;
//
// log::init("MyApp");
// log::info("Hello, world!");
// log::warn("Something went wrong!");
// log::error("Something went very wrong!");
//
// Output:
// MyApp Hello, world!
// MyApp Something went wrong!
// MyApp Something went very wrong!
pub struct Logger {
    title: String,
}

pub fn message_prefix() -> String {
    match LOGGER
        .lock()
        .expect_p("Could not acquire global Logger lock")
        .as_ref()
    {
        Some(logger) => logger.title.clone(),
        None => String::from(""),
    }
}

pub fn init(title: &str) -> () {
    LOGGER
        .lock()
        .expect_p("Could not acquire global Logger lock")
        .replace(Logger {
            title: format!("{} ", title),
        });
}

pub fn info(message: &str) {
    println!("{}{}", message_prefix(), message.blue());
}

pub fn warn(message: &str) {
    println!("{}{}", message_prefix(), message.yellow());
}

pub fn error(message: &str) {
    println!("{}{}", message_prefix(), message.red().bold());
}

pub fn error_exit(message: &str) -> ! {
    println!("{}{}", message_prefix(), message.red().bold());
    std::process::exit(1);
}
