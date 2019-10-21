pub mod menu;
pub mod param;
pub mod patch;

use font_kit::font::Font;
use raqote;
use std::collections::HashMap;
use std::sync::Arc;

pub static FONT_BYTES: &'static [u8; 92600] = include_bytes!("ui/fonts/inconsolata.ttf");

const BACKGROUND: raqote::SolidSource = raqote::SolidSource { r: 0x00, g: 0x00, b: 0x00, a: 0x00 };
const FOREGROUND: raqote::SolidSource = raqote::SolidSource { r: 0xFF, g: 0xFF, b: 0xFF, a: 0xFF };
const FOREGROUND_SOURCE: raqote::Source = raqote::Source::Solid(FOREGROUND);

pub type ScreenId = u32;

pub enum Input {
    Right,
    Left,
    Press,
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Push(ScreenId),
    Pop,
}

pub trait Screen {
    fn render(&self, target: &mut raqote::DrawTarget);
    fn handle(&mut self, input: Input) -> Option<Action>;

    fn load(&mut self) {}
    fn unload(&mut self) {}
}

type ScreenT = Box<dyn Screen>;

pub struct UI {
    screens: HashMap<ScreenId, ScreenT>,
    stack: Vec<ScreenId>,
    next_id: ScreenId,
}

impl UI {
    pub fn new() -> Self {
        UI {
            screens: HashMap::new(),
            stack: vec![],
            next_id: 0,
        }
    }

    pub fn register<T: 'static + Screen>(&mut self, screen: T) -> ScreenId {
        let screen_id = self.next_id;
        self.screens.insert(self.next_id, Box::new(screen));
        self.next_id += 1;
        screen_id
    }

    pub fn current_screen(&mut self) -> Option<&mut ScreenT> {
        let id = self.stack.last()?;
        self.screens.get_mut(id)
    }

    pub fn push_screen(&mut self, screen_id: ScreenId) {
        self.stack.push(screen_id);
    }

    pub fn pop_screen(&mut self) {
        self.stack.pop();
    }

    pub fn render(&mut self, target: &mut raqote::DrawTarget) {
        match self.current_screen() {
            Some(screen) => {
                screen.render(target);
            },
            None => {}
        }
    }

    pub fn handle(&mut self, input: Input) {
        let action = match self.current_screen() {
            Some(screen) => { screen.handle(input) },
            None => { None }
        };

        if let Some(action) = action {
            match action {
                Action::Push(screen_id) => {
                    self.push_screen(screen_id);

                    if let Some(screen) = self.current_screen() {
                        screen.load();
                    }
                },
                Action::Pop => {
                    if let Some(screen) = self.current_screen() {
                        screen.unload();
                    }

                    self.pop_screen();
                },
            }
        }
    }
}

fn render_lines(lines: Vec<String>, target: &mut raqote::DrawTarget) {
    target.clear(BACKGROUND);

    let draw_options = raqote::DrawOptions::new();
    let font = Font::from_bytes(Arc::new(FONT_BYTES.to_vec()), 0).unwrap();

    let mut draw_text = |text: &str, mut start: raqote::Point| {
        let mut ids = Vec::new();
        let mut positions = Vec::new();
        for c in text.chars() {
            let id = font.glyph_for_char(c).unwrap();
            ids.push(id);
            positions.push(start);
            start += font.advance(id).unwrap() / 70.0;
        }

        target.draw_glyphs(&font, 14.0, &ids, &positions, &FOREGROUND_SOURCE, &draw_options);
    };

    let offset = 2.0;
    let line_height = 14.0;

    for (i, line) in lines.iter().enumerate() {
        let point = raqote::Point::new(0.0, (line_height * (i + 1) as f32) + offset);
        draw_text(&line, point);
    }
}
