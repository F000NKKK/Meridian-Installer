//! Linux self-update handoff before GTK registers the application's unique id.

use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const PARENT_VARIABLE: &str = "MERIDIAN_INSTALLER_RESTART_PARENT";

pub(crate) fn start(program: &Path, parent: u32) -> io::Result<()> {
    let mut child = substrate_process::command(program)
        .env(PARENT_VARIABLE, parent.to_string())
        .spawn()?;
    std::thread::spawn(move || {
        // Spawning is the launcher's contract. This detached waiter only reaps the child;
        // after the parent exits it has no caller to receive a wait failure.
        let _status = child.wait();
    });
    Ok(())
}

pub(crate) fn wait_for_parent() -> io::Result<()> {
    let Some(value) = std::env::var_os(PARENT_VARIABLE) else {
        return Ok(());
    };
    let parent = value
        .to_str()
        .and_then(|text| text.parse::<u32>().ok())
        .filter(|pid| *pid != 0 && *pid != std::process::id())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid restart parent"))?;
    wait_for(parent, Duration::from_secs(30))
}

fn wait_for(parent: u32, timeout: Duration) -> io::Result<()> {
    let process = PathBuf::from("/proc").join(parent.to_string());
    let deadline = Instant::now() + timeout;
    while process.exists() {
        if Instant::now() >= deadline {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "the previous installer did not exit",
            ));
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_running_parent_cannot_register_a_second_instance() {
        let error = wait_for(std::process::id(), Duration::from_millis(1)).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::TimedOut);
    }

    #[test]
    fn an_exited_parent_allows_the_updated_process_to_continue() {
        let mut child = std::process::Command::new("sh")
            .args(["-c", "sleep 0.1"])
            .spawn()
            .unwrap();
        let pid = child.id();
        let waiter = std::thread::spawn(move || child.wait().unwrap());
        wait_for(pid, Duration::from_secs(5)).unwrap();
        assert!(waiter.join().unwrap().success());
    }
}
