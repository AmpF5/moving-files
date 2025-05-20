use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph},
};

use crate::{app::App, ui::popup::render_popup, utils::fs::match_file_extension};

use super::style::{SELECTED_ROW_BG_COLOR, alternate_colors, get_extension_color};

pub fn render_main_windows(app: &mut App, frame: &mut Frame) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(1), Min(0), Length(3)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());

    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [left_area, right_area] = horizontal.areas(main_area);

    render_widget(title_area, frame);

    render_to_list(app, right_area, frame);

    render_from_list(app, left_area, frame);

    render_footer(status_area, frame);

    if app.show_popup {
        render_popup(app, frame);
    }
}

fn render_footer(area: Rect, frame: &mut Frame) {
    frame.render_widget(Paragraph::new("[↓↑]: Navigate files | [␣]: Select/Unselect file | [f]: Open source folder | [Ctrl+f]: Open destination folder | [s] Swap folders | [q]: Quit")
            .centered()
            .block(Block::bordered().title("Available Commands")), 
            area);
}

fn render_widget(area: Rect, frame: &mut Frame) {
    frame.render_widget(
        Block::bordered()
            .title("Blazingly fast moving files")
            .title_alignment(Alignment::Center),
        area,
    );
}

fn render_to_list(app: &mut App, area: Rect, frame: &mut Frame) {
    let items: Vec<ListItem> = app
        .files_to
        .items
        .iter()
        .enumerate()
        .map(|(i, file)| {
            let bg_color = alternate_colors(i);

            let fg_color = get_extension_color(match_file_extension(file.extension.as_str()));

            let styled_file = Line::from(vec![
                Span::styled(file.name.clone(), Style::default()),
                Span::styled(
                    format!(".{}", file.extension.clone()),
                    Style::new().fg(fg_color),
                ),
            ])
            .bg(bg_color);

            ListItem::from(styled_file)
        })
        .collect();

    let files_to_list = List::new(items)
        .block(Block::bordered().title(format!("Import to {}", app.files_to.path)))
        .style(Style::new().white());

    frame.render_stateful_widget(files_to_list, area, &mut app.files_to.state);
}

fn render_from_list(app: &mut App, area: Rect, frame: &mut Frame) {
    let items: Vec<ListItem> = app
        .files_from
        .items
        .iter()
        .enumerate()
        .map(|(i, file)| {
            let bg_color = alternate_colors(i);

            let fg_color = get_extension_color(match_file_extension(file.extension.as_str()));

            let mut styled_file = Line::from(vec![
                Span::styled(file.name.clone(), Style::default()),
                Span::styled(
                    format!(".{}", file.extension.clone()),
                    Style::new().fg(fg_color),
                ),
            ])
            .bg(bg_color);

            if file.is_selected {
                styled_file = styled_file.clone().style(
                    Style::new()
                        .bg(SELECTED_ROW_BG_COLOR)
                        .add_modifier(Modifier::BOLD),
                );
            }
            ListItem::from(styled_file)
        })
        .collect();

    let files_from_list = List::new(items)
        .block(Block::bordered().title(format!("Import from {}", app.files_from.path)))
        .style(Style::new().white())
        .highlight_style(Style::new().italic())
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(files_from_list, area, &mut app.files_from.state);
}
