use crate::{is_linux, InfoTrait};
use std::path::Path;
use std::process::Command;
use std::error::Error;

pub struct PackageManager {
	pub name: String,
	pub packages: usize,
}

impl PackageManager {
	pub fn new(name: &str, packages: usize) -> Self {
		PackageManager {
			name: String::from(name),
			packages: packages,
		}
	}
}

pub struct PackageManagers(pub Vec<PackageManager>);

impl InfoTrait for PackageManagers {
    fn get() -> Result<Self, Box<dyn Error>> {
		let _ = is_linux()?;

		let mut to_return = Vec::new();

		let has_bin = |package_manager: &str| -> bool {
			Path::new("/usr/bin/").join(package_manager).exists()
		};
		let mut add = |package_manager: &str, command: &str| {
			to_return.push(PackageManager::new(package_manager, {
				let output = Command::new("sh")
					.arg("-c")
					.arg(&format!(r#"{}"#, command))
					.output().unwrap();

				let stdout_string = String::from_utf8(output.stdout).unwrap();
				let stdout_lines: Vec<&str> = stdout_string.split("\n").collect();
				stdout_lines.len() - 1

			}));
		};

		if has_bin("kiss") {
			add("kiss", "kiss l");
		}
		if has_bin("pacman") {
			add("pacman", "pacman -Qq --color never");
		}
		if has_bin("dpkg") {
			add("dpkg", "dpkg-query -f '.\n' -W");
		}
		if has_bin("rpm") {
			add("rpm", "rpm -qa");
		}
		if has_bin("xbps-query") {
			add("xbps-query", "xbps-query -l");
		}
		if has_bin("apk") {
			add("apk", "apk info");
		}
		if has_bin("opkg") {
			add("opkg", "opkg list-installed");
		}
		if has_bin("pacman-g2") {
			add("pacman-g2", "pacman-g2 -Q");
		}
		if has_bin("lvu") {
			add("lvu", "lvu installed");
		}
		if has_bin("tce-status") {
			add("tce-status", "tce-status -i");
		}
		if has_bin("pkg-info") {
			add("pkg-info", "pkg_info");
		}
		if has_bin("tazpkg") {
			add("tazpkg", "tazpkg list");
		}
		if has_bin("sorcery") {
			add("sorcery", "gaze installed");
		}
		if has_bin("alps") {
			add("alps", "alps showinstalled");
		}
		if has_bin("butch") {
			add("butch", "butch list");
		}
		if has_bin("mine") {
			add("mine", "mine -q");
		}
		if has_bin("dnf") {
			add("dnf", "dnf list installed");
		}
		if has_bin("apt") {
			add("apt", "apt list --installed");
		}
		if has_bin("flatpak") {
			add("flatpak", "flatpak list");
		}
		if has_bin("cargo") {
			add("cargo", "cargo install --list");
		}
		if has_bin("pip") {
			add("pip", "pip list");
		}
		if has_bin("snap") {
			let daemon_running = {
				let try_output = Command::new("sh")
					.arg("-c")
					.arg(r#"ps aux | grep -qFm 1 snapd"#)
					.output();
				match try_output {
					Ok(output) => output.status.success(),
					Err(_) => false,
				}
			};
			if daemon_running {
				add("snap", "snap list");
			}
		}

		Ok(PackageManagers(to_return))
	}
}
