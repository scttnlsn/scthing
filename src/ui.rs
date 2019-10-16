use raqote;

pub enum Input {
    Right,
    Left,
    Press,
}

pub trait Screen {
    fn render(&self, target: &mut raqote::DrawTarget);
    fn handle(&mut self, message: Input);
}

pub struct UI<T: Screen> {
    screens: Vec<T>,
}

impl<T: Screen> UI<T> {
    pub fn new() -> Self {
        UI {
            screens: vec![],
        }
    }

    pub fn current_screen(&mut self) -> Option<&mut T> {
        self.screens.last_mut()
    }

    pub fn push_screen(&mut self, screen: T) {
        self.screens.push(screen)
    }

    pub fn pop_screen(&mut self) -> Option<T> {
        self.screens.pop()
    }

    pub fn render(&mut self, target: &mut raqote::DrawTarget) {
        match self.current_screen() {
            Some(screen) => {
                screen.render(target);
            },
            None => {}
        }
    }

    pub fn handle(&mut self, message: Input) {
        match self.current_screen() {
            Some(screen) => {
                screen.handle(message);
            },
            None => {}
        }
    }
}
