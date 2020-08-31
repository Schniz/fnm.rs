use crate::shellcode::*;

mod unaliasing {
    test_shell!(Bash, Zsh, PowerShell, Fish; {
        EvalFnmEnv::default()
            .then(Call::new("fnm", vec!["install", "v10.11"]))
            .then(Call::new("fnm", vec!["alias", "v10.11", "test-alias"]))
            .then(OutputContains::new(Call::new("fnm", vec!["ls"]), "test-alias"))
            .then(OutputContains::new(Call::new("fnm", vec!["uninstall", "test-alias"]), "Unaliased"))
            .then(OutputContains::new(Call::new("fnm", vec!["ls"]), "test-alias").not_match(true))
    });
}

mod uninstalling {
    test_shell!(Bash, Zsh, PowerShell, Fish; {
        EvalFnmEnv::default()
            .then(Call::new("fnm", vec!["install", "v10.11.0"]))
            .then(OutputContains::new(Call::new("fnm", vec!["uninstall", "v10.11.0"]), "Uninstalled version"))
            .then(OutputContains::new(
                IgnoreErrors::new(GetStderr::new(Call::new("fnm", vec!["use", "v10.11.0"]))),
                "is not currently installed",
            ))
    });
}
