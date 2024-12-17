use fltk::{
    prelude::*,
    draw,
    enums,
    table,
    table::Table
};


pub fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(
        enums::FrameType::ThinUpBox,
        x,
        y,
        w,
        h,
        enums::Color::FrameDefault,
    );
    draw::set_draw_color(enums::Color::Black);
    draw::set_font(enums::Font::Helvetica, 14);
    draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
    draw::pop_clip();
}

pub fn draw_data(txt: &str, x: i32, y: i32, w: i32, h: i32, selected: bool) {
    draw::push_clip(x, y, w, h);
    if selected {
        draw::set_draw_color(enums::Color::from_u32(0x00D3_D3D3));
    } else {
        draw::set_draw_color(enums::Color::White);
    }
    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(enums::Color::Gray0);
    draw::set_font(enums::Font::Helvetica, 14);
    draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
    draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}

pub fn draw_table(data: Vec<String>, mut table: Table, headers: Vec<String>) {

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
                        draw_header(&format!("{}", headers[col as usize]), x, y, w, h)
                    }
                    table::TableContext::RowHeader => {
                        draw_header(&format!("{}", row + 1), x, y, w, h)
                    }
                    table::TableContext::Cell => {
                        draw_data(
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
