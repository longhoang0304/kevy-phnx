use std::any::Any;

pub mod actions;

pub struct Action {
    command: string,
    key: string,
    value: Any,
}

impl Action {
    pub fn new() -> Box<Action> {
        Action {
            command: (),
            key: (),
            value: (),
        }
    }
}
