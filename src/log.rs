use colored::Colorize;
use chrono::Local;

pub const INFO:         LogType = LogType::INFO;
pub const WARN:         LogType = LogType::WARN;
pub const ERROR:        LogType = LogType::ERROR;
pub const FATAL:        LogType = LogType::FATAL;
pub const VERBOSE:      LogType = LogType::VERBOSE;
pub const SUCCESS:      LogType = LogType::SUCCESS;
pub const INPUT:        LogType = LogType::INPUT;
pub const SELECTION:    LogType = LogType::SELECTION;

pub enum LogType {
    INFO,
    WARN,
    ERROR,
    FATAL, 
    VERBOSE,
    SUCCESS,
    INPUT,
    SELECTION,
}

pub fn log(log_type: LogType, message: &str) {
    let now: String = Local::now().format("%M:%S").to_string();

    let log_type_str = match log_type {
        LogType::INFO       => "Info".cyan().bold(),
        LogType::WARN       => "Warning".yellow().bold(),
        LogType::ERROR      => "Error".red().bold(),
        LogType::FATAL      => "Fatal".bright_magenta().bold(),
        LogType::VERBOSE    => "Verbose".white().bold(),
        LogType::SUCCESS    => "Success".green().bold(),
        LogType::INPUT      => "Input".bright_blue().bold(),
        LogType::SELECTION  => "Select".white(),
    };
    
    match log_type {
        LogType::INPUT => print!("{} {}\t{}", 
            now.truecolor(128, 128, 128), 
            log_type_str, 
            message
        ),
        _ => println!("{} {}\t{}", 
            now.truecolor(128, 128, 128), 
            log_type_str, 
            message
        ),
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => (
        $crate::log::log($crate::log::INFO, &format!($($arg)*))
    )
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (
        $crate::log::log($crate::log::WARN, &format!($($arg)*))
    )
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (
        $crate::log::log($crate::log::ERROR, &format!($($arg)*))
    )
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => (
        $crate::log::log($crate::log::FATAL, &format!($($arg)*))
    )
}

#[macro_export]
macro_rules! verbose {
    ($($arg:tt)*) => (
        $crate::log::log($crate::log::VERBOSE, &format!($($arg)*))
    )
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => (
        $crate::log::log($crate::log::SUCCESS, &format!($($arg)*))
    )
}

#[macro_export]
macro_rules! input {
    ($($arg:tt)*) => (
        $crate::log::log($crate::log::INPUT, &format!($($arg)*))
    )
}

#[macro_export]
macro_rules! selection {
    ($($arg:tt)*) => (
        $crate::log::log($crate::log::SELECTION, &format!($($arg)*))
    )
}