use colored::Colorize;

pub fn info(message: &str) {
    println!("{}", message.blue());
}

pub fn warn(message: &str) {
    println!("{}", message.yellow());
}

pub fn error(message: &str) {
    println!("{}", message.red().bold());
}

pub fn error_exit(message: &str) -> ! {
    println!("{}", message.red().bold());
    std::process::exit(1);
}