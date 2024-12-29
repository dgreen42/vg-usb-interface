#[cfg(target_os = "macos")]
pub mod start_gui_macos {
    use fltk::{
        prelude::*,
        text::TextBuffer,
    };
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

        // 0: App
        // 1: Sender
        // 2: Revicer
        // 3: Device Settings main
        // 4.0: Read Write::Buttons
        // 4.1: Read Write::Input
        // 5: Talbe

        let gui_comp = gui::create_window();
        let app = gui_comp.0;
        let sender = gui_comp.1;
        let reciever = gui_comp.2;
        let device_choice = gui_comp.3.0;
        let mut device_status = gui_comp.3.1;
        let mut device_read_type = gui_comp.3.2;
        let _read_write_buttons = gui_comp.4.0;
        let read_write_input = gui_comp.4.1;
        let table = gui_comp.5;

        let device_settings = gui::create_options_window(&sender);
        let mut device_settings_choices = device_settings.0.1.0;
        let mut device_settings_input = device_settings.0.1.1;
        let mut options_window = device_settings.1;

        let mut device = String::new();
        let mut baud_rate: u32 = 9600;
        let mut parity = String::from("None");
        let mut timeout: u64 = 10; 
        let mut exclusivity = false;
        let mut data_bits = String::from("8");
        let mut stop_bits = String::from("1");
        let mut flow_control = String::from("None");
        let mut data: Vec<String> = Vec::new();
        let mut active_read = 0;
        let mut device_status_state = String::new();
        let mut file_name = String::new();
        let mut read_type = String::from("Active");

        device_settings_choices[0].set_value(0);
        device_settings_input.set_value("10");
        device_settings_choices[1].set_value(0);
        device_settings_choices[2].set_value(0);
        device_settings_choices[3].set_value(3);
        device_settings_choices[4].set_value(0);
        device_settings_choices[5].set_value(0);


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
                        parity = device_settings_choices[0].choice().unwrap();
                        logger::log(&format!("{} {} {} {} {} {} {} {}", parity, exclusivity, baud_rate, data_bits, flow_control, stop_bits, timeout, device));
                    },
                    gui::Message::Exclusivity => {
                        let c_exclusivity = device_settings_choices[1].choice().unwrap();
                        match c_exclusivity.as_ref() {
                            "Yes" => exclusivity = true,
                            "No" => exclusivity = false,
                            _ => exclusivity = false,
                        };
                        logger::log(&format!("{} {} {} {} {} {} {} {}", parity, exclusivity, baud_rate, data_bits, flow_control, stop_bits, timeout, device));
                    },
                    gui::Message::BaudRate => {
                        baud_rate = device_settings_choices[2].choice().unwrap().parse::<u32>().unwrap();
                        logger::log(&format!("{} {} {} {} {} {} {} {}", parity, exclusivity, baud_rate, data_bits, flow_control, stop_bits, timeout, device));
                    },
                    gui::Message::DataBits => {
                        data_bits = device_settings_choices[3].choice().unwrap();
                        logger::log(&format!("{} {} {} {} {} {} {} {}", parity, exclusivity, baud_rate, data_bits, flow_control, stop_bits, timeout, device));
                    },
                    gui::Message::FlowControl => {
                        flow_control = device_settings_choices[4].choice().unwrap();
                        logger::log(&format!("{} {} {} {} {} {} {} {}", parity, exclusivity, baud_rate, data_bits, flow_control, stop_bits, timeout, device));
                    },
                    gui::Message::StopBits => {
                        stop_bits = device_settings_choices[5].choice().unwrap();
                        logger::log(&format!("{} {} {} {} {} {} {} {}", parity, exclusivity, baud_rate, data_bits, flow_control, stop_bits, timeout, device));
                    },
                    gui::Message::Duration => {
                        timeout = device_settings_input.value().parse::<u64>().unwrap();
                        logger::log(&format!("{} {} {} {} {} {} {} {}", parity, exclusivity, baud_rate, data_bits, flow_control, stop_bits, timeout, device));
                    },
                    gui::Message::Device => {
                        device = device_choice.choice().unwrap();
                        logger::log(&format!("{} {} {} {} {} {} {} {}", parity, exclusivity, baud_rate, data_bits, flow_control, stop_bits, timeout, device));
                    },
                    gui::Message::Read => {
                        match read_type.as_str() {
                            "One Shot" => {
                                active_read = 2;
                            },
                            "Active" => {
                                active_read = 1;
                            }
                            _ => {
                                active_read = 0;
                            },
                        }
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
                        options_window.show();
                    },
                    gui::Message::Preferences => {
                        gui::create_preferences_window(&sender);
                    },
                    gui::Message::ReadType => {
                        read_type = device_read_type.choice().unwrap();
                        logger::log(&format!("{}", read_type));
                    },
                    _ => {}
                }

            }


            if active_read == 1 {
                let con_device = match port_connection::connect_port_tty(&device , baud_rate, &parity, timeout, exclusivity, &data_bits, &flow_control, &stop_bits) {
                    Ok(dev) => {
                        port_read::read_stream_macos(dev);
                    },
                    Err(e) => {
                        logger::log_connection_error_tty(e, &device);
                    }
                };
            }

            if active_read == 2 {
                let con_device = match port_connection::connect_port_tty(&device , baud_rate, &parity, timeout, exclusivity, &data_bits, &flow_control, &stop_bits) {
                    Ok(dev) => {
                        port_read::read_stream_linux(dev);
                    },
                    Err(e) => {
                        logger::log_connection_error_tty(e, &device);
                    }
                };
                active_read = 0;
            }


            device_status.set_buffer(TextBuffer::default());
            let mut status_buffer = device_status.buffer().unwrap();

            if device.is_empty() {
                status_buffer.set_text(&"No device connected");
                device_settings_choices[0].set_value(0);
            } else {
                match active_read {
                    0 => {
                        device_status_state = "Inactive".to_string();
                        status_buffer.set_text(&gui::device_status_output(&device, &device_status_state, &read_type));
                    },
                    1 => {
                        device_status_state = "Reading".to_string();
                        status_buffer.set_text(&gui::device_status_output(&device, &device_status_state, &read_type));
                    },
                    2 => {
                        device_status_state = "Reading".to_string();
                        status_buffer.set_text(&gui::device_status_output(&device, &device_status_state, &read_type));
                    }
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
