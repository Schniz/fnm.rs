use super::shell::Shell;
use indoc::indoc;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Bash;

impl Shell for Bash {
    fn path(&self, path: &PathBuf) -> String {
        format!("export PATH={:?}/bin:$PATH", path.to_str().unwrap())
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("export {}={:?}", name, value)
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
