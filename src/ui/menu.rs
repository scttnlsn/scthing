use crate::ui;
use raqote;

#[derive(Debug, Clone)]
pub struct MenuItem {
    label: String,
    action: Option<ui::Action>,
}

impl MenuItem {
    pub fn new(label: &str, action: ui::Action) -> Self {
        MenuItem {
            label: label.to_string(),
            action: Some(action),
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
    pub fn new(menu_items: Vec<MenuItem>) -> Self {
        Menu {
            path: vec![],
            selected: 0,
            items: menu_items
        }
    }

    pub fn down(&mut self) {
        if self.selected < (self.items.len() - 1) {
            self.selected += 1;
        }
    }

    pub fn up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn select(&mut self) -> Option<ui::Action> {
        let item = &self.items[self.selected];
        item.action
    }
}

impl ui::Screen for Menu {
    fn render(&self, target: &mut raqote::DrawTarget) {
        let lines = self.items.iter().enumerate().map(|(i, item)| {
            if self.selected == i {
                format!("* {}", item.label)
            } else {
                format!("  {}", item.label)
            }
        }).collect();

        ui::render_lines(lines, target);
    }

    fn handle(&mut self, input: ui::Input) -> Option<ui::Action> {
        match input {
            ui::Input::Left => {
                self.up();
                None
            },
            ui::Input::Right => {
                self.down();
                None
            },
            ui::Input::Press => {
                self.select()
            },
        }
    }
}
