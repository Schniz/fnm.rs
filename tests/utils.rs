pub fn run_test_file(current_dir: impl AsRef<std::path::Path>) {
    let fnm_dir = tempfile::tempdir().unwrap();
    let path_env = std::env::var("PATH").unwrap();
    let target_dir = {
        let mut path = std::path::PathBuf::from(env!("CARGO_BIN_EXE_fnm"));
        path.pop();
        path
    };
    let path_str = format!("{}:{}", target_dir.to_str().unwrap(), path_env);
    duct::cmd!("bash", "run.sh")
        .dir(current_dir.as_ref())
        .env("PATH", path_str)
        .env("FNM_DIR", fnm_dir.path())
        .run()
        .unwrap();
}

#[macro_export]
macro_rules! bash_feature {
    ($feature_name:ident) => {
        #[test]
        fn $feature_name() {
            let current_dir = std::env::current_dir()
                .unwrap()
                .join("feature_tests")
                .join(stringify!($feature_name));
            $crate::utils::run_test_file(current_dir);
        }
    };
}
