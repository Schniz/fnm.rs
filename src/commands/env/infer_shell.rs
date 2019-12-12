use super::shell::{Bash, Shell, Zsh};

#[derive(Debug)]
struct ProcessInfo {
    parent_pid: Option<u32>,
    command: String,
}

const MAX_ITERATIONS: u8 = 10;

pub fn infer_shell() -> Option<Box<dyn Shell>> {
    let mut pid = Some(std::process::id());
    let mut visited = 0;

    while pid != None && visited < MAX_ITERATIONS {
        let process_info = get_process_info(pid.unwrap()).ok()?;

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

fn get_process_info(pid: u32) -> std::io::Result<ProcessInfo> {
    use std::io::{BufRead, BufReader};
    use std::process::Command;

    let buffer = Command::new("ps")
        .arg("-o")
        .arg("ppid,comm")
        .arg(pid.to_string())
        .stdout(std::process::Stdio::piped())
        .spawn()?
        .stdout
        .expect("Can't read stdout from `ps`");

    let mut lines = BufReader::new(buffer).lines();

    // skip header line
    let first_line = lines
        .next()
        .expect("Can't read the header (1st) line from `ps`")?;

    dbg!(&first_line);

    let line = lines
        .next()
        .expect("Can't read the data (2nd) line from `ps`")?;

    let mut parts = line.trim().split(' ');
    let ppid = parts
        .next()
        .expect("Can't read the ppid from ps, should be the first item in the table");
    let command = parts
        .next()
        .expect("Can't read the command from ps, should be the first item in the table");

    Ok(ProcessInfo {
        parent_pid: u32::from_str_radix(ppid, 10).ok(),
        command: command.into(),
    })
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
        let parent_pid = process_info.ok().and_then(|x| x.parent_pid);
        assert_eq!(parent_pid, Some(std::process::id()));
    }
}
