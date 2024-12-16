pub mod start_gui_linux {
    use fltk::{
        prelude::*,
        table,
        enums,
        draw,
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
        let gui_comp = gui::create_window();
        let app = gui_comp.0;
        let reciever = gui_comp.1;
        let device_settings_choices = gui_comp.2.0;
        let device_settings_input = gui_comp.2.1;
        let _read_write_buttons = gui_comp.3.0;
        let read_write_input = gui_comp.3.1;
        let mut table = gui_comp.4;

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

        let mut file_name = String::new();

        let log_file_result = match logger::create_log() {
            Some(file) => file,
            None => panic!("Failed to create temp file"),
        };
        let temp_dir = Path::new("./temp");
        let temp_path = Path::new("./temp/temp_data.csv");
        if temp_path.is_file() {
            let remove = remove_file(temp_path);
            match remove {
                Ok(s) => println!("Temp file removed: {:?}", s),
                Err(e) => eprintln!("Failed to remove temp file: {}", e),
            }
        }
        if !temp_dir.is_dir() {
            let dir = create_dir(temp_dir);
            match dir {
                Ok(s) => logger::log(&format!("Created temp dir {:?}", s)),
                Err(e) => logger::log(&format!("Failed to create temp dir {:?}", e)),
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
                println!("{:?}", message);
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
                    gui::Message::Close => {},
                    gui::Message::FileName => {
                        file_name = read_write_input.value();
                        println!("{}", file_name);
                    },
                    gui::Message::Write => {
                        let file_path = Path::new(&file_name);
                        read_write_utils::write_file(file_path, temp_path);
                    },
                    _ => {}
                }

                if active_read == 1 {
                    let device = match port_connection::connect_port_tty(&device , baud_rate, &parity, timeout, exclusivity, &data_bits, &flow_control, &stop_bits) {
                        Some(dev) => dev, 
                        None => panic!("Failed to connect to device"),
                    };
                    port_read::read_stream(device);
                }

                data = read_write_utils::read_temp(temp_path);
                // change unit to the units that are selected from the device
                let headers = vec!["Moisture".to_string(), "unit".to_string(), "Light".to_string(), "unit".to_string(), "Temp".to_string(), "unit".to_string(), "Time".to_string()];
                let mut data_matrix: Vec<Vec<String>> = Vec::new();
                data_matrix.push(headers.clone());
                for record in data.clone() {
                    let record_split: Vec<String> = record.split(",").map(|s| s.to_string()).collect();
                    data_matrix.push(record_split);
                }
                table.set_rows(data_matrix.len() as i32);
                table.set_cols(7);
                table.draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
                    table::TableContext::StartPage => {
                        draw::set_font(enums::Font::Helvetica, 14)
                    }
                    table::TableContext::ColHeader => {
                        table_functions::draw_header(&format!("{}", headers[col as usize]), x, y, w, h)
                    }
                    table::TableContext::RowHeader => {
                        table_functions::draw_header(&format!("{}", row + 1), x, y, w, h)
                    }
                    table::TableContext::Cell => {
                        table_functions::draw_data(
                            &format!("{:?}", data_matrix[row as usize][col as usize]),
                            x,
                            y,
                            w,
                            h,
                            t.is_selected(row, col),
                        );
                        //println!("{} {} {} {}", x, y, w, h);
                    }
                    _ => (),
                });

            }

        }

        app.run().unwrap();
    }
}