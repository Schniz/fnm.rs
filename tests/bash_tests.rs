#[macro_use]
mod utils;

#[cfg(not(windows))]
mod raw_bash_tests {
    bash_feature![aliases];
    bash_feature![almost_matching_dotfiles];
    bash_feature![current];
    bash_feature![exec];
    bash_feature![existing_installation];
    bash_feature![latest_lts];
    bash_feature![list_local_with_nothing_installed];
    bash_feature![log_level_error];
    bash_feature![log_level_quiet];
    bash_feature![matching_dotfiles];
    bash_feature![node_version];
    bash_feature![partial_semver];
    bash_feature![system_node];
    bash_feature![use_nvmrc_lts];
}
