use ratatui::widgets::ListState;

use super::file::File;

#[derive(Debug, Default)]
pub struct FileList {
    pub items: Vec<File>,
    pub state: ListState,
    pub path: String,
}

impl FileList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn select_next(&mut self) {
        self.state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }

    pub fn selected_count(&self) -> usize {
        self.items.iter().filter(|f| f.is_selected).count()
    }

    pub fn change_status(&mut self) {
        if let Some(i) = self.state.selected() {
            self.items[i].is_selected = !self.items[i].is_selected;
        }
    }
}

#[derive(Debug, Default)]
pub enum FileListType {
    #[default]
    FileListFrom,
    FileListTo,
}
