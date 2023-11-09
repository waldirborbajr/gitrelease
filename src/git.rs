use std::process::Command;
use std::str;

pub struct Version {
    major: u32,
    minor: u32,
    release: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, release: u32) -> Version {
        Version {
            major,
            minor,
            release,
        }
    }

    pub fn increment_release(&mut self) {
        if self.release == 99 {
            self.major = 0;
            self.increment_minor();
            self.release = 0;
        } else {
            // self.major = 0;
            // self.minor = 0;
            self.release += 1;
        }
    }

    pub fn increment_minor(&mut self) {
        if self.minor == 99 {
            self.increment_major();
            self.minor = 0;
            self.release = 0;
        } else {
            // self.major = 0;
            self.minor += 1;
            // self.release = 0;
        }
    }

    pub fn increment_major(&mut self) {
        self.major += 1;
        // self.minor = 0;
        // self.release = 0;
    }

    pub fn format(&self) -> String {
        format!("v{}.{}.{}", self.major, self.minor, self.release)
    }
}

pub fn get_current_git_tag() -> Result<String, String> {
    let output = Command::new("git")
        .arg("describe")
        .arg("--abbrev=0")
        .arg("--tags")
        .output()
        .expect("Failed to execute command");

    let git_describe = str::from_utf8(&output.stderr).expect("Failed to convert to string");
    if git_describe.trim() == "fatal: No names found, cannot describe anything." {
        return Ok("v0.1.0".to_string());
    }

    if output.status.success() {
        let git_describe = str::from_utf8(&output.stdout).expect("Failed to convert to string");
        Ok(git_describe.trim().to_string())
    } else {
        Err("Git describe command failed".to_string())
    }
}

pub fn new_tag(new_tag: &str) -> Result<String, String> {
    let output = Command::new("git")
        .arg("tag")
        .arg("-a")
        .arg(new_tag)
        .arg("-m")
        .arg("new release")
        .arg("main")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let git_describe = str::from_utf8(&output.stdout).expect("Failed to convert to string");
        Ok(git_describe.trim().to_string())
    } else {
        Err("Git describe command failed".to_string())
    }
}

pub fn push_new_tag() -> Result<String, String> {
    let output = Command::new("git")
        .arg("push")
        .arg("--tags")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let git_describe = str::from_utf8(&output.stdout).expect("Failed to convert to string");
        Ok(git_describe.trim().to_string())
    } else {
        Err("Git describe command failed".to_string())
    }
}
