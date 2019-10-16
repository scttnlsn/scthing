use crate::ui;
use raqote;

#[derive(Debug)]
pub struct Param {
    label: String,
    value: i32,
}

impl Param {
    pub fn new(label: String) -> Self {
        Param {
            label: label,
            value: 0,
        }
    }

    pub fn inc(&mut self) {
        self.value += 1;
    }

    pub fn dec(&mut self) {
        self.value -= 1;
    }
}

impl ui::Screen for Param {
    fn render(&self, target: &mut raqote::DrawTarget) {
        let lines = vec![
            format!("{} = {:?}", self.label, self.value)
        ];

        ui::render_lines(lines, target);
    }

    fn handle(&mut self, input: ui::Input) -> Option<ui::Action> {
        match input {
            ui::Input::Left => {
                self.dec();
                None
            },
            ui::Input::Right => {
                self.inc();
                None
            },
            ui::Input::Press => {
                Some(ui::Action::Pop)
            },
        }
    }
}
