use std::{
    fs::{create_dir, File, remove_file},
    io::Write,
    path::Path,
};
use fltk::prelude::FltkError;

pub fn create_log() -> Option<()> {
    if !Path::new("./log").is_dir() {
        let new_dir = create_dir(Path::new("./log"));
        match new_dir {
            Ok(s) => println!("Created log directory: {:?}", s),
            Err(e) => eprintln!("Failed to create log directory: {}", e),
        }
    }

    let log_path = Path::new("./log/log.log");
    if log_path.is_file() {
        let remove = remove_file(log_path);
        match remove {
            Ok(s) => println!("Log file removed: {:?}", s),
            Err(e) => eprintln!("Failed to remove log file: {}", e),
        }
    }
    match File::create(log_path) {
        Ok(file) => file,
        Err(e) => panic!("Failed to open file: {}", e),
    };

    return Some(())
}

pub fn log<T: std::fmt::Display>(info: &T) {

    let log_path = Path::new("./log/log.log");
    let mut log_file = match File::options().append(true).open(log_path) {
        Ok(file) => file,
        Err(e) => panic!("Failed to open log file {}", e),
    };

    let result = log_file.write_fmt(format_args!("{}\n", info));
    match result {
        Ok(_s) => println!("Wrote log line: {}", info),
        Err(e) => eprintln!("Failed to write log line {}", e),
    }

}

pub fn log_error(gui_component: Result<(), FltkError>, component: &str) {

    let log_path = Path::new("./log/log.log");
    let mut log_file = match File::options().append(true).open(log_path) {
        Ok(file) => file,
        Err(e) => panic!("Failed to open log file {}", e),
    };

    let info = match gui_component {
        Ok(_s) => format!("Component: {} {:?}", "Ok", component),
        Err(e) => format!("Component: {} {} {:?}", "Error", component, e),
    };

    let result = log_file.write_fmt(format_args!("{:?}\n", info));
    match result {
        Ok(_s) => println!("Wrote log line: {}", info),
        Err(e) => eprintln!("Failed to write log line {}", e),
    }
}

