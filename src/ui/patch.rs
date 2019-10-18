use crate::osc;
use crate::ui;
use raqote;

#[derive(Debug)]
pub struct Patch {
    name: String,
    menu: ui::menu::Menu,
}

impl Patch {
    pub fn new(name: String, menu: ui::menu::Menu) -> Self {
        Patch {
            name: name,
            menu: menu,
        }
    }

    pub fn start(&self) {
        osc::send("start", Some(vec![osc::Type::String(self.name.clone())]));
    }

    pub fn stop(&self) {
        osc::send("stop", None);
    }
}

impl ui::Screen for Patch {
    fn render(&self, target: &mut raqote::DrawTarget) {
        self.menu.render(target);
    }

    fn handle(&mut self, input: ui::Input) -> Option<ui::Action> {
        self.menu.handle(input)
    }

    fn load(&mut self) {
        self.start();
    }

    fn unload(&mut self) {
        self.stop();
    }
}
