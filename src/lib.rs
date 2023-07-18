pub mod log;

use colored::Colorize;

pub trait PrettyError<T> {
    fn expect_p(self, message: &str) -> T;
}

impl<T, E> PrettyError<T> for Result<T, E> {
    fn expect_p(self, message: &str) -> T {
        match self {
            Ok(value) => value,
            Err(_) => {
                println!("{}", message.red().bold());
                std::process::exit(1);
            },
        }
    }
}

impl<T> PrettyError<T> for Option<T> {
    fn expect_p(self, message: &str) -> T {
        match self {
            Some(value) => value,
            None => {
                println!("{}", message.red().bold());
                std::process::exit(1);
            },
        }
    }
}