use std::process::exit;

use gitrelease::get_current_git_tag;
use gitrelease::new_tag;
use gitrelease::push_new_tag;
use gitrelease::show_version;
use gitrelease::Version;

// git describe --tags
// git describe --exact-match --tags $(git log -n1 --pretty='%h')
// git tag --points-at HEAD
// git describe --tags --abbrev=0

// git tag -a 0.1 -m "Initial public release" master
// git push --tags

fn main() {
    println!("{}", show_version());

    let git_tag = get_current_git_tag();

    println!("\nCurrent version: {:?}", git_tag);

    match git_tag {
        Ok(tag) => {
            let tag_parts: Vec<&str> = tag.trim().split('v').collect();
            if tag_parts.len() == 2 {
                let version_parts: Vec<&str> = tag_parts[1].split('.').collect();
                if version_parts.len() == 3 {
                    let major = version_parts[0].parse().unwrap();
                    let minor = version_parts[1].parse().unwrap();
                    let release = version_parts[2].parse().unwrap();
                    let mut version = Version::new(major, minor, release);

                    version.increment_release();

                    println!("New version: {}", version.format());

                    // Commit new tag
                    let new_tag = new_tag(&version.format());
                    match new_tag {
                        Ok(_) => {
                            // Push new tag
                            let push_new_tag = push_new_tag();
                            match push_new_tag {
                                Ok(_) => {
                                    println!("Committed and Pushed new version.");
                                    exit(0);
                                }
                                Err(error) => {
                                    eprintln!("Error: {}", error);
                                    exit(1);
                                }
                            }
                        }
                        Err(error) => {
                            eprintln!("Error: {}", error);
                            exit(1);
                            // You can handle the error here, such as exiting the program or taking other actions.
                        }
                    }
                } else {
                    println!("Invalid tag format.");
                }
            } else {
                eprintln!("Invalid tag format.");
                exit(1);
            }
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            exit(1);
            // You can handle the error here, such as exiting the program or taking other actions.
        }
    }
}
