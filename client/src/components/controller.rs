#![allow(dead_code)]

use piston::{Button, ButtonArgs, ButtonState};
use std::collections::HashMap;

pub struct Controller {
    pub held: HashMap<Button, bool>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            held: HashMap::new(),
        }
    }

    pub fn on_press<F>(&mut self, args: &ButtonArgs, f: Option<F>)
    where
        F: FnOnce(Button),
    {
        if args.state == ButtonState::Press {
            if let Some(f) = f {
                f(args.button);
            }

            self.held.insert(args.button, true);
        }
    }

    pub fn on_release<F>(&mut self, args: &ButtonArgs, f: Option<F>)
    where
        F: FnOnce(Button),
    {
        if args.state == ButtonState::Release {
            if let Some(f) = f {
                f(args.button);
            }

            self.held.insert(args.button, false);
        }
    }

    pub fn on_hold<F>(&self, button: &Button, f: F)
    where
        F: FnOnce(),
    {
        if let Some(is_held) = self.held.get(button) {
            if *is_held {
                f()
            }
        }
    }

    pub fn on_click(&self) {}
}

pub trait Controllable {
    fn controller(&self) -> &Controller;
    fn controller_mut(&mut self) -> &mut Controller;
    fn button(&mut self, args: &ButtonArgs) -> ();
}
