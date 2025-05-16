#[derive(Debug, Default, Clone)]
pub struct File {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub is_selected: bool,
}

impl File {
    pub fn init(path: String, name: String, extension: String) -> File {
        File {
            path,
            name,
            extension,
            is_selected: false,
        }
    }

    pub fn toggle_selection(&mut self) {
        self.is_selected = !self.is_selected;
    }
}

pub enum FileExtension {
    Webp,
    Png,
    Jpg,
    Txt,
    NotImplemented,
}
