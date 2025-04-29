use color_eyre::Result;
use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::{Color, Style}, widgets::{block::Position, ListItem, ListState, Widget}};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout}, style::{palette::tailwind::SLATE, Modifier, Stylize}, text::Line, widgets::{Block, List, ListDirection, Paragraph}, DefaultTerminal, Frame
};
static SELECTED_STYLE: Style = Style::new().bg(SLATE.c900).add_modifier(Modifier::BOLD);
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
pub struct File {
    path: String,
    is_selected: bool
}

impl File {
    fn init(path: String) -> File {
        File {path, is_selected: false} 
    }
}

#[derive(Debug, Default)]
pub struct FileList {
    items: Vec<File>,
    state: ListState
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    files_from: FileList,
    files_to: FileList
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        self.files_from = FileList {
            items: vec![
                File::init(String::from("From 1")),
                File::init(String::from("From 2")),
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

        // let mut state = ListState::default();

        frame.render_widget(Block::bordered().title("Blazingly fast moving files").title_alignment(Alignment::Center), title_area);


        frame.render_widget(Block::bordered().title("Import to"), right_area);

        self.render_from_list(left_area, frame);

        self.render_footer(status_area, frame);
    }

    fn render_footer(&mut self, area: Rect, frame: &mut Frame) {
        frame.render_widget(Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
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
                ListItem::from(file.path.clone()).bg(color)
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

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q')  => self.quit(),
            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Char(' ') => self.change_status(),
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
            self.files_from.items[i].is_selected = true
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
