use std::process::Command;
use std::str;

fn get_current_git_tag() -> Result<String, String> {
    let current_release = Command::new("git")
        .arg("describe")
        .arg("--tags")
        .output()
        .expect("Failed to execute command");

    if current_release.status.success() {
        let stdout = str::from_utf8(&current_release.stdout).expect("Failed to convert to string");
        let tag = stdout.trim().to_string();
        Ok(tag)
    } else {
        let stderr =
            str::from_utf8(&current_release.stderr).expect("Failed to convert stderr to string");
        let error_message = stderr.trim().to_string();
        Err(error_message)
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

    match get_current_git_tag() {
        Ok(tag) => {
            println!("Current Git tag: {}", tag);
            // Assuming the tag has a format like "v1.0.0", we can parse and increment the release number:
            let parts: Vec<&str> = tag.split('.').collect();
            if parts.len() == 3 {
                let major: i32 = parts[0].trim_start_matches('v').parse().unwrap();
                let minor: i32 = parts[1].parse().unwrap();
                let release: i32 = parts[2].parse().unwrap();
                // Increment the release number by 1
                let new_release = release + 1;
                let new_tag = format!("v{}.{}.{}", major, minor, new_release);
                println!("New Git tag: {}", new_tag);
            } else {
                println!("Tag format is not as expected.");
            }
        }
        Err(error) => {
            println!("Error: {}", error);
            // You can handle the error here, such as exiting the program or taking other actions.
        }
    }
}
