use std::path::{Path, PathBuf};
use crate::company::Company;
use std::io;

pub enum InputMode {
    Normal,
    Editing,
}

#[derive(PartialEq)]
pub enum Focus {
    Nothing,
    Charts,
    ExpenseReport,
}

pub struct App {
    pub input: String,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
    pub progress: f64,
    pub db_path: PathBuf,
    pub company: Company,
    pub focus: Focus,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            progress: 0.0,
            db_path: PathBuf::new(),
            company: Company::default(),
            focus: Focus::Nothing,
        }
    }
}

impl App {
    pub fn toggle_focus(&mut self, focus: Focus) {
        if self.focus == focus {
            self.focus = Focus::Nothing;
        }
        else {
            self.focus = focus;
        }

    }

    pub fn on_tick(&mut self) {
        self.progress += 0.01;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }
    }

    pub fn load_company(mut self) -> io::Result<Self> {
        self.company = self.company.load(self.db_path.as_path())?;
        Ok(self)
    }

    pub fn database<T>(mut self, path: T) -> Self
    where T: Into<PathBuf>
    {
        self.db_path = path.into();
        self
    }
}