use crate::osc;
use crate::ui;
use raqote;

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub value: f32,
    pub step: f32
}

impl Param {
    pub fn new(name: String, value: f32, step: f32) -> Self {
        Param {
            name: name,
            value: value,
            step: step,
        }
    }

    pub fn inc(&mut self) {
        self.value += self.step;
    }

    pub fn dec(&mut self) {
        self.value -= self.step;
    }

    pub fn send(&self) {
        osc::send("set", Some(vec![
            osc::Type::String(self.name.clone()),
            osc::Type::Float(self.value),
        ]));
    }
}

impl ui::Screen for Param {
    fn render(&self, target: &mut raqote::DrawTarget) {
        let lines = vec![
            format!("{} = {:?}", self.name, self.value)
        ];

        ui::render_lines(lines, target);
    }

    fn handle(&mut self, input: ui::Input) -> Option<ui::Action> {
        match input {
            ui::Input::Left => {
                self.dec();
                self.send();
                None
            },
            ui::Input::Right => {
                self.inc();
                self.send();
                None
            },
            ui::Input::Press => {
                Some(ui::Action::Pop)
            },
        }
    }

    fn load(&mut self) {
    }

    fn unload(&mut self) {
    }
}
