use std::{fs, path::Path};

use color_eyre::{Result, eyre::Ok};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Modifier, Stylize, palette::tailwind::SLATE},
    text::Line,
    widgets::{Block, List, Paragraph},
};
use ratatui::{
    layout::{Alignment, Flex, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Clear, ListItem, ListState, Wrap},
};
use rfd::FileDialog;

// static SELECTED_STYLE: Style = Style::new().bg(SLATE.c500).add_modifier(Modifier::BOLD);

const SELECTED_ROW_BG_COLOR: Color = SLATE.c500;
const NORMAL_ROW_BG_COLOR: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;

enum FileExtension {
    Webp,
    Png,
    Jpg,
    Txt,
    NotImplemented,
}

fn get_extension_color(file_extension: FileExtension) -> Color {
    match file_extension {
        FileExtension::Jpg => Color::Rgb(0, 204, 255),
        FileExtension::Webp => Color::Rgb(255, 255, 153),
        FileExtension::Png => Color::Rgb(204, 153, 255),
        FileExtension::Txt => Color::Rgb(255, 204, 153),
        FileExtension::NotImplemented => Color::Rgb(255, 255, 255),
    }
}

fn match_file_extension(file_extension: &str) -> FileExtension {
    match file_extension {
        "webp" => FileExtension::Webp,
        "png" => FileExtension::Png,
        "jpg" => FileExtension::Jpg,
        "txt" => FileExtension::Txt,
        _ => FileExtension::NotImplemented,
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug, Default)]
enum FileListType {
    #[default]
    FileListFrom,
    FileListTo,
}
#[derive(Debug, Default, Clone)]
pub struct File {
    path: String,
    name: String,
    extension: String,
    is_selected: bool,
}

impl File {
    fn init(path: String, name: String, extension: String) -> File {
        File {
            path,
            name,
            extension,
            is_selected: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct FileList {
    items: Vec<File>,
    state: ListState,
    path: String,
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    files_from: FileList,
    files_to: FileList,
    show_popup: bool,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;

        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        use Constraint::{Fill, Length, Min};

        let vertical = Layout::vertical([Length(1), Min(0), Length(3)]);
        let [title_area, main_area, status_area] = vertical.areas(frame.area());

        let horizontal = Layout::horizontal([Fill(1); 2]);
        let [left_area, right_area] = horizontal.areas(main_area);

        frame.render_widget(
            Block::bordered()
                .title("Blazingly fast moving files")
                .title_alignment(Alignment::Center),
            title_area,
        );

        self.render_to_list(right_area, frame);

        self.render_from_list(left_area, frame);

        self.render_footer(status_area, frame);

        if self.show_popup {
            self.render_popup(frame);
        }
    }

    fn render_footer(&mut self, area: Rect, frame: &mut Frame) {
        frame.render_widget(Paragraph::new("[↓↑]: Navigate files | [␣]: Select/Unselect file | [f]: Open source folder | [Ctrl+f]: Open destination folder | [s] Swap folders | [q]: Quit")
            .centered()
            .block(Block::bordered().title("Available Commands")), 
            area);
    }

    fn render_from_list(&mut self, area: Rect, frame: &mut Frame) {
        let items: Vec<ListItem> = self
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
            .block(Block::bordered().title(format!("Import from {}", self.files_from.path)))
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        frame.render_stateful_widget(files_from_list, area, &mut self.files_from.state);
    }

    fn render_to_list(&mut self, area: Rect, frame: &mut Frame) {
        let items: Vec<ListItem> = self
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
            .block(Block::bordered().title(format!("Import to {}", &self.files_to.path)))
            .style(Style::new().white());

        frame.render_stateful_widget(files_to_list, area, &mut self.files_to.state);
    }

    fn render_popup(&mut self, frame: &mut Frame) {
        let area = center(
            frame.area(),
            Constraint::Percentage(50),
            Constraint::Length(6),
        );
        let selected_count = self
            .files_from
            .items
            .iter()
            .filter(|f| f.is_selected)
            .count();
        let msg = format!(
            "Are you sure you want to move {} file(s)?\nDestination: {} \n\nPress (Y)es to confirm or (N)o/(Esc) to cancel",
            selected_count, &self.files_to.path
        );
        let popup = Paragraph::new(msg)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
            .block(Block::bordered().title("Confirm action"));

        frame.render_widget(Clear, area);
        frame.render_widget(popup, area);
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        if self.show_popup {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    self.show_popup = false;
                    if let Err(e) = self.move_files() {
                        eprintln!("Error moving files: {}", e);
                    }
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.show_popup = false;
                }
                _ => {}
            }
        } else {
            match (key.modifiers, key.code) {
                (_, KeyCode::Char('q')) => self.quit(),
                (_, KeyCode::Down) => self.select_next(),
                (_, KeyCode::Up) => self.select_previous(),
                (_, KeyCode::Char(' ')) => self.change_status(),
                (_, KeyCode::Char('s')) => self.swap_file_lists(),
                (KeyModifiers::CONTROL, KeyCode::Char('f')) => {
                    self.load_files_via_file_explorer(FileListType::FileListTo)
                }
                (_, KeyCode::Char('f')) => {
                    self.load_files_via_file_explorer(FileListType::FileListFrom)
                }
                (_, KeyCode::Enter) => {
                    self.show_popup = true;
                }
                _ => {}
            }
        }
    }

    fn select_next(&mut self) {
        self.files_from.state.select_next();
    }

    fn select_previous(&mut self) {
        self.files_from.state.select_previous();
    }

    fn change_status(&mut self) {
        if let Some(i) = self.files_from.state.selected() {
            self.files_from.items[i].is_selected = !self.files_from.items[i].is_selected;
        }
    }

    fn load_files_via_file_explorer(&mut self, list_type: FileListType) {
        if let Some(folder_path) = pick_folder(&list_type) {
            let dir_path = folder_path.clone().into_os_string().into_string().unwrap();
            match fs::read_dir(folder_path) {
                std::result::Result::Ok(entries) => {
                    let files: Vec<File> = entries
                        .filter_map(Result::ok)
                        .filter_map(|f| {
                            let path = f.path();

                            let path_string = path.to_string_lossy().to_string();

                            let file_name = path.file_name().and_then(|f| f.to_str())?;

                            let name: &str = file_name.rsplit_once('.')?.0;

                            let extension: String =
                                path.extension().and_then(|s| s.to_str())?.to_string();

                            Some(File::init(path_string, name.into(), extension))
                        })
                        .collect();

                    match list_type {
                        FileListType::FileListFrom => {
                            self.files_from.items = files;
                            self.files_from.path = dir_path;
                            self.files_from.state.select(Some(0));
                        }
                        FileListType::FileListTo => {
                            self.files_to.items = files;
                            self.files_to.path = dir_path;
                        }
                    }
                }
                Err(e) => eprint!("Error reading dir: {}", e),
            }
        }
    }

    fn swap_file_lists(&mut self) {
        std::mem::swap(&mut self.files_from, &mut self.files_to);
        self.files_from
            .items
            .iter_mut()
            .for_each(|f| f.is_selected = false)
    }

    fn move_files(&mut self) -> color_eyre::Result<()> {
        let files_to_move: Vec<File> = self
            .files_from
            .items
            .iter()
            .filter(|f| f.is_selected)
            .cloned()
            .collect();

        for file in &files_to_move {
            let old_path = Path::new(&file.path);
            let new_path_string =
                format!("{}/{}.{}", self.files_to.path, file.name, file.extension);
            let new_path = Path::new(&new_path_string);
            fs::rename(old_path, new_path)?;
        }

        self.files_from.items.retain(|f| !f.is_selected);

        self.files_to.items.extend(files_to_move);
        Ok(())
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.running = false;
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG_COLOR
    } else {
        ALT_ROW_BG_COLOR
    }
}

fn pick_folder(list_type: &FileListType) -> Option<std::path::PathBuf> {
    let title = match list_type {
        FileListType::FileListFrom => "Select folder to import files from",
        FileListType::FileListTo => "Select folder to import files to",
    };
    FileDialog::new().set_title(title).pick_folder()
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
