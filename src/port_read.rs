#[cfg(target_os = "linux")]
use serialport::TTYPort;

#[cfg(target_os = "linux")]
use serialport::SerialPort;

use std::{
    fs::{File, create_dir, remove_file},
    path::{Path},
    io::{Read, Write},
    str::from_utf8
};

#[cfg(target_os = "linux")]
pub fn read_stream_linux(mut device: TTYPort) {

    let temp_path = Path::new("./temp/temp_data.csv");
    let mut temp_file = match File::options().append(true).open(temp_path) {
        Ok(file) => file,
        Err(_e) => panic!("Failed to create temp file"),
    };

    let mut bit_buf: [u8; 1] = [0; 1];
    let mut line_buf: [u8; 32] = [0; 32];
    let mut line_count = 0;
    let mut reader = device.read(&mut bit_buf);
    let mut result: Vec<String> = Vec::new();

    while !reader.is_err() {
        reader = device.read(&mut bit_buf);
        let current_char = from_utf8(&bit_buf).unwrap();
        if current_char != "\n" {
            if current_char == " " {
                continue
            }
            line_buf[line_count] = bit_buf[0];
            line_count += 1;
        } else {
            line_buf[line_count] = bit_buf[0];
            for byte in line_buf {
                if byte != b'\0' {
                    let written = temp_file.write(&[byte]);
                    match written {
                        Ok(s) => println!("Ok line byte: {}", s),
                        Err(e) => eprintln!("Failed to write line: {}", e),
                    };
                    let mut field = String::new();
                    if from_utf8(&[byte]).unwrap() != "," {
                        field.push_str(from_utf8(&[byte]).unwrap())
                    } else {
                        result.push(field);
                    }
                } else {
                    continue
                }
            }
            //line_count = 0;
            //line_buf = [0; 32];
            break
        }
    }
}

#[cfg(target_os = "linux")]
pub fn read_stream_win(mut device: Box<dyn SerialPort>) {

    let temp_path = Path::new("./temp/temp_data.csv");
    let mut temp_file = match File::options().append(true).open(temp_path) {
        Ok(file) => file,
        Err(_e) => panic!("Failed to create temp file"),
    };

    let mut bit_buf: [u8; 1] = [0; 1];
    let mut line_buf: [u8; 32] = [0; 32];
    let mut line_count = 0;
    let mut reader = device.read(&mut bit_buf);
    let mut result: Vec<String> = Vec::new();

    while !reader.is_err() {
        reader = device.read(&mut bit_buf);
        let current_char = from_utf8(&bit_buf).unwrap();
        if current_char != "\n" {
            if current_char == " " {
                continue
            }
            line_buf[line_count] = bit_buf[0];
            line_count += 1;
        } else {
            line_buf[line_count] = bit_buf[0];
            for byte in line_buf {
                if byte != b'\0' {
                    let written = temp_file.write(&[byte]);
                    match written {
                        Ok(s) => println!("Ok line byte: {}", s),
                        Err(e) => eprintln!("Failed to write line: {}", e),
                    };
                    let mut field = String::new();
                    if from_utf8(&[byte]).unwrap() != "," {
                        field.push_str(from_utf8(&[byte]).unwrap())
                    } else {
                        result.push(field);
                    }
                } else {
                    continue
                }
            }
            //line_count = 0;
            //line_buf = [0; 32];
            break
        }
    }
}
