mod infer_shell;
mod shell;

use self::shell::Shell;
use super::command::Command;
use crate::config::FnmConfig;
use clap::Clap;
use std::fmt::Debug;

#[derive(Clap, Debug)]
pub struct Env {}

#[derive(Debug)]
pub enum Error {}

impl Command for Env {
    type Error = Error;

    fn apply(self, config: FnmConfig) -> Result<(), Self::Error> {
        let maybe_shell: Option<Box<dyn Shell>> = if cfg!(windows) {
            Some(Box::from(self::shell::WindowsCmd))
        } else {
            use self::infer_shell::infer_shell;
            infer_shell()
        };
        let shell = maybe_shell.expect("Can't infer shell!");
        dbg!(shell);
        Ok(())
    }

    fn handle_error(err: Self::Error) {
        dbg!(err);
    }
}

#[cfg(windows)]
fn infer_shell() -> Option<Box<WindowsCmd>> {
    Box::from(WindowsCmd)
}
