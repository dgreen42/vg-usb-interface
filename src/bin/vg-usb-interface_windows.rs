use vg_usb_interface::windows;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    windows();
}
