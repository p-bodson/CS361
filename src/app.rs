pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub input: String,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
    pub progress: f64,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            progress: 0.0,
        }
    }
}

impl App {
    pub fn on_tick(&mut self) {
        self.progress += 0.01;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }
    }
}