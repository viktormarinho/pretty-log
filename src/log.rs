use colored::Colorize;


// Custom logger that contains a title on every message
// I recommend to initialize it as a global variable using lazy_static!
// Example:
// lazy_static! {
//     pub static ref log: Logger = Logger::new("MyApp");
// }
//
// Then you can use it like this:
// log.info("Hello World!");
// log.warn("Something is wrong!");
// log.error("Something is very wrong!");
// log.error_exit("Something is very wrong and I can't continue!");
pub struct Logger {
    title: String,
}

impl Logger {
    pub fn new(title: &str) -> Logger {
        Logger {
            title: title.to_string(),
        }
    }

    pub fn info(&self, message: &str) {
        println!("{} {}", self.title, message.blue());
    }

    pub fn warn(&self, message: &str) {
        println!("{} {}", self.title, message.yellow());
    }
    
    pub fn error(&self, message: &str) {
        println!("{} {}", self.title, message.red().bold());
    }
    
    pub fn error_exit(&self, message: &str) -> ! {
        println!("{} {}", self.title, message.red().bold());
        std::process::exit(1);
    }
}


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