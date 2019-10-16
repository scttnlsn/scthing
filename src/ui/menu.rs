use crate::ui;
use raqote;

pub struct MenuItem {
    label: &'static str,
    children: Vec<MenuItem>,
    back: bool,
    action: Option<ui::Action>,
}

impl MenuItem {
    pub fn menu(label: &'static str, children: Vec<MenuItem>) -> Self {
        MenuItem {
            label: label,
            children: children,
            back: false,
            action: None,
        }
    }

    pub fn item(label: &'static str, action: ui::Action) -> Self {
        MenuItem {
            label: label,
            children: vec![],
            back: false,
            action: Some(action),
        }
    }

    fn insert_back_button(self: &mut Self) {
        if self.children.len() > 0 {
            for child in &mut self.children {
                child.insert_back_button();
            }

            self.children.push(MenuItem {
                label: "<-",
                children: vec![],
                back: true,
                action: None,
            })
        }
    }
}

pub struct Menu {
    path: Vec<usize>,
    selected: usize,
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new(mut menu_items: Vec<MenuItem>) -> Self {
        for item in &mut menu_items {
            item.insert_back_button()
        }

        Menu {
            path: vec![],
            selected: 0,
            items: menu_items
        }
    }

    fn active_items(&self) -> &Vec<MenuItem> {
        let mut items = &self.items;

        for i in &self.path {
            items = &items[*i as usize].children;
        }

        items
    }

    pub fn down(&mut self) {
        if self.selected < (self.active_items().len() - 1) {
            self.selected += 1;
        }
    }

    pub fn up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn select(&mut self) -> Option<ui::Action> {
        let item = &self.active_items()[self.selected];

        if item.back {
            self.selected = self.path.pop().unwrap();
            None
        } else if item.children.len() > 0 {
            self.path.push(self.selected);
            self.selected = 0;
            None
        } else {
            item.action
        }
    }
}

impl ui::Screen for Menu {
    fn render(&self, target: &mut raqote::DrawTarget) {
        let lines = self.active_items().iter().enumerate().map(|(i, item)| {
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
