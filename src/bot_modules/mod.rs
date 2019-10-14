mod main;
mod moderation;

pub mod bot_modules {
    use crate::command::Command;

    pub trait BotModule {
        fn name(&self) -> String;
        fn desc(&self) -> String;
        fn enabled(&self) -> bool;
        fn commands(&self) -> Vec<Box<dyn Command>>;
    }

    pub fn get_modules() -> Vec<Box<dyn BotModule>> {
        vec! [
            Box::new(super::main::MainModule{})
        ]
    }
}