use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::DefaultTerminal;

use crate::{
    models::file_list::{FileList, FileListType},
    ui::render::render_main_windows,
    utils::fs::{load_files_via_file_explorer, move_selected_files},
};

#[derive(Debug, Default)]
pub struct App {
    pub running: bool,
    pub files_from: FileList,
    pub files_to: FileList,
    pub show_popup: bool,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        self.running = true;

        while self.running {
            terminal.draw(|frame| render_main_windows(&mut self, frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        if self.show_popup {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    self.show_popup = false;
                    if let Err(e) = move_selected_files(self) {
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
                (_, KeyCode::Down) => self.files_from.select_next(),
                (_, KeyCode::Up) => self.files_from.select_previous(),
                (_, KeyCode::Char(' ')) => self.files_from.change_status(),
                (_, KeyCode::Char('s')) => self.swap_file_lists(),
                (KeyModifiers::CONTROL, KeyCode::Char('f')) => {
                    load_files_via_file_explorer(self, FileListType::FileListTo)
                }
                (_, KeyCode::Char('f')) => {
                    load_files_via_file_explorer(self, FileListType::FileListFrom)
                }
                (_, KeyCode::Enter) => {
                    self.show_popup = true;
                }
                _ => {}
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

    fn handle_crossterm_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
