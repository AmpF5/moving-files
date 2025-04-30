use std::{fs, path::{Path, PathBuf}};

use color_eyre::{eyre::{Error, Ok}, Result};
use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::{Color, Style}, widgets::{block::Position, ListItem, ListState, Widget}};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout}, style::{palette::tailwind::SLATE, Modifier, Stylize}, text::Line, widgets::{Block, List, ListDirection, Paragraph}, DefaultTerminal, Frame
};
use rfd::FileDialog;
static SELECTED_STYLE: Style = Style::new().bg(SLATE.c500).add_modifier(Modifier::BOLD);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;

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
    FileListTo
}
#[derive(Debug, Default)]
pub struct File {
    path: String,
    extension: String,
    is_selected: bool
}

impl File {
    fn init(path: String, extension: String) -> File {
        File {path, extension, is_selected: false} 
    }
}

#[derive(Debug, Default)]
pub struct FileList {
    items: Vec<File>,
    state: ListState,
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    files_from: FileList,
    files_to: FileList
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        self.files_from = FileList {
            items: vec![
                File::init(String::from("From 1"), ".jpg".into()),
                File::init(String::from("From 2"), ".jpg".into()),
                File::init(String::from("From 3"), ".jpg".into()),
                File::init(String::from("From 4"), ".jpg".into()),
                File::init(String::from("From 5"), ".jpg".into()),
                File::init(String::from("From 6"), ".jpg".into()),
            ],
            state: ListState::default(),
        };

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

        frame.render_widget(Block::bordered().title("Blazingly fast moving files").title_alignment(Alignment::Center), title_area);

        self.render_to_list(right_area, frame);

        self.render_from_list(left_area, frame);

        self.render_footer(status_area, frame);
    }

    fn render_footer(&mut self, area: Rect, frame: &mut Frame) {
        frame.render_widget(Paragraph::new("Use ↓↑ to move, ␣ to select/unselect, f to open file explorer, q to QUIT")
            .centered()
            .block(Block::bordered().title("Options")), 
            area);
    }

    fn render_from_list(&mut self, area: Rect, frame: &mut Frame) {
        let items: Vec<ListItem> = self
            .files_from
            .items
            .iter()
            .enumerate()
            .map(|(i, file)| {
                let color = alternate_colors(i);
                let mut item = ListItem::from(file.path.clone()).style(Style::new().bg(color));
                if file.is_selected {
                    item = item.style(SELECTED_STYLE);
                } else {
                    item = item.style(Style::new().bg(color))
                }
                item
            })
            .collect();

        let files_from_list = List::new(items)
            .block(Block::bordered().title("Import from"))
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
                let color = alternate_colors(i);
                let mut item = ListItem::from(file.path.clone()).style(Style::new().bg(color));
                item = item.style(Style::new().bg(color));
                item
            })
            .collect();

        let files_to_list = List::new(items)
            .block(Block::bordered().title("Import to"))
            .style(Style::new().white());

        frame.render_stateful_widget(files_to_list, area, &mut self.files_to.state);

    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Char('q'))  => self.quit(),
            (_, KeyCode::Down) => self.select_next(),
            (_, KeyCode::Up) => self.select_previous(),
            (_, KeyCode::Char(' ')) => self.change_status(),
            (KeyModifiers::CONTROL, KeyCode::Char('f')) => self.load_files_via_file_explorer(FileListType::FileListTo),
            (_, KeyCode::Char('f')) => self.load_files_via_file_explorer(FileListType::FileListFrom),
            _ => {}
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
        if let Some(folder_path) = pick_folder() {
            match fs::read_dir(folder_path) {
                std::result::Result::Ok(entries) => { 
                    let files : Vec<File> = entries
                        .filter_map(Result::ok)
                        .filter_map(|f| {
                            let path = f.path();
                            let path_string = path.to_string_lossy().to_string();
                            let extension = path.extension()
                                .and_then(|e| e.to_str())
                                .unwrap_or("")
                                .to_string();
                            Some(File::init(path_string, extension))
                        })
                        .collect();

                    match list_type {
                        FileListType::FileListFrom => {
                            self.files_from.items = files;
                            self.files_from.state.select(Some(0));
                        }
                        FileListType::FileListTo => {
                            self.files_to.items = files;
                        }
                    }

                },
                Err(e) => eprint!("Error reading dir: {}", e)
            }
        }
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
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
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

fn pick_folder() -> Option<std::path::PathBuf> {
    FileDialog::new()
        .set_title("Select folder")
        .pick_folder()
}
