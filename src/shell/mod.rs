mod bash;
mod fish;
mod infer_shell;
mod shell;
mod windows_cmd;
mod zsh;

pub use bash::Bash;
pub use fish::Fish;
pub use shell::{Shell, AVAILABLE_SHELLS};
pub use windows_cmd::WindowsCmd;
pub use zsh::Zsh;

/// Always returns WindowsCmd (because this is what we support on Windows)
#[cfg(windows)]
pub fn infer_shell() -> Box<dyn Shell> {
    Box::from(windows_cmd::WindowsCmd)
}

/// Tries to infer shell or dies trying
#[cfg(unix)]
pub fn infer_shell() -> Box<dyn Shell> {
    infer_shell::infer_shell().expect("Can't infer shell")
}
