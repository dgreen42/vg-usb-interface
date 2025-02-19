use serialport::{
    SerialPortInfo,
    available_ports,
};
use std::fmt::Debug;
use fltk::{
    app::{self, App, Receiver, Sender}, button::Button, enums::{FrameType, Shortcut, Color}, frame::Frame, group::{Flex, Grid}, input::{Input, IntInput}, menu::{self, Choice, SysMenuBar}, output::Output, prelude::*, table::Table, window::Window, text::TextDisplay
};

use fltk_theme::{
    WidgetTheme,
    ThemeType,
};

use crate::logger::{log_error, log};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Device,
    DeviceStatus,
    Parity,
    Exclusivity,
    BaudRate,
    DataBits,
    FlowControl,
    StopBits,
    Duration,
    Table,
    Read,
    Write,
    FileName,
    Stop,
    Close,
    SetDefaults,
    Preferences,
    Theme,
    ReadType,
    ApplyPreferences,
}

fn theme_choice(themes_string: &i32) -> ThemeType {
    match themes_string {
        0 => {
            log(&String::from("Theme: Classic"));
            return ThemeType::Classic;
        },
        1 => {
            log(&String::from("Theme: Aero"));
            return ThemeType::Aero;
        },
        2 => {
            log(&String::from("Theme: Metro"));
            return ThemeType::Metro;
        },
        3 => {
            log(&String::from("Theme: AquaClassic"));
            return ThemeType::AquaClassic;
        },
        4 => {
            log(&String::from("Theme: GreyBird"));
            return ThemeType::Greybird;
        },
        5 => {
            log(&String::from("Theme: Blue"));
            return ThemeType::Blue;
        },
        6 => {
            log(&String::from("Theme: Dark"));
            return ThemeType::Dark;
        },
        7 => {
            log(&String::from("Theme: HighContrast"));
            return ThemeType::HighContrast;
        },
        &_ => {
            log(&String::from("Theme: Classic"));
            return ThemeType::Classic;
        },
    }
}

pub fn create_preferences_window(send: &Sender<Message>, theme: &i32) -> ((Grid, Vec<Choice>), Window) {

    let base_theme = theme_choice(theme);
    let preferences_window = Window::new(400, 200, 500, 500, "Preferences");

    let theme = WidgetTheme::new(base_theme);
    theme.apply();

    let mut preference_grid = Grid::default().with_size(500, 500);
    preference_grid.set_layout_ext(2, 1, 1, 1);

    let mut preferences_grid_lower = Grid::default().with_size(500, 80);
    preferences_grid_lower.set_layout_ext(1, 2, 20, 10);

    let mut apply_preferences = Button::default().with_label("Apply").with_size(80, 10);
    let device_preferences_apply_result = preferences_grid_lower.set_widget(&mut apply_preferences, 0, 1);

    preferences_grid_lower.end();
    
    let device_preferences = create_device_preferences(send);
    let mut device_preferences_grid = device_preferences.0.clone();
    let device_preferences_choices_result = preference_grid.set_widget(&mut device_preferences_grid, 0, 0);
    let device_preferences_grid_lower_result = preference_grid.set_widget(&mut preferences_grid_lower, 1, 0);

    preference_grid.end();

    log_error(device_preferences_grid_lower_result, "device_preferences_grid_lower_result");
    log_error(device_preferences_choices_result, "device_preferences_choices_result");
    log_error(device_preferences_apply_result, "device_preferences_apply_result");

    preferences_window.end();

    let mut device_preferences_choices = device_preferences.1.clone();

    device_preferences_choices[0].emit(*send, Message::Theme);
    apply_preferences.emit(*send, Message::ApplyPreferences);

    return (device_preferences, preferences_window)

}

pub fn create_options_window(send: &Sender<Message>, theme: &i32) -> ((Grid, (Vec<Choice>, IntInput)), Window) {

    let base_theme = theme_choice(theme);
    let options_window = Window::new(200, 200, 500, 500, "Options");

    let device_settings = create_device_settings_options(); 

    let theme = WidgetTheme::new(base_theme);
    theme.apply();

    options_window.end();

    let mut device_setting_choices = device_settings.1.clone();

    device_setting_choices.0[0].emit(*send, Message::Parity);
    device_setting_choices.0[1].emit(*send, Message::Exclusivity);
    device_setting_choices.0[2].emit(*send, Message::BaudRate);
    device_setting_choices.0[3].emit(*send, Message::DataBits);
    device_setting_choices.0[4].emit(*send, Message::FlowControl);
    device_setting_choices.0[5].emit(*send, Message::StopBits);
    device_setting_choices.1.emit(*send, Message::Duration);

    return (device_settings, options_window)
}


pub fn create_window(theme: &i32) -> (App, Sender<Message>, Receiver<Message>, (Choice, TextDisplay, Choice), (Vec<Button>, Input), Table, Window) {

    let base_theme = theme_choice(theme);

    let app = App::default();
    let (send, recieve) = app::channel::<Message>();

    let mut main_window = Window::new(1, 1, 1200, 750, "VG Meter 200 USB Interface");
    main_window.make_resizable(true);
    let theme = WidgetTheme::new(base_theme);
    theme.apply();

    let flex_base = Flex::default()
        .with_size(1100, 700)
        .column()
        .center_of_parent();


    let mut base_grid = Grid::default().with_size(1100, 700)
        .center_of_parent();
    base_grid.set_layout_ext(3, 1, 10, 10);


    let mut sub_grid_upper = Grid::default().with_size(1000, 220)
        .center_of_parent();
    sub_grid_upper.set_layout_ext(1, 1, 10, 10);

    let mut device_settings = create_device_settings_main(); 
    let sub_upper_grid_result = sub_grid_upper.set_widget(&mut device_settings.0, 0, 0);
    log_error(sub_upper_grid_result, "sub_grid_upper_grid_result");

    sub_grid_upper.end();

    let mut sub_grid_bottom = Grid::default().with_size(1000, 470)
        .center_of_parent();
    sub_grid_bottom.set_layout_ext(1, 2, 10, 10);

    let mut read_write = create_read_write();
    let mut data_table = create_data_table();
    let read_write_grid_result = sub_grid_bottom.set_widget(&mut read_write.0, 0, 0);
    log_error(read_write_grid_result, "sub_grid_botom_read_write_grid_result");
    let data_table_grid_result = sub_grid_bottom.set_widget(&mut data_table.0, 0, 1);
    log_error(data_table_grid_result, "sub_grid_bottom_data_table_grid_result");

    sub_grid_bottom.end();

    let base_grid_upper_grid_result = base_grid.set_widget(&mut sub_grid_upper, 1, 0);
    log_error(base_grid_upper_grid_result, "base_grid_upper_grid_result");
    let base_grid_bottom_grid_result = base_grid.set_widget(&mut sub_grid_bottom, 2, 0);
    log_error(base_grid_bottom_grid_result, "base_grid_bottom_grid_result");

    base_grid.end();

    flex_base.end();

    let flex_menu = Flex::default()
        .with_size(1200, 30)
        .top_window();

    let mut menu_grid = Grid::default().with_size(100, 30);
    menu_grid.set_layout_ext(1, 1, 1, 1);

    let mut menu = create_menu(&send);
    let menu_grid_result = menu_grid.set_widget(&mut menu, 0, 0);
    log_error(menu_grid_result, "menu_grid_result");

    menu_grid.end();

    flex_menu.unwrap().end();

    main_window.end();

    let mut device_setting_choice = device_settings.1;
    let mut device_status_output = device_settings.2;
    let mut device_read_type = device_settings.3;

    device_setting_choice.emit(send, Message::Device);
    device_status_output.emit(send, Message::DeviceStatus);
    device_read_type.emit(send, Message::ReadType);

    let mut read_write_buttons = read_write.1;

    read_write_buttons.0[0].emit(send, Message::Read);
    read_write_buttons.0[1].emit(send, Message::Write);
    read_write_buttons.0[2].emit(send, Message::Stop);
    read_write_buttons.0[3].emit(send, Message::Close);
    read_write_buttons.1.emit(send, Message::FileName);

    let mut data_table_table = data_table.1;

    data_table_table.emit(send, Message::Table);

    return (app, send, recieve, (device_setting_choice, device_status_output, device_read_type), read_write_buttons, data_table_table, main_window)
}

fn create_read_write() -> (Grid, (Vec<Button>, Input))  {

    let mut read_write_grid = Grid::default().with_size(300, 450);
    read_write_grid.set_layout_ext(6, 2, 5, 5);
    let mut b_read = Button::default().with_label("Read Data").with_size(80, 10);
    let mut b_write = Button::default().with_label("Write Data").with_size(80, 10);
    let mut b_stop = Button::default().with_label("Stop Reading").with_size(80, 10);
    let mut b_close = Button::default().with_label("Close Connection").with_size(80, 10);
    let mut l_file_name = Frame::default().with_label("File Name");
    let mut i_file_name = Input::default().with_size(80, 5);

    let b_read_grid_result = read_write_grid.set_widget(&mut b_read, 1, 0);
    let b_stop_grid_result = read_write_grid.set_widget(&mut b_stop, 2, 0);
    let b_close_grid_result = read_write_grid.set_widget(&mut b_close, 3, 0);
    let l_file_name_grid_result = read_write_grid.set_widget(&mut l_file_name, 3, 1);
    let i_file_name_grid_result = read_write_grid.set_widget(&mut i_file_name, 4, 1);
    let b_write_grid_result = read_write_grid.set_widget(&mut b_write, 4, 0);

    let read_write_end_result = read_write_grid.end();

    log_error(b_read_grid_result, "b_read_grid_result");
    log_error(b_stop_grid_result, "b_stop_grid_result");
    log_error(b_close_grid_result, "b_close_grid_result");
    log_error(l_file_name_grid_result, "l_file_name_grid_result");
    log_error(i_file_name_grid_result, "i_file_name_grid_result");
    log_error(b_write_grid_result, "b_write_grid_result");
    log_error(Ok(read_write_end_result), "read_write_end_result");

    let b_vec = vec![b_read, b_write, b_stop, b_close];

    return (read_write_grid, (b_vec, i_file_name))
}

fn create_data_table() -> (Grid, Table) {
    let mut data_table_grid = Grid::default() .with_size(400, 450);
    data_table_grid.set_layout_ext(1, 1, 5, 5);
    let mut table = Table::default();
    table.make_resizable(true);
    table.end();
    let table_grid_result = data_table_grid.set_widget(&mut table, 0, 0);
    log_error(table_grid_result, "table_grid_result");
    data_table_grid.end();


    return (data_table_grid, table)
}

pub fn avail_ports() -> Vec<SerialPortInfo> {
    let mut ports: Vec<SerialPortInfo> = Vec::new();
    let avail_ports = available_ports();
    match avail_ports {
        Ok(s) => ports = s,
        Err(e) => eprintln!("No ports available, {}", e),
    };

    return ports
}

fn create_device_settings_main() -> (Grid, Choice, TextDisplay, Choice) {

    let mut device_grid = Grid::default().with_size(1000, 190);
    device_grid.set_layout_ext(1, 2, 5, 5);

    let mut device_grid_left = Grid::default().with_size(500, 200);
    device_grid_left.set_layout_ext(2, 1, 5, 5);

    let mut device_grid_left_upper = Grid::default().with_size(200, 50);
    device_grid_left_upper.set_layout_ext(2, 1, 1, 1);


    let ports = avail_ports();

    let mut l_device = Frame::default().with_label("Device").with_size(50, 50);
    let mut c_device = Choice::default().with_size(50, 50);
    c_device.add_choice("None");
    for port in ports {
        c_device.add_choice(&port.port_name);
    }
    let device_grid_left_result = device_grid.set_widget(&mut device_grid_left, 0, 0);
    let device_grid_left_upper_result = device_grid_left.set_widget(&mut device_grid_left_upper, 0, 0);
    let l_device_grid_left_upper_result = device_grid_left_upper.set_widget(&mut l_device, 0, 0);
    let c_device_grid_left_upper_result = device_grid_left_upper.set_widget(&mut c_device, 1, 0);

    device_grid_left_upper.end();

    let mut device_grid_left_lower = Grid::default().with_size(200, 50);
    device_grid_left_lower.set_layout_ext(2, 1, 1, 1);

    let mut l_read_type = Frame::default().with_label("Read Type").with_size(50, 50);
    let mut c_read_type = Choice::default().with_size(50, 50);
    c_read_type.add_choice("One Shot");
    c_read_type.add_choice("Active");
    let device_grid_left_lower_result = device_grid_left.set_widget(&mut device_grid_left_lower, 1, 0);
    let l_read_type_left_lower_result = device_grid_left_lower.set_widget(&mut l_read_type, 0, 0);
    let c_read_type_left_lower_result = device_grid_left_lower.set_widget(&mut c_read_type, 1, 0);

    device_grid_left_lower.end();
    device_grid_left.end();

    let mut o_device_status = TextDisplay::default().with_size(400, 200);

    //left side
    //right side
    let o_device_grid_status_result = device_grid.set_widget(&mut o_device_status, 0, 1);

    log_error(device_grid_left_result, "device_grid_left_result");
    log_error(device_grid_left_upper_result, "device_grid_left_upper_result");
    log_error(l_device_grid_left_upper_result, "l_device_grid_left_upper_result");
    log_error(c_device_grid_left_upper_result, "c_device_grid_left_upper_result");
    log_error(o_device_grid_status_result, "o_device_grid_status_result");
    log_error(device_grid_left_lower_result, "device_grid_left_lower_result");
    log_error(l_read_type_left_lower_result, "l_read_type_left_lower_result");
    log_error(c_read_type_left_lower_result, "c_read_type_left_lower_result");

    device_grid.end();

    return (device_grid, c_device, o_device_status, c_read_type)
}

fn create_device_settings_options() -> (Grid, (Vec<Choice>, IntInput)) {

    let mut device_grid = Grid::default().with_size(500, 500);
    device_grid.set_layout_ext(7, 2, 10, 10);

    let mut l_parity = Frame::default().with_label("Parity");
    let mut c_parity = Choice::default().with_size(20, 10);
    c_parity.add_choice("None");
    c_parity.add_choice("Odd");
    c_parity.add_choice("Even");

    let mut l_exlusivity = Frame::default().with_label("Exclusivity");
    let mut c_exlusivity = Choice::default().with_size(20, 10);
    c_exlusivity.add_choice("Yes");
    c_exlusivity.add_choice("No");

    let mut l_baud_rate = Frame::default().with_label("Baud Rate");
    let mut c_baud_rate = Choice::default().with_size(20, 10);
    c_baud_rate.add_choice("9600");
    c_baud_rate.add_choice("0");
    c_baud_rate.add_choice("50");
    c_baud_rate.add_choice("75");
    c_baud_rate.add_choice("110");
    c_baud_rate.add_choice("134");
    c_baud_rate.add_choice("150");
    c_baud_rate.add_choice("200");
    c_baud_rate.add_choice("300");
    c_baud_rate.add_choice("600");
    c_baud_rate.add_choice("1200");
    c_baud_rate.add_choice("1800");
    c_baud_rate.add_choice("2400");
    c_baud_rate.add_choice("4800");
    c_baud_rate.add_choice("19200");
    c_baud_rate.add_choice("38400");
    //lower half

    let mut l_data_bits = Frame::default().with_label("Data Bits");
    let mut c_data_bits = Choice::default().with_size(20, 10);
    c_data_bits.add_choice("5");
    c_data_bits.add_choice("6");
    c_data_bits.add_choice("7");
    c_data_bits.add_choice("8");

    let mut l_flow_control = Frame::default().with_label("Flow Control");
    let mut c_flow_control = Choice::default().with_size(20, 10);
    c_flow_control.add_choice("None");
    c_flow_control.add_choice("Software");
    c_flow_control.add_choice("Hardware");

    let mut l_stop_bits = Frame::default().with_label("Stop Bits");
    let mut c_stop_bits = Choice::default().with_size(20, 10);
    c_stop_bits.add_choice("1");
    c_stop_bits.add_choice("2");

    let mut l_duration = Frame::default().with_label("Duration");
    let mut s_duration = IntInput::default().with_size(20, 10);

    let l_parity_grid_result = device_grid.set_widget(&mut l_parity, 0, 0);
    let c_parity_grid_result = device_grid.set_widget(&mut c_parity, 0, 1);
    let l_exlusivity_grid_result = device_grid.set_widget(&mut l_exlusivity, 1, 0);
    let c_exlusivity_grid_result = device_grid.set_widget(&mut c_exlusivity, 1, 1);
    let l_baud_rate_grid_result = device_grid.set_widget(&mut l_baud_rate, 2, 0);
    let c_baud_rate_grid_result = device_grid.set_widget(&mut c_baud_rate, 2, 1);
    let l_data_bits_grid_result = device_grid.set_widget(&mut l_data_bits, 3, 0);
    let c_data_bits_grid_result = device_grid.set_widget(&mut c_data_bits, 3, 1);
    let l_flow_control_grid_result = device_grid.set_widget(&mut l_flow_control, 4, 0);
    let c_flow_control_grid_result = device_grid.set_widget(&mut c_flow_control, 4, 1);
    let l_stop_bits_grid_result = device_grid.set_widget(&mut l_stop_bits, 5, 0);
    let c_stop_bits_grid_result = device_grid.set_widget(&mut c_stop_bits, 5, 1);
    let l_duration_grid_result = device_grid.set_widget(&mut l_duration, 6, 0);
    let s_duration_grid_result = device_grid.set_widget(&mut s_duration, 6, 1);

    device_grid.end();

    log_error(l_parity_grid_result, "l_parity_grid_result");
    log_error(c_parity_grid_result, "c_parity_grid_result");
    log_error(l_exlusivity_grid_result, "l_exlusivity_grid_result");
    log_error(c_exlusivity_grid_result, "c_exlusivity_grid_result");
    log_error(l_baud_rate_grid_result, "l_baud_rate_grid_result");
    log_error(c_baud_rate_grid_result, "c_baud_rate_grid_result");
    log_error(l_data_bits_grid_result, "l_data_bits_grid_result");
    log_error(c_data_bits_grid_result, "c_data_bits_grid_result");
    log_error(l_flow_control_grid_result, "l_flow_control_grid_result");
    log_error(c_flow_control_grid_result, "c_flow_control_grid_result");
    log_error(l_stop_bits_grid_result, "l_stop_bits_grid_result");
    log_error(c_stop_bits_grid_result, "c_stop_bits_grid_result");
    log_error(l_duration_grid_result, "l_duration_grid_result");
    log_error(s_duration_grid_result, "s_duration_grid_result");


    let c_vec = vec![c_parity, c_exlusivity, c_baud_rate, c_data_bits, c_flow_control, c_stop_bits];
    let pieces = (c_vec, s_duration);

    return (device_grid, pieces)
}

fn create_menu(send: &Sender<Message>) -> SysMenuBar {

    let mut menu = SysMenuBar::default().with_size(1200, 30);
    menu.set_frame(FrameType::FlatBox);
    menu.set_color(Color::Light1);
    menu.add_emit("File/Set Defaults", Shortcut::Ctrl | 'd', menu::MenuFlag::Normal, *send, Message::SetDefaults);
    menu.add_emit("File/Preferences", Shortcut::Ctrl | 'p', menu::MenuFlag::Normal, *send, Message::Preferences);

    return menu
}

fn create_device_preferences(send: &Sender<Message>) -> (Grid, Vec<Choice>) {

    let mut preferences_grid = Grid::default().with_size(500, 400);
    preferences_grid.set_layout_ext(4, 2, 10, 10);

    let mut l_themes = Frame::default().with_label("Themes");
    let mut c_themes = Choice::default().with_size(80, 30);
    c_themes.add_emit("Classic", Shortcut::None, menu::MenuFlag::Normal, *send, Message::Theme);
    c_themes.add_emit("Areo", Shortcut::None, menu::MenuFlag::Normal, *send, Message::Theme);
    c_themes.add_emit("Metro", Shortcut::None, menu::MenuFlag::Normal, *send, Message::Theme);
    c_themes.add_emit("AquaClassic", Shortcut::None, menu::MenuFlag::Normal, *send, Message::Theme);
    c_themes.add_emit("GreyBird", Shortcut::None, menu::MenuFlag::Normal, *send, Message::Theme);
    c_themes.add_emit("Blue", Shortcut::None, menu::MenuFlag::Normal, *send, Message::Theme);
    c_themes.add_emit("Dark", Shortcut::None, menu::MenuFlag::Normal, *send, Message::Theme);
    c_themes.add_emit("HighContrast", Shortcut::None, menu::MenuFlag::Normal, *send, Message::Theme);
    let mut l_p1 = Frame::default().with_label("Place Holder 1");
    let mut c_p1 = Choice::default().with_size(80, 30);
    let mut l_p2 = Frame::default().with_label("Place Holder 2");
    let mut c_p2 = Choice::default().with_size(80, 30);
    let mut l_p3 = Frame::default().with_label("Place Holder 3");
    let mut c_p3 = Choice::default().with_size(80, 30);

    let l_themes_preferences_grid_result = preferences_grid.set_widget(&mut l_themes, 0, 0);
    let c_themes_preferences_grid_result = preferences_grid.set_widget(&mut c_themes, 0, 1);
    let l_themes_preferences_grid_result1 = preferences_grid.set_widget(&mut l_p1, 1, 0);
    let c_themes_preferences_grid_result1 = preferences_grid.set_widget(&mut c_p1, 1, 1);
    let l_themes_preferences_grid_result2 = preferences_grid.set_widget(&mut l_p2, 2, 0);
    let c_themes_preferences_grid_result2 = preferences_grid.set_widget(&mut c_p2, 2, 1);
    let l_themes_preferences_grid_result3 = preferences_grid.set_widget(&mut l_p3, 3, 0);
    let c_themes_preferences_grid_result3 = preferences_grid.set_widget(&mut c_p3, 3, 1);

    preferences_grid.end();

    log_error(l_themes_preferences_grid_result, "l_themes_preferences_grid_result");
    log_error(c_themes_preferences_grid_result, "c_themes_preferences_grid_result");

    let preference_choices = vec![c_themes, c_p1, c_p2, c_p3];

    return (preferences_grid, preference_choices)
}

pub fn device_status_output(device: &str, device_status: &str, read_type: &str) -> String {
    return format!("Device: {}\nDevice Status: {}\nRead Type: {}\n", device, device_status, read_type);
}
