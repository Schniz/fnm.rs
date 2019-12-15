use indoc::indoc;
use std::fmt::Debug;
use std::path::PathBuf;

pub trait Shell: Debug {
    fn path(&self, path: &PathBuf) -> String;
    fn set_env_var(&self, name: &str, value: &str) -> String;
    fn use_on_cd(&self) -> String;
}

#[cfg(windows)]
pub const AVAILABLE_SHELLS: [&'static str; 1] = ["cmd"];

#[cfg(unix)]
pub const AVAILABLE_SHELLS: [&'static str; 2] = ["bash", "zsh"];

#[derive(Debug)]
pub struct Bash;

impl Shell for Bash {
    fn path(&self, path: &PathBuf) -> String {
        format!("export PATH={}/bin:$PATH", path.to_str().unwrap())
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("export {}={}", name, value)
    }

    fn use_on_cd(&self) -> String {
        indoc!(
            r#"
                __fnmcd () {
                    cd "$@"

                    if [[ -f .node-version && .node-version ]]; then
                        echo "fnm: Found .node-version"
                        fnm use
                    elif [[ -f .nvmrc && .nvmrc ]]; then
                        echo "fnm: Found .nvmrc"
                        fnm use
                    fi
                }

                alias cd=__fnmcd
            "#
        )
        .into()
    }
}

#[derive(Debug)]
pub struct Zsh;

impl Shell for Zsh {
    fn path(&self, path: &PathBuf) -> String {
        format!("export PATH={}/bin:$PATH", path.to_str().unwrap())
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("export {}={}", name, value)
    }

    fn use_on_cd(&self) -> String {
        indoc!(
            r#"
                autoload -U add-zsh-hook
                _fnm_autoload_hook () {
                    if [[ -f .node-version && -r .node-version ]]; then
                        echo "fnm: Found .node-version"
                        fnm use
                    elif [[ -f .nvmrc && -r .nvmrc ]]; then
                        echo "fnm: Found .nvmrc"
                        fnm use
                    fi
                }

                add-zsh-hook chpwd _fnm_autoload_hook \
                    && _fnm_autoload_hook
            "#
        )
        .into()
    }
}

#[derive(Debug)]
pub struct WindowsCmd;

impl Shell for WindowsCmd {
    fn path(&self, path: &PathBuf) -> String {
        format!("SET PATH={};%PATH%", path.to_str().unwrap())
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("SET {}={}", name, value)
    }

    fn use_on_cd(&self) -> String {
        "".into()
    }
}

impl std::str::FromStr for Box<dyn Shell> {
    type Err = String;

    fn from_str(s: &str) -> Result<Box<dyn Shell>, Self::Err> {
        match s {
            "cmd" => Ok(Box::from(WindowsCmd)),
            "zsh" => Ok(Box::from(Zsh)),
            "bash" => Ok(Box::from(Bash)),
            shell_type => Err(format!("I don't know the shell type of {:?}", shell_type)),
        }
    }
}
