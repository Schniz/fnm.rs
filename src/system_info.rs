use sys_info::os_type;

pub enum HostPlatform {
    Windows,
    Darwin,
    Linux,
}

impl std::fmt::Display for HostPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Windows => "windows",
            Self::Linux => "linux",
            Self::Darwin => "darwin",
        };
        f.write_str(value)
    }
}

impl Default for HostPlatform {
    fn default() -> Self {
        let host = os_type().expect("Can't read OS type");
        match host.to_lowercase().as_str() {
            "darwin" => Self::Darwin,
            "linux" => Self::Linux,
            "windows" => panic!("We don't support Windows for now"),
            os => panic!(format!("I don't know this OS: {}", os)),
        }
    }
}

pub enum HostArch {
    X86,
    X64,
}

impl std::fmt::Display for HostArch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::X64 => "x64",
            Self::X86 => "x86",
        })
    }
}

impl Default for HostArch {
    fn default() -> Self {
        Self::X64
    }
}
