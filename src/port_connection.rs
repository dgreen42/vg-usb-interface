#[cfg(target_os = "linux")]
use serialport::{
    DataBits, FlowControl, Parity, SerialPort, StopBits, TTYPort, Error, ErrorKind, Result
};

#[cfg(target_os = "macos")]
use serialport::{
    DataBits, FlowControl, Parity, SerialPort, StopBits, TTYPort, Error, ErrorKind, Result
};

#[cfg(target_os = "windows")]
use serialport::{
    DataBits, FlowControl, Parity, SerialPort, SerialPortBuilder, StopBits, Error, ErrorKind, Result 
};

use std:: time::Duration;
use crate::logger;


#[cfg(target_os = "windows")]
pub fn connect_port_win(
    path: &str,
    baud_rate: u32,
    parity: &str,
    time_out: u64,
    exclusivity: bool,
    data_bits: &str,
    flow_control: &str,
    stop_bits: &str,
) -> Result<Box<dyn SerialPort>>
{
    let device = match serialport::new(path, baud_rate).open() {
        Ok(port) => Some(port),
        Err(_e) => None,
    };

    if !device.is_some() {
        return Err(Error {kind: ErrorKind::InvalidInput, description: String::from("Could not open device")})
    }

    logger::log(&format!("Device open success: {:?}", device));

    let device = port_settings_win(device.unwrap(), parity, time_out, data_bits, flow_control, stop_bits);

    logger::log(&format!("Device options set: {:?}", device));
    
    return Ok(device)
}

#[cfg(target_os = "windows")]
fn port_settings_win(
    mut device: Box<dyn SerialPort>,
    parity: &str,
    time_out: u64,
    //exclusivity: bool,
    data_bits: &str,
    flow_control: &str,
    stop_bits: &str,
    ) -> Box<dyn SerialPort> {

    let par = match parity {
        "None" => Parity::None,
        "Odd" => Parity::Odd,
        "Even" => Parity::Even,
        _ => Parity::None, // default is none
    };

    let dsp = device.set_parity(par);
    match dsp {
        Ok(_s) => println!("Parity set to: {}", par),
        Err(e) => eprintln!("Failed to set parity {}", e),
    };

    let dur = Duration::new(time_out, 0);
    let dsto = device.set_timeout(dur);
    match dsto {
        Ok(_s) => println!("Timeout set to: {:?}", dur),
        Err(e) => eprintln!("Failed to set timeout {}", e),
    };

    /*
    let dse = device.set_exclusive(exclusivity);
    match dse {
        Ok(_s) => println!("Exculsivity set to: {:?}", dur),
        Err(e) => eprintln!("Failed to set exclusivity {}", e),
    };
    */

    let d_bits = match data_bits {
        "Five" => DataBits::Five,
        "Six" => DataBits::Six,
        "Seven" => DataBits::Seven,
        "Eight" => DataBits::Eight,
        _ => DataBits::Eight, // default is 8 
    };
    let dsdb = device.set_data_bits(d_bits);
    match dsdb {
        Ok(_s) => println!("Data Bits set to: {:?}", d_bits),
        Err(e) => eprintln!("Failed to set data bits {}", e),
    };

    let f_control = match flow_control {
        "None" => FlowControl::None,
        "Software" => FlowControl::Software,
        "Hardware" => FlowControl::Hardware,
        _ => FlowControl::None,
    };
    let dsfc = device.set_flow_control(f_control);
    match dsfc {
        Ok(_s) => println!("Flow Control set to: {:?}", f_control),
        Err(e) => eprintln!("Failed to set flow control {}", e),
    };

    let s_bits = match stop_bits {
        "One" => StopBits::One,
        "Two" => StopBits::Two,
        _ => StopBits::One,
    };
    let dssb = device.set_stop_bits(s_bits);
    match dssb {
        Ok(_s) => println!("Stop Bits set to: {:?}", s_bits),
        Err(e) => eprintln!("Failed to set Stop Bits {}", e),
    };

    return device 
}

#[cfg(target_os = "linux")]
pub fn connect_port_tty(
    path: &str,
    baud_rate: u32,
    parity: &str,
    time_out: u64,
    exclusivity: bool,
    data_bits: &str,
    flow_control: &str,
    stop_bits: &str,
) -> Result<TTYPort>
{
    let device = match TTYPort::open(&serialport::new(path, baud_rate)) {
        Ok(port) => Some(port),
        Err(_e) => None,
    };

    if !device.is_some() {
        return Err(Error {kind: ErrorKind::InvalidInput, description: String::from("Could not open device")})
    }

    logger::log(&format!("Device open success: {:?}", device));

    let device = port_settings_tty(device.unwrap(), parity, time_out, exclusivity, data_bits, flow_control, stop_bits);

    logger::log(&format!("Device options set: {:?}", device));

    return Ok(device);
}

#[cfg(target_os = "linux")]
fn port_settings_tty(
    mut device: TTYPort,
    parity: &str,
    time_out: u64,
    exclusivity: bool,
    data_bits: &str,
    flow_control: &str,
    stop_bits: &str,
    ) -> TTYPort 
{

    let par = match parity {
        "None" => Parity::None,
        "Odd" => Parity::Odd,
        "Even" => Parity::Even,
        _ => Parity::None, // default is none
    };

    let dsp = device.set_parity(par);
    match dsp {
        Ok(_s) => println!("Parity set to: {}", par),
        Err(e) => eprintln!("Failed to set parity {}", e),
    };

    let dur = Duration::new(time_out, 0);
    let dsto = device.set_timeout(dur);
    match dsto {
        Ok(_s) => println!("Timeout set to: {:?}", dur),
        Err(e) => eprintln!("Failed to set timeout {}", e),
    };

    let dse = device.set_exclusive(exclusivity);
    match dse {
        Ok(_s) => println!("Exculsivity set to: {:?}", dur),
        Err(e) => eprintln!("Failed to set exclusivity {}", e),
    };

    let d_bits = match data_bits {
        "Five" => DataBits::Five,
        "Six" => DataBits::Six,
        "Seven" => DataBits::Seven,
        "Eight" => DataBits::Eight,
        _ => DataBits::Eight, // default is 8 
    };
    let dsdb = device.set_data_bits(d_bits);
    match dsdb {
        Ok(_s) => println!("Data Bits set to: {:?}", d_bits),
        Err(e) => eprintln!("Failed to set data bits {}", e),
    };

    let f_control = match flow_control {
        "None" => FlowControl::None,
        "Software" => FlowControl::Software,
        "Hardware" => FlowControl::Hardware,
        _ => FlowControl::None,
    };
    let dsfc = device.set_flow_control(f_control);
    match dsfc {
        Ok(_s) => println!("Flow Control set to: {:?}", f_control),
        Err(e) => eprintln!("Failed to set flow control {}", e),
    };

    let s_bits = match stop_bits {
        "One" => StopBits::One,
        "Two" => StopBits::Two,
        _ => StopBits::One,
    };
    let dssb = device.set_stop_bits(s_bits);
    match dssb {
        Ok(_s) => println!("Stop Bits set to: {:?}", s_bits),
        Err(e) => eprintln!("Failed to set Stop Bits {}", e),
    };

    return device 
}

#[cfg(target_os = "macos")]
pub fn connect_port_tty(
    path: &str,
    baud_rate: u32,
    parity: &str,
    time_out: u64,
    exclusivity: bool,
    data_bits: &str,
    flow_control: &str,
    stop_bits: &str,
) -> Option<serialport::TTYPort>
{
    let device = match TTYPort::open(&serialport::new(path, baud_rate)) {
        Ok(port) => Some(port),
        Err(_e) => None,
    };

    if !device.is_some() {
        println!("Could not open device");
        return None
    }

    logger::log(&format!("Device open success: {:?}", device));

    let device = port_settings_tty(device.unwrap(), parity, time_out, exclusivity, data_bits, flow_control, stop_bits);
    
    logger::log(&format!("Device options set: {:?}", device));

    return Some(device);
}

#[cfg(target_os = "macos")]
fn port_settings_tty(
    mut device: TTYPort,
    parity: &str,
    time_out: u64,
    exclusivity: bool,
    data_bits: &str,
    flow_control: &str,
    stop_bits: &str,
    ) -> TTYPort 
{

    let par = match parity {
        "None" => Parity::None,
        "Odd" => Parity::Odd,
        "Even" => Parity::Even,
        _ => Parity::None, // default is none
    };

    let dsp = device.set_parity(par);
    match dsp {
        Ok(_s) => println!("Parity set to: {}", par),
        Err(e) => eprintln!("Failed to set parity {}", e),
    };

    let dur = Duration::new(time_out, 0);
    let dsto = device.set_timeout(dur);
    match dsto {
        Ok(_s) => println!("Timeout set to: {:?}", dur),
        Err(e) => eprintln!("Failed to set timeout {}", e),
    };

    let dse = device.set_exclusive(exclusivity);
    match dse {
        Ok(_s) => println!("Exculsivity set to: {:?}", dur),
        Err(e) => eprintln!("Failed to set exclusivity {}", e),
    };

    let d_bits = match data_bits {
        "Five" => DataBits::Five,
        "Six" => DataBits::Six,
        "Seven" => DataBits::Seven,
        "Eight" => DataBits::Eight,
        _ => DataBits::Eight, // default is 8 
    };
    let dsdb = device.set_data_bits(d_bits);
    match dsdb {
        Ok(_s) => println!("Data Bits set to: {:?}", d_bits),
        Err(e) => eprintln!("Failed to set data bits {}", e),
    };

    let f_control = match flow_control {
        "None" => FlowControl::None,
        "Software" => FlowControl::Software,
        "Hardware" => FlowControl::Hardware,
        _ => FlowControl::None,
    };
    let dsfc = device.set_flow_control(f_control);
    match dsfc {
        Ok(_s) => println!("Flow Control set to: {:?}", f_control),
        Err(e) => eprintln!("Failed to set flow control {}", e),
    };

    let s_bits = match stop_bits {
        "One" => StopBits::One,
        "Two" => StopBits::Two,
        _ => StopBits::One,
    };
    let dssb = device.set_stop_bits(s_bits);
    match dssb {
        Ok(_s) => println!("Stop Bits set to: {:?}", s_bits),
        Err(e) => eprintln!("Failed to set Stop Bits {}", e),
    };

    return device 
}
