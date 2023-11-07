use std::process::Command;
use std::str;

struct Version {
    major: u32,
    minor: u32,
    release: u32,
}

impl Version {
    fn new(major: u32, minor: u32, release: u32) -> Self {
        Self {
            major,
            minor,
            release,
        }
    }

    fn increment_release(&self) -> Self {
        if self.release == 99 {
            Self {
                major: 0,
                minor: self.increment_minor().minor,
                release: 0,
            }
        } else {
            Self {
                major: self.major,
                minor: self.minor,
                release: self.release + 1,
            }
        }
    }

    fn increment_minor(&self) -> Self {
        if self.minor == 99 {
            Self {
                major: self.increment_major().major,
                minor: 0,
                release: 0,
            }
        } else {
            Self {
                major: self.major,
                minor: self.minor + 1,
                release: 0,
            }
        }
    }

    fn increment_major(&self) -> Self {
        Self {
            major: self.major + 1,
            minor: 0,
            release: 0,
        }
    }

    fn format(&self) -> String {
        format!("v{}.{}.{}", self.major, self.minor, self.release)
    }
}

fn get_current_git_tag() -> Result<String, String> {
    let output = Command::new("git")
        .arg("describe")
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

fn show_version() -> String {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    format!("{} via 🦀 v{}/2023", NAME, VERSION)
}

// git describe --tags
// git describe --exact-match --tags $(git log -n1 --pretty='%h')
// git tag --points-at HEAD
// git describe --tags --abbrev=0

// git tag -a 0.1 -m "Initial public release" master
// git push --tags

fn main() {
    println!("{}", show_version());

    let git_tag = get_current_git_tag();

    match git_tag {
        Ok(tag) => {
            let tag_parts: Vec<&str> = tag.trim().split('v').collect();
            if tag_parts.len() == 2 {
                let version_parts: Vec<&str> = tag_parts[0].split('.').collect();
                if version_parts.len() == 3 {
                    let major = version_parts[0].parse().unwrap();
                    let minor = version_parts[1].parse().unwrap();
                    let release = version_parts[2].parse().unwrap();
                    let version = Version::new(major, minor, release);

                    version.increment_release();

                    println!("New version: {}", version.format());
                } else {
                    println!("Invalid tag format.");
                }
            } else {
                println!("Invalid tag format.");
            }
        }
        Err(error) => {
            println!("Error: {}", error);
            // You can handle the error here, such as exiting the program or taking other actions.
        }
    }
}
