mod gui;
mod port_read;
mod read_write_utils;
mod port_connection;
mod table_functions;
mod logger;
mod start_gui_linux;
mod start_gui_windows;
//    for vg-usb-meter //
/// baudrate = 9600
/// 8bits
/// No parity
/// 1 stop bits
/// EOL = \r
/// data comes as a csv line

#[cfg(target_os = "linux")]
pub fn linux() {
    start_gui_linux::start_gui_linux::start_gui();
}

#[cfg(target_os = "windows")]
pub fn windows() {
    start_gui_windows::start_gui_windows::start_gui();
}

#[cfg(target_os = "macos")]
pub fn macos() {

}

