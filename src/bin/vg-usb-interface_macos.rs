use vg_usb_interface::macos;
use std::env;

fn main() {

    env::set_var("RUST_BACKTRACE", "1");
    linux();
}
