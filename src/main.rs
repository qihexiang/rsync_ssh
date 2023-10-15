use clap::Parser;
use serde::Deserialize;
use std::{process::Command, fs::read_to_string};
use directories::BaseDirs;

#[derive(Parser, Debug)]
struct Args {
    config_name: String,
}

#[derive(Deserialize)]
struct Config {
    username: String,
    hostname: String,
    port: Option<u16>,
    remote_path: String,
    local_path: String,
    #[serde(default)]
    excludes: Vec<String>
}

impl Config {
    fn ssh_command(&self) -> String {
        format!("ssh -p {}", self.port.unwrap_or(22))
    }

    fn remote_arg(&self) -> String {
        format!("{}@{}:{}", self.username, self.hostname, self.remote_path)
    }

    fn exclude_args(&self) -> Vec<String> {
        self.excludes.iter().map(|value| format!("--exclude={}", value)).collect::<Vec<_>>()
    }
}

fn main() {
    let Args { config_name } = Args::parse();
    let basedirs = BaseDirs::new().unwrap();
    let target = basedirs.home_dir();
    let target = target.join(".rsync_ssh").join(format!("{}.yml", config_name));
    let config = read_to_string(target).unwrap();
    let config: Config = serde_yaml::from_str(&config).unwrap();
    let mut child = Command::new("rsync")
        .arg("-avz")
        .arg("-e")
        .arg(config.ssh_command())
        .arg(config.remote_arg())
        .arg(&config.local_path)
        .arg("--progress")
        .args(config.exclude_args())
        .spawn()
        .unwrap();
    child.wait().unwrap();
}
