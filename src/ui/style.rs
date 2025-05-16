use ratatui::style::{Color, palette::tailwind::SLATE};

use crate::models::file::FileExtension;

pub const SELECTED_ROW_BG_COLOR: Color = SLATE.c500;
pub const NORMAL_ROW_BG_COLOR: Color = SLATE.c950;
pub const ALT_ROW_BG_COLOR: Color = SLATE.c900;

pub fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG_COLOR
    } else {
        ALT_ROW_BG_COLOR
    }
}

pub fn get_extension_color(file_extension: FileExtension) -> Color {
    match file_extension {
        FileExtension::Jpg => Color::Rgb(0, 204, 255),
        FileExtension::Webp => Color::Rgb(255, 255, 153),
        FileExtension::Png => Color::Rgb(204, 153, 255),
        FileExtension::Txt => Color::Rgb(255, 204, 153),
        FileExtension::NotImplemented => Color::Rgb(255, 255, 255),
    }
}
