use font_kit::font::Font;
use raqote;
use std::sync::Arc;

pub static FONT_BYTES: &'static [u8; 92600] = include_bytes!("fonts/inconsolata.ttf");

const BACKGROUND: raqote::SolidSource = raqote::SolidSource { r: 0x00, g: 0x00, b: 0x00, a: 0x00 };
const FOREGROUND: raqote::SolidSource = raqote::SolidSource { r: 0xFF, g: 0xFF, b: 0xFF, a: 0xFF };
const FOREGROUND_SOURCE: raqote::Source = raqote::Source::Solid(FOREGROUND);

#[derive(Debug)]
pub struct MenuItem {
    pub label: &'static str,
    pub children: Vec<MenuItem>,
    back: bool
}

impl MenuItem {
    pub fn new(label: &'static str, children: Vec<MenuItem>) -> Self {
        MenuItem {
            label: label,
            children: children,
            back: false,
        }
    }
}

#[derive(Debug)]
pub struct Menu {
    path: Vec<usize>,
    selected: usize,
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new(mut menu_items: Vec<MenuItem>) -> Self {
        for item in &mut menu_items {
            item.children.push(MenuItem {
                label: "<-",
                children: vec![],
                back: true,
            })
        }

        Menu {
            path: vec![],
            selected: 0,
            items: menu_items
        }
    }

    fn active_items(self: &Self) -> &Vec<MenuItem> {
        let mut items = &self.items;

        for i in &self.path {
            items = &items[*i as usize].children;
        }

        items
    }

    pub fn render(self: &Self, target: &mut raqote::DrawTarget) {
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

        for (i, item) in self.active_items().iter().enumerate() {
            let point = raqote::Point::new(0.0, (line_height * (i + 1) as f32) + offset);
            let txt = if self.selected == i {
                format!("* {}", item.label)
            } else {
                format!("  {}", item.label)
            };

            draw_text(&txt, point);
        }
    }

    pub fn down(self: &mut Self) {
        if self.selected < (self.active_items().len() - 1) {
            self.selected += 1;
        }
    }

    pub fn up(self: &mut Self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn select(self: &mut Self) {
        let item = &self.active_items()[self.selected];

        if item.back {
            self.selected = self.path.pop().unwrap();
        } else if item.children.len() > 0 {
            self.path.push(self.selected);
            self.selected = 0;
        } else {
            // TODO: add callback to menu item, executed here
            println!("selected: {}", item.label);
        }
    }
}
