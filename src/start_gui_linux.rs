#[cfg(target_os = "linux")]
pub mod start_gui_linux {
    use fltk:: prelude::*;
    use std::{
        fs::{create_dir, remove_file, File}, path::Path,
    };

    use crate::gui;
    use crate::port_read;
    use crate::port_connection;
    use crate::logger;
    use crate::read_write_utils;
    use crate::table_functions;

    pub fn start_gui() {

        let _log_file_result = match logger::create_log() {
            Some(file) => file,
            None => panic!("Failed to create temp file"),
        };

        let gui_comp = gui::create_window();
        let app = gui_comp.0;
        let reciever = gui_comp.1;
        let mut device_settings_choices = gui_comp.2.0;
        let device_settings_input = gui_comp.2.1;
        let _read_write_buttons = gui_comp.3.0;
        let read_write_input = gui_comp.3.1;
        let mut read_write_output = gui_comp.3.2;
        let table = gui_comp.4;

        let mut device = String::new();
        let mut baud_rate: u32 = 0;
        let mut parity = String::new();
        let mut timeout: u64 = 0; 
        let mut exclusivity = false;
        let mut data_bits = String::new();
        let mut stop_bits = String::new();
        let mut flow_control = String::new();
        let mut data: Vec<String> = Vec::new();
        let mut active_read = 0;
        let mut device_status = String::new();

        let mut file_name = String::new();

        let temp_dir = Path::new("./temp");
        let temp_path = Path::new("./temp/temp_data.csv");
        if temp_path.is_file() {
            let remove = remove_file(temp_path);
            match remove {
                Ok(s) => logger::log(&format!("Temp file removed: {:?}", s)),
                Err(e) => logger::log(&format!("Failed to remove temp file: {}", e)),
            }
        }
        if !temp_dir.is_dir() {
            let dir = create_dir(temp_dir);
            match dir {
                Ok(_s) => logger::log(&format!("Created temp directory {:?}", temp_dir)),
                Err(e) => logger::log(&format!("Failed to create temp directory {:?}", e)),
            }
        }
        if !temp_path.is_file() {
            let file = File::create_new(temp_path);
            match file {
                Ok(s) => logger::log(&format!("Created temp file {:?}", s)),
                Err(e) => logger::log(&format!("Failed to create temp file {:?}", e)),
            }
        }

        while app.wait() {

            let log_path = Path::new("./log/log.log");
            let log_file = match File::options().append(true).open(log_path) {
                Ok(file) => file,
                Err(e) => panic!("Failed to open log file {}", e),
            };

            if let Some(message) = reciever.recv() {
                logger::log(&format!("Message passed {:?}", message));
                match message {
                    gui::Message::Parity => {
                        parity = device_settings_choices[1].choice().unwrap();
                        logger::log(&format!("{} {} {} {} {} {} {} {}", parity, exclusivity, baud_rate, data_bits, flow_control, stop_bits, timeout, device));
                    },
                    gui::Message::Exclusivity => {
                        let c_exclusivity = device_settings_choices[2].choice().unwrap();
                        match c_exclusivity.as_ref() {
                            "Yes" => exclusivity = true,
                            "No" => exclusivity = false,
                            _ => exclusivity = false,
                        };
                        logger::log(&format!("{} {} {} {} {} {} {} {}", parity, exclusivity, baud_rate, data_bits, flow_control, stop_bits, timeout, device));
                    },
                    gui::Message::BaudRate => {
                        baud_rate = device_settings_choices[3].choice().unwrap().parse::<u32>().unwrap();
                        logger::log(&format!("{} {} {} {} {} {} {} {}", parity, exclusivity, baud_rate, data_bits, flow_control, stop_bits, timeout, device));
                    },
                    gui::Message::DataBits => {
                        data_bits = device_settings_choices[4].choice().unwrap();
                        logger::log(&format!("{} {} {} {} {} {} {} {}", parity, exclusivity, baud_rate, data_bits, flow_control, stop_bits, timeout, device));
                    },
                    gui::Message::FlowControl => {
                        flow_control = device_settings_choices[5].choice().unwrap();
                        logger::log(&format!("{} {} {} {} {} {} {} {}", parity, exclusivity, baud_rate, data_bits, flow_control, stop_bits, timeout, device));
                    },
                    gui::Message::StopBits => {
                        stop_bits = device_settings_choices[6].choice().unwrap();
                        logger::log(&format!("{} {} {} {} {} {} {} {}", parity, exclusivity, baud_rate, data_bits, flow_control, stop_bits, timeout, device));
                    },
                    gui::Message::Duration => {
                        timeout = device_settings_input.value().parse::<u64>().unwrap();
                        logger::log(&format!("{} {} {} {} {} {} {} {}", parity, exclusivity, baud_rate, data_bits, flow_control, stop_bits, timeout, device));
                    },
                    gui::Message::Device => {
                        device = device_settings_choices[0].choice().unwrap();
                        logger::log(&format!("{} {} {} {} {} {} {} {}", parity, exclusivity, baud_rate, data_bits, flow_control, stop_bits, timeout, device));
                    },
                    gui::Message::Read => {
                        active_read = 1;
                    },
                    gui::Message::Stop => {
                        active_read = 0;
                    },
                    gui::Message::Close => {
                        device = String::new();
                    },
                    gui::Message::FileName => {},
                    gui::Message::Write => {
                        file_name = read_write_input.value();
                        logger::log(&format!("file name set: {}", file_name));
                        let file_path = Path::new(&file_name);
                        read_write_utils::write_file(file_path, temp_path);
                    },
                    gui::Message::SetDefaults => {
                        gui::create_options_window();
                    },
                    gui::Message::Preferences => {},
                    _ => {}
                }

            }


            if active_read == 1 {
                let con_device = match port_connection::connect_port_tty(&device , baud_rate, &parity, timeout, exclusivity, &data_bits, &flow_control, &stop_bits) {

                    Ok(dev) => {
                        port_read::read_stream_linux(dev);
                    },
                    Err(e) => {
                        logger::log_connection_error_tty(e, &device);
                    }
                };
            }

            if device.is_empty() {
                read_write_output.set_value(&"No device connected");
                device_settings_choices[0].set_value(0);
            } else {
                match active_read {
                    0 => {
                        device_status = "Inactive".to_string();
                        read_write_output.set_value(&format!("{}: {}", device, device_status));
                    },
                    1 => {
                        device_status = "Reading".to_string();
                        read_write_output.set_value(&format!("{}: {}", device, device_status));
                    },
                    _ => {},
                }
            }

            data = read_write_utils::read_temp(temp_path);
            // change unit to the units that are selected from the device
            let headers = vec!["Moisture".to_string(), "unit".to_string(), "Light".to_string(), "unit".to_string(), "Temp".to_string(), "unit".to_string(), "Time".to_string()];
            table_functions::draw_table(data, table.clone(), headers);

            app.redraw();

        } 

        app.run().unwrap();
    }
}
