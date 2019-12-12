use super::shell::{Bash, Shell, Zsh};

#[derive(Debug)]
struct ProcessInfo {
    parent_pid: Option<u32>,
    command: String,
}

pub fn infer_shell() -> Option<Box<dyn Shell>> {
    let mut pid = Some(std::process::id());
    let mut visited = 0;

    while pid != None && visited < 10 {
        let process_info = get_process_info(pid.unwrap());

        match process_info.command.as_str().trim_start_matches('-') {
            "sh" | "bash" => return Some(Box::from(Bash)),
            "zsh" => return Some(Box::from(Zsh)),
            _ => (),
        };

        pid = process_info.parent_pid;

        visited = visited + 1;
    }

    None
}

fn get_process_info(pid: u32) -> ProcessInfo {
    use std::io::{BufRead, BufReader};
    use std::process::Command;

    let buffer = Command::new("ps")
        .arg("-o")
        .arg("ppid,comm")
        .arg(pid.to_string())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Can't infer shell!")
        .stdout
        .expect("Can't read stdout from `ps`");

    let mut lines = BufReader::new(buffer).lines();

    // skip header line
    lines
        .next()
        .expect("ps doesn't work as expected! have you changed it? 1")
        .expect("ps doesn't work as expected! have you changed it? 2");

    let line = lines
        .next()
        .expect("ps doesn't work as expected! have you changed it? 3")
        .expect("ps doesn't work as expected! have you changed it? 4");

    let mut parts = line.split(' ');
    let ppid = parts
        .next()
        .expect("ps doesn't work as expected! have you changed it? 5");
    let command = parts
        .next()
        .expect("ps doesn't work as expected! have you changed it? 6");

    ProcessInfo {
        parent_pid: u32::from_str_radix(ppid, 10).ok(),
        command: command.into(),
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::process::{Command, Stdio};

    #[test]
    fn test_get_process_info() {
        let subprocess = Command::new("bash")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Can't execute command");
        let process_info = get_process_info(subprocess.id());
        assert_eq!(process_info.parent_pid, Some(std::process::id()));
    }
}
