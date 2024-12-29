use vg_usb_interface::windows;
use std::env;
use windows_sys::Win32::System::Console::FreeConsole;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
 //   unsafe { FreeConsole() };
    windows();
}
