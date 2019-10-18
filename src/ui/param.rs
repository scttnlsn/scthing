use crate::osc;
use crate::ui;
use raqote;

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub value: f32,
    pub step: f32,
    pub min: f32,
    pub max: f32,
}

impl Param {
    pub fn new(name: String, value: f32, step: f32, min: f32, max: f32) -> Self {
        Param {
            name: name,
            value: value,
            step: step,
            min: min,
            max: max,
        }
    }

    pub fn inc(&mut self) {
        self.value += self.step;
        if self.value > self.max {
            self.value = self.max;
        }
    }

    pub fn dec(&mut self) {
        self.value -= self.step;
        if self.value < self.min {
            self.value = self.min;
        }
    }

    pub fn perc(&self) -> f32 {
        self.value / self.max
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
        let mut lines = vec![
            format!("{}:", self.name),
            format!("{:.*}", 2, self.value),
        ];

        let max_width = 13.0; // max chars wide
        let val_width = (max_width * self.perc()).round() as usize;
        lines.push(vec!["-"; val_width].join(""));

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
}
