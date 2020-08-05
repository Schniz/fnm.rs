use crate::shellcode::*;
use crate::test_shell;

test_shell!(basic, Zsh, Bash, Fish, PowerShell, WinCmd, {
    EvalFnmEnv::default()
        .then(Call::new("fnm", vec!["install", "v8.11.3"]))
        .then(Call::new("fnm", vec!["use", "v8.11.3"]))
        .then(test_node_version("v8.11.3"))
});

test_shell!(nvmrc, Zsh, Bash, Fish, PowerShell, WinCmd, {
    EvalFnmEnv::default()
        .then(WriteFile::new(".nvmrc", "v8.11.3"))
        .then(Call::new("fnm", vec!["install"]))
        .then(Call::new("fnm", vec!["use"]))
        .then(test_node_version("v8.11.3"))
});

test_shell!(multishell, Zsh, Bash, Fish, PowerShell, {
    EvalFnmEnv::default()
        .then(Call::new("fnm", vec!["install", "v8.11.3"]))
        .then(Call::new("fnm", vec!["install", "v11.9.0"]))
        .then(Call::new("fnm", vec!["use", "v8.11.3"]))
        .then(SubShell::new(
            DieOnErrors
                .then(EvalFnmEnv::default())
                .then(Call::new("fnm", vec!["use", "11"]))
                .then(test_node_version("v11.9.0")),
        ))
        .then(test_node_version("v8.11.3"))
});

test_shell!(use_on_cd, Zsh, Bash, Fish, {
    EvalFnmEnv::default()
        .use_on_cd(true)
        .then(Call::new("mkdir", vec!["inner_path"]))
        .then(Call::new("echo", vec!["v8.11.3", ">", "inner_path/.nvmrc"]))
        .then(Call::new("fnm", vec!["install", "v8.11.3"]))
        .then(Call::new("cd", vec!["inner_path"]))
        .then(test_node_version("v8.11.3"))
});

test_shell!(node_dist_mirror, Zsh, Bash, Fish, {
    EvalFnmEnv::default()
        .node_dist_mirror(Some("https://npm.taobao.org/mirrors/node"))
        .then(Call::new("fnm", vec!["install", "v8.11.3"]))
        .then(Call::new("fnm", vec!["use", "v8.11.3"]))
        .then(test_node_version("v8.11.3"))
});
