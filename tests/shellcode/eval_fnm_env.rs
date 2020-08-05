use super::expression::Expression;
use super::shell::{Bash, Fish, PowerShell, WinCmd, Zsh};
use std::fmt::Write;

#[derive(Debug, Default)]
pub(crate) struct EvalFnmEnv {
    use_on_cd: bool,
    node_dist_mirror: Option<reqwest::Url>,
}

impl EvalFnmEnv {
    pub(crate) fn use_on_cd(self, use_on_cd: bool) -> Self {
        Self { use_on_cd, ..self }
    }

    pub(crate) fn node_dist_mirror(self, node_dist_mirror: Option<impl reqwest::IntoUrl>) -> Self {
        Self {
            node_dist_mirror: node_dist_mirror.map(|x| x.into_url().unwrap()),
            ..self
        }
    }
}

impl std::fmt::Display for EvalFnmEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fnm")?;
        if let Some(node_dist_mirror) = &self.node_dist_mirror {
            write!(f, " --node-dist-mirror='{}'", node_dist_mirror)?;
        }
        write!(f, " env")?;
        if self.use_on_cd {
            write!(f, " --use-on-cd")?;
        }
        Ok(())
    }
}

impl Expression<WinCmd> for EvalFnmEnv {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        write!(writer, r#"FOR /f "tokens=*" %i IN ('{}') DO CALL %i"#, self)
    }
}

impl Expression<PowerShell> for EvalFnmEnv {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        write!(writer, r#"{} | Out-String | Invoke-Expression"#, self)
    }
}

impl Expression<Zsh> for EvalFnmEnv {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        write!(writer, r#"eval "$({})""#, self)
    }
}

impl Expression<Bash> for EvalFnmEnv {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        write!(writer, r#"eval "$({})""#, self)
    }
}

impl Expression<Fish> for EvalFnmEnv {
    fn write_shell(&self, writer: &mut impl Write) -> std::fmt::Result {
        write!(writer, r#"{} | source"#, self)
    }
}
