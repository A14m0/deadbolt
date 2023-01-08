use colored::Colorize;

#[macro_export]
macro_rules! debug {
    () => {
        crate::log::log(crate::log::LogType::LogDebug, String::new());
    };
    ($($x:tt)*) => {
        crate::log::log(crate::log::LogType::LogDebug, std::format_args!($($x)*).to_string())
    };
}

#[macro_export]
macro_rules! info {
    () => {
        crate::log::log(crate::log::LogType::LogInfo, String::new());
    };
    ($($x:tt)*) => {
        crate::log::log(crate::log::LogType::LogInfo, std::format_args!($($x)*).to_string())
    };
}

#[macro_export]
macro_rules! warn {
    () => {
        crate::log::log(crate::log::LogType::LogWarn, String::new());
    };
    ($($x:tt)*) => {
        crate::log::log(crate::log::LogType::LogWarn, std::format_args!($($x)*).to_string())
    };
}

#[macro_export]
macro_rules! error {
    () => {
        log(LogType::LogErr, String::new());
    };
    ($($x:tt)*) => {{
        crate::log::log(crate::log::LogType::LogErr, std::format_args!($($x)*).to_string());
        println!("{}", std::backtrace::Backtrace::force_capture());
        std::process::exit(1);
    }};
}




pub enum LogType {
    LogDebug,
    LogInfo,
    LogWarn,
    LogErr
}

/// logs data with color and handy pre-labeling
pub fn log(
    ltype: LogType, 
    string: String
) {
    let strng = &string[..];
    _log(ltype, strng);
}

pub fn _log(
    ltype: LogType, 
    string: &str
) {
    // print fancy colors depending the input type
    match ltype {
        LogType::LogDebug   => {
            if cfg!(debug_assertions) {
                println!("{} - {}", "[DEBUG]".bright_cyan(), string.bright_blue())
            }
        }
        LogType::LogInfo    => println!("{} - {}", "[INFO]".green(), string),
        LogType::LogWarn    => println!("{} - {}", "[WARN]".yellow(), string),
        LogType::LogErr     => println!("{} - {}", "[FAIL]".red(), string),
    }
}