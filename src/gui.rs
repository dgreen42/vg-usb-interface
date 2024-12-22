use serialport::{
    SerialPortInfo,
    available_ports,
};
use std::fmt::Debug;
use fltk::{
    app::{self, App, Receiver, Sender}, button::Button, enums::{FrameType, Shortcut, Color}, frame::Frame, group::{Flex, Grid}, input::{Input, IntInput}, menu::{self, Choice, SysMenuBar}, output::Output, prelude::*, table::Table, window::Window
};

use fltk_theme::{
    WidgetTheme,
    ThemeType,
};

use crate::logger::log_error;

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
}

pub fn create_preferences_window(send: &Sender<Message>) {

    let base_theme = ThemeType::Dark;
    let mut preferences_window = Window::new(400, 200, 500, 500, "Preferences");

    let app_preferences = create_app_preferences(send);
    app_preferences.end();

    let theme = WidgetTheme::new(base_theme);
    theme.apply();

    preferences_window.show();
}

pub fn create_options_window() {

    let base_theme = ThemeType::Dark;
    let mut options_window = Window::new(200, 200, 500, 500, "Options");

    let device_settings = create_device_settings(); 
    device_settings.0.end();

    let theme = WidgetTheme::new(base_theme);
    theme.apply();

    options_window.end();
    options_window.show();

}


pub fn create_window() -> (App, Receiver<Message>, (Vec<Choice>,  IntInput), (Vec<Button>, Input, Output), Table, Sender<Message>) {

    let base_theme = ThemeType::Dark;

    let app = App::default();
    let (send, recieve) = app::channel::<Message>();

    let mut main_window = Window::new(200, 200, 1200, 750, "VG Meter 200 USB Interface");
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


    let mut sub_grid_upper = Grid::default().with_size(1000, 200)
        .center_of_parent();
    sub_grid_upper.set_layout_ext(1, 1, 10, 10);

    let mut device_settings = create_device_settings(); 
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

    /*
    let base_grid_menu_grid_result = base_grid.set_widget(&mut menu_grid, 0, 0);
    log_error(base_grid_menu_grid_result, "base_grid_menu_grid_result");
    */

    let base_grid_upper_grid_result = base_grid.set_widget(&mut sub_grid_upper, 1, 0);
    log_error(base_grid_upper_grid_result, "base_grid_upper_grid_result");
    //failed
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
    main_window.show();


    let mut device_setting_choices = device_settings.1;

    device_setting_choices.0[0].emit(send, Message::Device);
    device_setting_choices.0[1].emit(send, Message::Parity);
    device_setting_choices.0[2].emit(send, Message::Exclusivity);
    device_setting_choices.0[3].emit(send, Message::BaudRate);
    device_setting_choices.0[4].emit(send, Message::DataBits);
    device_setting_choices.0[5].emit(send, Message::FlowControl);
    device_setting_choices.0[6].emit(send, Message::StopBits);
    device_setting_choices.1.emit(send, Message::Duration);

    let mut read_write_buttons = read_write.1;

    read_write_buttons.0[0].emit(send, Message::Read);
    read_write_buttons.0[1].emit(send, Message::Write);
    read_write_buttons.0[2].emit(send, Message::Stop);
    read_write_buttons.0[3].emit(send, Message::Close);
    read_write_buttons.1.emit(send, Message::FileName);
    read_write_buttons.2.emit(send, Message::DeviceStatus);

    let mut data_table_table = data_table.1;

    data_table_table.emit(send, Message::Table);


    return (app, recieve, device_setting_choices, read_write_buttons, data_table_table, send)
}

fn create_read_write() -> (Grid, (Vec<Button>, Input, Output))  {

    let mut read_write_grid = Grid::default().with_size(300, 450);
    read_write_grid.set_layout_ext(6, 2, 5, 5);
    let mut b_read = Button::default().with_label("Read Data").with_size(80, 10);
    let mut b_write = Button::default().with_label("Write Data").with_size(80, 10);
    let mut b_stop = Button::default().with_label("Stop Reading").with_size(80, 10);
    let mut b_close = Button::default().with_label("Close Connection").with_size(80, 10);
    let mut l_file_name = Frame::default().with_label("File Name");
    let mut i_file_name = Input::default().with_size(80, 5);
    let mut l_device_status = Frame::default().with_label("Device Status");
    let mut o_device_status = Output::default().with_size(80, 10);
    let b_read_grid_result = read_write_grid.set_widget(&mut b_read, 1, 0);
    log_error(b_read_grid_result, "b_read_grid_result");
    let b_stop_grid_result = read_write_grid.set_widget(&mut b_stop, 2, 0);
    log_error(b_stop_grid_result, "b_stop_grid_result");
    let b_close_grid_result = read_write_grid.set_widget(&mut b_close, 3, 0);
    log_error(b_close_grid_result, "b_close_grid_result");
    let l_device_status_result = read_write_grid.set_widget(&mut l_device_status, 1, 1);
    log_error(l_device_status_result, "l_device_status_result ");
    let o_device_status_result = read_write_grid.set_widget(&mut o_device_status, 2, 1);
    log_error(o_device_status_result , "o_device_status_result");
    let l_file_name_grid_result = read_write_grid.set_widget(&mut l_file_name, 3, 1);
    log_error(l_file_name_grid_result, "l_file_name_grid_result");
    let i_file_name_grid_result = read_write_grid.set_widget(&mut i_file_name, 4, 1);
    log_error(i_file_name_grid_result, "i_file_name_grid_result");
    let b_write_grid_result = read_write_grid.set_widget(&mut b_write, 4, 0);
    log_error(b_write_grid_result, "b_write_grid_result");
    let read_write_end_result = read_write_grid.end();
    log_error(Ok(read_write_end_result), "read_write_end_result");

    let b_vec = vec![b_read, b_write, b_stop, b_close];


    return (read_write_grid, (b_vec, i_file_name, o_device_status))
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

fn avail_ports() -> Vec<SerialPortInfo> {
    let mut ports: Vec<SerialPortInfo> = Vec::new();
    let avail_ports = available_ports();
    match avail_ports {
        Ok(s) => ports = s,
        Err(e) => eprintln!("No ports available, {}", e),
    };

    return ports
}

fn create_device_settings() -> (Grid, (Vec<Choice>, IntInput)) {

    let mut device_grid = Grid::default().with_size(500, 100);
    device_grid.set_layout_ext(4, 4, 5, 5);

    let ports = avail_ports();

    let mut l_device = Frame::default().with_label("Device");
    let mut c_device = Choice::default().with_size(20, 10);
    c_device.add_choice("None");
    for port in ports {
        c_device.add_choice(&port.port_name);
    }
    let l_device_grid_result = device_grid.set_widget(&mut l_device, 0, 0);
    log_error(l_device_grid_result, "l_device_grid_result");
    let c_device_grid_result = device_grid.set_widget(&mut c_device, 1, 0);
    log_error(c_device_grid_result, "c_device_grid_result");


    let mut l_parity = Frame::default().with_label("Parity");
    let mut c_parity = Choice::default().with_size(20, 10);
    c_parity.add_choice("None");
    c_parity.add_choice("Odd");
    c_parity.add_choice("Even");
    let l_parity_grid_result = device_grid.set_widget(&mut l_parity, 0, 1);
    log_error(l_parity_grid_result, "l_parity_grid_result");
    let c_parity_grid_result = device_grid.set_widget(&mut c_parity, 1, 1);
    log_error(c_parity_grid_result, "c_parity_grid_result");

    let mut l_exlusivity = Frame::default().with_label("Exclusivity");
    let mut c_exlusivity = Choice::default().with_size(20, 10);
    c_exlusivity.add_choice("Yes");
    c_exlusivity.add_choice("No");
    let l_exlusivity_grid_result = device_grid.set_widget(&mut l_exlusivity, 0, 2);
    log_error(l_exlusivity_grid_result, "l_exlusivity_grid_result");
    let c_exlusivity_grid_result = device_grid.set_widget(&mut c_exlusivity, 1, 2);
    log_error(c_exlusivity_grid_result, "c_exlusivity_grid_result");

    let mut l_baud_rate = Frame::default().with_label("Baud Rate");
    let mut c_baud_rate = Choice::default().with_size(20, 10);
    c_baud_rate.add_choice("9600");
    let l_baud_rate_grid_result = device_grid.set_widget(&mut l_baud_rate, 0, 3);
    log_error(l_baud_rate_grid_result, "l_baud_rate_grid_result");
    let c_baud_rate_grid_result = device_grid.set_widget(&mut c_baud_rate, 1, 3);
    log_error(c_baud_rate_grid_result, "c_baud_rate_grid_result");

    //lower half

    let mut l_data_bits = Frame::default().with_label("Data Bits");
    let mut c_data_bits = Choice::default().with_size(20, 10);
    c_data_bits.add_choice("5");
    c_data_bits.add_choice("6");
    c_data_bits.add_choice("7");
    c_data_bits.add_choice("8");
    let l_data_bits_grid_result = device_grid.set_widget(&mut l_data_bits, 2, 0);
    log_error(l_data_bits_grid_result, "l_data_bits_grid_result");
    let c_data_bits_grid_result = device_grid.set_widget(&mut c_data_bits, 3, 0);
    log_error(c_data_bits_grid_result, "c_data_bits_grid_result");

    let mut l_flow_control = Frame::default().with_label("Flow Control");
    let mut c_flow_control = Choice::default().with_size(20, 10);
    c_flow_control.add_choice("None");
    c_flow_control.add_choice("Software");
    c_flow_control.add_choice("Hardware");
    let l_flow_control_grid_result = device_grid.set_widget(&mut l_flow_control, 2, 1);
    log_error(l_flow_control_grid_result, "l_flow_control_grid_result");
    let c_flow_control_grid_result = device_grid.set_widget(&mut c_flow_control, 3, 1);
    log_error(c_flow_control_grid_result, "c_flow_control_grid_result");

    let mut l_stop_bits = Frame::default().with_label("Stop Bits");
    let mut c_stop_bits = Choice::default().with_size(20, 10);
    c_stop_bits.add_choice("1");
    c_stop_bits.add_choice("2");
    let l_stop_bits_grid_result = device_grid.set_widget(&mut l_stop_bits, 2, 2);
    log_error(l_stop_bits_grid_result, "l_stop_bits_grid_result");
    let c_stop_bits_grid_result = device_grid.set_widget(&mut c_stop_bits, 3, 2);
    log_error(c_stop_bits_grid_result, "c_stop_bits_grid_result");

    let mut l_duration = Frame::default().with_label("Duration");
    let mut s_duration = IntInput::default().with_size(20, 10);
    let l_duration_grid_result = device_grid.set_widget(&mut l_duration, 2, 3);
    log_error(l_duration_grid_result, "l_duration_grid_result");
    let s_duration_grid_result = device_grid.set_widget(&mut s_duration, 3, 3);
    log_error(s_duration_grid_result, "s_duration_grid_result");

    device_grid.end();

    let c_vec = vec![c_device, c_parity, c_exlusivity, c_baud_rate, c_data_bits, c_flow_control, c_stop_bits];
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

fn create_app_preferences(send: &Sender<Message>) -> Grid {

    let mut preferences_grid = Grid::default().with_size(500, 500);
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
    let mut l_themes1 = Frame::default().with_label("Themes");
    let mut c_themes1 = Choice::default().with_size(80, 30);
    let mut l_themes2 = Frame::default().with_label("Themes");
    let mut c_themes2 = Choice::default().with_size(80, 30);
    let mut l_themes3 = Frame::default().with_label("Themes");
    let mut c_themes3 = Choice::default().with_size(80, 30);

    let l_themes_preferences_grid_result = preferences_grid.set_widget(&mut l_themes, 0, 0);
    log_error(l_themes_preferences_grid_result, "l_themes_preferences_grid_result");
    let c_themes_preferences_grid_result = preferences_grid.set_widget(&mut c_themes, 0, 1);
    log_error(c_themes_preferences_grid_result, "c_themes_preferences_grid_result");
    let l_themes_preferences_grid_result1 = preferences_grid.set_widget(&mut l_themes1, 1, 0);
    let c_themes_preferences_grid_result1 = preferences_grid.set_widget(&mut c_themes1, 1, 1);
    let l_themes_preferences_grid_result2 = preferences_grid.set_widget(&mut l_themes2, 2, 0);
    let c_themes_preferences_grid_result2 = preferences_grid.set_widget(&mut c_themes2, 2, 1);
    let l_themes_preferences_grid_result3 = preferences_grid.set_widget(&mut l_themes3, 3, 0);
    let c_themes_preferences_grid_result3 = preferences_grid.set_widget(&mut c_themes3, 3, 1);

    return preferences_grid 
}

