use std::{
    fs::{create_dir, File, remove_file},
    io::Write,
    path::Path,
};
use fltk::prelude::FltkError;

#[cfg(target_os = "linux")]
use serialport::{
    TTYPort,
};

#[cfg(target_os = "windows")]
use serialport::{
    SerialPort,
};


pub fn create_log() -> Option<()> {
    if !Path::new("./log").is_dir() {
        let new_dir = create_dir(Path::new("./log"));
        match new_dir {
            Ok(_s) => (),
            Err(e) => panic!("Failed to create log directory: {}", e),
        }
    }

    let log_path = Path::new("./log/log.log");
    if log_path.is_file() {
        let remove = remove_file(log_path);
        match remove {
            Ok(_s) => (),
            Err(e) => panic!("Failed to remove log file: {}", e),
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
        Ok(_s) => (),
        Err(e) => panic!("Failed to write log line {}", e),
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

    let result = log_file.write_fmt(format_args!("{:?} :: ", info));
    match result {
        Ok(_s) => log(&format!("Wrote log line: {}", info)),
        Err(e) => log(&format!("Failed to write log line {}", e)),
    }
}

pub fn log_connection_error_tty(error: serialport::Error, name: &str) -> serialport::Error {
    log(&format!("Failed to connect to device: {} :: {:?}", name, error));
    error
}

pub fn log_connection_error_win(error: serialport::Error, name: &str) -> serialport::Error {
    log(&format!("Failed to connect to device: {} :: {:?}", name, error));
    error
}

